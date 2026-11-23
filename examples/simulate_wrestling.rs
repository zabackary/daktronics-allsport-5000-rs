//! Simulates an All Sport 5000 console sending Wrestling real-time data (RTD).
//!
//! The program plays a fake dual meet (bout after bout, three 2:00 periods
//! each) and writes the console's RTD serial frames to a serial port, 10 times
//! a second. Point it at one end of a null-modem cable or a virtual serial
//! port pair (com0com on Windows, `socat` on Linux/macOS) and read the other
//! end with `dump_wrestling_as_json` or any other RTD consumer. This lets you
//! test without a console. See `examples/SIMULATOR.md`.
//!
//! Usage: `cargo run --example simulate_wrestling -- COM10`
//!
//! Field positions come from the Simple Wrestling layout in
//! `src/sports/wrestling.rs`.
use std::{
    env,
    io::{self, Write},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB1";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM2";

/// Time between frames; the clock counts down in tenths of a second.
const TICK: Duration = Duration::from_millis(100);
/// Size of the data buffer: the last Simple Wrestling field (match number,
/// item 283, length 3) ends at byte 285.
const BUFFER_LEN: usize = 285;
/// Items 1..=39 hold the clocks and their flags; they're resent every tick.
const CLOCK_REGION_LEN: usize = 39;
/// Two minutes, in tenths of a second.
const PERIOD_TENTHS: u32 = 1200;
const PERIODS_PER_BOUT: u32 = 3;
const INTERMISSION_TICKS: u32 = 30;
const BETWEEN_BOUTS_TICKS: u32 = 50;
/// A score happens with probability 1/SCORE_ODDS each running tick: about one
/// every 30 s, so a 6-minute bout ends around 10-20 points like a real one.
const SCORE_ODDS: u64 = 300;
const TECH_FALL_MARGIN: u32 = 15;
const HOME_TEAM: &str = "CENTRAL";
const GUEST_TEAM: &str = "WESTVIEW";

/// Encodes one RTD frame exactly as the console sends it.
///
/// Layout: `SYN "00000000" SOH "004210" <4-digit start index> STX <data>
/// EOT <2-hex-digit checksum> ETB`. The checksum is the wrapping byte sum of
/// everything from the first `0` up to and including the EOT.
fn encode_frame(start_index: usize, data: &[u8]) -> Vec<u8> {
    let mut body = Vec::with_capacity(data.len() + 32);
    body.extend_from_slice(b"00000000\x01004210");
    body.extend_from_slice(format!("{start_index:04}").as_bytes());
    body.push(0x02);
    body.extend_from_slice(data);
    body.push(0x04);
    let checksum = body.iter().fold(0u8, |sum, byte| sum.wrapping_add(*byte));

    let mut frame = Vec::with_capacity(body.len() + 4);
    frame.push(0x16);
    frame.extend_from_slice(&body);
    frame.extend_from_slice(format!("{checksum:02X}").as_bytes());
    frame.push(0x17);
    frame
}

/// The console's data buffer, addressed with Daktronics' one-based items.
struct Board {
    buf: Vec<u8>,
}

impl Board {
    fn new() -> Self {
        Self {
            buf: vec![b' '; BUFFER_LEN],
        }
    }

    /// Writes `value` into `len` bytes at `item`, padded and truncated.
    fn put(&mut self, item: usize, len: usize, right_justify: bool, value: &str) {
        let text = if right_justify {
            format!("{value:>len$}")
        } else {
            format!("{value:<len$}")
        };
        self.buf[item - 1..item - 1 + len].copy_from_slice(&text.as_bytes()[..len]);
    }

    /// Writes a one-character flag: `on_char` when set, a space otherwise.
    fn flag(&mut self, item: usize, set: bool, on_char: char) {
        self.buf[item - 1] = if set { on_char as u8 } else { b' ' };
    }
}

/// Small xorshift PRNG so the example needs no extra dependencies.
struct Rng(u64);

impl Rng {
    fn seeded() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0x9E37_79B9, |d| d.as_nanos() as u64);
        Self(nanos | 1)
    }

    fn below(&mut self, n: u64) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 % n
    }

    fn side(&mut self) -> Side {
        if self.below(2) == 0 {
            Side::Home
        } else {
            Side::Guest
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Side {
    Home,
    Guest,
}

impl Side {
    const fn other(self) -> Self {
        match self {
            Side::Home => Side::Guest,
            Side::Guest => Side::Home,
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Side::Home => HOME_TEAM,
            Side::Guest => GUEST_TEAM,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Running,
    Intermission(u32),
    BetweenBouts(u32),
}

/// State of a simulated dual meet.
struct Dual {
    home_team_score: u32,
    guest_team_score: u32,
    match_number: u32,
    period: u32,
    tenths_left: u32,
    phase: Phase,
    horn: bool,
    home_points: u32,
    guest_points: u32,
    /// Riding time in tenths; positive means home has the advantage.
    riding_tenths: i32,
    top: Option<Side>,
}

impl Dual {
    const fn new() -> Self {
        Self {
            home_team_score: 0,
            guest_team_score: 0,
            match_number: 1,
            period: 1,
            tenths_left: PERIOD_TENTHS,
            phase: Phase::Running,
            horn: false,
            home_points: 0,
            guest_points: 0,
            riding_tenths: 0,
            top: None,
        }
    }

    fn tick(&mut self, rng: &mut Rng) {
        self.horn = false;
        match self.phase {
            Phase::Running => self.run_clock(rng),
            Phase::Intermission(0) => self.start_period(rng),
            Phase::BetweenBouts(0) => self.start_bout(),
            Phase::Intermission(n) => self.phase = Phase::Intermission(n - 1),
            Phase::BetweenBouts(n) => self.phase = Phase::BetweenBouts(n - 1),
        }
    }

    fn run_clock(&mut self, rng: &mut Rng) {
        self.tenths_left -= 1;
        match self.top {
            Some(Side::Home) => self.riding_tenths += 1,
            Some(Side::Guest) => self.riding_tenths -= 1,
            None => {}
        }
        if rng.below(SCORE_ODDS) == 0 {
            self.score(rng);
        }
        if self.home_points.abs_diff(self.guest_points) >= TECH_FALL_MARGIN {
            self.finish_bout(rng);
        } else if self.tenths_left == 0 {
            self.end_period(rng);
        }
    }

    /// Picks a plausible scoring move for the current position.
    fn score(&mut self, rng: &mut Rng) {
        let (side, points, name, new_top) = match self.top {
            None => {
                let side = rng.side();
                (side, 3, "takedown", Some(side))
            }
            Some(top) => match rng.below(3) {
                0 => (top, 2 + rng.below(3) as u32, "near fall", Some(top)),
                1 => (top.other(), 1, "escape", None),
                _ => (top.other(), 2, "reversal", Some(top.other())),
            },
        };
        self.top = new_top;
        self.add_points(side, points);
        println!(
            "Match {} P{}: {} {name} +{points} ({}-{})",
            self.match_number,
            self.period,
            side.label(),
            self.home_points,
            self.guest_points
        );
    }

    fn add_points(&mut self, side: Side, points: u32) {
        match side {
            Side::Home => self.home_points += points,
            Side::Guest => self.guest_points += points,
        }
    }

    fn end_period(&mut self, rng: &mut Rng) {
        self.horn = true;
        if self.period < PERIODS_PER_BOUT {
            self.phase = Phase::Intermission(INTERMISSION_TICKS);
        } else {
            self.finish_bout(rng);
        }
    }

    fn start_period(&mut self, rng: &mut Rng) {
        self.period += 1;
        self.tenths_left = PERIOD_TENTHS;
        self.top = Some(rng.side());
        self.phase = Phase::Running;
    }

    /// Ends the bout and awards team points (decision 3, major 4, tech fall 5).
    fn finish_bout(&mut self, rng: &mut Rng) {
        self.horn = true;
        if self.home_points == self.guest_points {
            let side = rng.side();
            self.add_points(side, 3);
            println!(
                "Match {}: {} sudden-victory takedown",
                self.match_number,
                side.label()
            );
        }
        let margin = self.home_points.abs_diff(self.guest_points);
        let team_points = match margin {
            m if m >= TECH_FALL_MARGIN => 5,
            m if m >= 8 => 4,
            _ => 3,
        };
        let winner = if self.home_points > self.guest_points {
            Side::Home
        } else {
            Side::Guest
        };
        match winner {
            Side::Home => self.home_team_score += team_points,
            Side::Guest => self.guest_team_score += team_points,
        }
        println!(
            "Match {} won by {} {}-{}; team score {}-{}",
            self.match_number,
            winner.label(),
            self.home_points,
            self.guest_points,
            self.home_team_score,
            self.guest_team_score
        );
        self.phase = Phase::BetweenBouts(BETWEEN_BOUTS_TICKS);
    }

    fn start_bout(&mut self) {
        *self = Self {
            home_team_score: self.home_team_score,
            guest_team_score: self.guest_team_score,
            match_number: self.match_number % 999 + 1,
            ..Self::new()
        };
    }

    fn render(&self, board: &mut Board) {
        self.render_clock(board);
        board.put(48, 20, false, HOME_TEAM);
        board.put(68, 20, false, GUEST_TEAM);
        board.put(108, 4, true, &self.home_team_score.to_string());
        board.put(112, 4, true, &self.guest_team_score.to_string());
        board.put(142, 2, true, &self.period.to_string());
        let riding = self.riding_tenths.unsigned_abs() / 10;
        let riding_text = format!("{}:{:02}", riding / 60, riding % 60);
        board.put(201, 8, false, &riding_text);
        let home_riding = if self.riding_tenths > 0 {
            riding_text.as_str()
        } else {
            ""
        };
        let guest_riding = if self.riding_tenths < 0 {
            riding_text.as_str()
        } else {
            ""
        };
        board.put(209, 8, false, home_riding);
        board.put(217, 8, false, guest_riding);
        board.flag(257, self.riding_tenths > 0, '<');
        board.flag(268, self.riding_tenths < 0, '>');
        board.put(279, 2, true, &self.home_points.to_string());
        board.put(281, 2, true, &self.guest_points.to_string());
        board.put(283, 3, true, &self.match_number.to_string());
    }

    fn render_clock(&self, board: &mut Board) {
        let t = self.tenths_left;
        let short = if t >= 600 {
            format!("{}:{:02}", t / 600, (t / 10) % 60)
        } else {
            format!("{}.{}", t / 10, t % 10)
        };
        let long = format!("{}:{:02}.{}", t / 600, (t / 10) % 60, t % 10);
        board.put(1, 5, false, &short);
        board.put(6, 8, false, &long);
        board.put(14, 5, false, &short);
        board.put(19, 8, false, &long);
        board.flag(27, t == 0, 'z');
        board.flag(28, self.phase != Phase::Running, 's');
        board.flag(29, self.horn, 'h');
        board.flag(30, self.horn, 'h');
    }
}

fn main() -> io::Result<()> {
    let tty_path = env::args().nth(1).unwrap_or_else(|| DEFAULT_TTY.into());
    let mut port = tokio_serial::new(&tty_path, 19200)
        .parity(tokio_serial::Parity::None)
        .open()
        .map_err(io::Error::from)?;
    println!("Sending Wrestling RTD on {tty_path} (Ctrl+C to stop)");

    let mut rng = Rng::seeded();
    let mut dual = Dual::new();
    let mut board = Board::new();
    dual.render(&mut board);
    // A real console dumps the whole buffer when STOP is pressed; start with that.
    port.write_all(&encode_frame(0, &board.buf))?;

    loop {
        thread::sleep(TICK);
        let before = board.buf.clone();
        dual.tick(&mut rng);
        dual.render(&mut board);
        let frame = if board.buf[CLOCK_REGION_LEN..] == before[CLOCK_REGION_LEN..] {
            encode_frame(0, &board.buf[..CLOCK_REGION_LEN])
        } else {
            encode_frame(0, &board.buf)
        };
        port.write_all(&frame)?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use daktronics_allsport_5000::packet::Packet;

    /// Strips the SYN/ETB framing the serial codec removes before parsing.
    fn parse(frame: &[u8]) -> Packet {
        assert_eq!(frame.first(), Some(&0x16));
        assert_eq!(frame.last(), Some(&0x17));
        Packet::try_from(&frame[1..frame.len() - 1]).expect("frame should parse")
    }

    #[test]
    fn encodes_known_packet() {
        let frame = encode_frame(6, b"16:0916:09   16:0916:09    s   ");
        assert_eq!(
            &frame[1..frame.len() - 1],
            b"00000000\x010042100006\x0216:0916:09   16:0916:09    s   \x044F"
        );
    }

    #[test]
    fn full_board_round_trips() {
        let mut board = Board::new();
        let mut dual = Dual::new();
        dual.home_points = 7;
        dual.match_number = 12;
        dual.render(&mut board);
        let packet = parse(&encode_frame(0, &board.buf));
        assert_eq!(packet.start_index(), 0);
        let data = packet.data().unwrap();
        assert_eq!(&data[0..5], "2:00 ");
        assert_eq!(&data[47..54], "CENTRAL");
        assert_eq!(&data[278..280], " 7");
        assert_eq!(&data[282..285], " 12");
    }
}
