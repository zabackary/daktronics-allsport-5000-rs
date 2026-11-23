# Testing without a console: the console simulator

`simulate_wrestling.rs` pretends to be an All Sport 5000 console. It plays a
fake wrestling dual meet and writes the console's real-time data (RTD) serial
frames to a serial port. Point it at one end of a connected port pair, and you
can develop and test anything that reads RTD without real hardware. That
includes this crate's decoders, overlays, and graphics systems.

## Setting up a serial port pair

You need two serial ports wired to each other. One is for the simulator, the
other for the program under test.

- **Windows**: use a null-modem cable between two ports, or create a virtual
  pair with [com0com](https://com0com.sourceforge.net/) (e.g.
  `COM10 <-> COM11`).
- **Linux/macOS**: create a virtual pair with `socat`:

  ```sh
  socat -d -d pty,raw,echo=0 pty,raw,echo=0
  ```

  It prints two device paths (e.g. `/dev/pts/3` and `/dev/pts/4`). Keep it
  running while you test.

## Running

Start the simulator on one end of the pair:

```sh
cargo run --example simulate_wrestling -- COM10        # or /dev/pts/3
```

Read the other end, for example with the wrestling reader example:

```sh
cargo run --example dump_wrestling_as_json --features serde -- COM11   # or /dev/pts/4
```

Any other RTD consumer works too. Configure it for **19200 baud, 8 data bits,
no parity, 1 stop bit**, the same settings the console uses. Press **Ctrl+C**
to stop.

## What it sends

- A full buffer dump on start-up (like pressing STOP on a real console), then a
  frame every 100 ms.
- Each frame carries the clock fields. When anything else changes, it carries
  the whole buffer.
- A dual meet between two teams:
  - Each bout has three 2:00 periods.
  - About every 30 s there is a score: a takedown (3), escape (1), reversal (2)
    or near fall (2-4).
  - Riding time is shown as the advantage time.
  - A 15-point lead ends the bout early (technical fall). A tie after the third
    period is settled by a sudden-victory takedown.
  - After each bout, the winner's team gets 3, 4 or 5 team points, and the next
    match number starts.
- Each score is printed in the terminal, so you can compare it with what the
  receiving side shows.

The team names, period length and scoring rate are constants at the top of
`simulate_wrestling.rs`.

The example's tests check the frame encoding against a known packet from
`src/packet.rs`. They also send a rendered scoreboard through the crate's own
`Packet` parser:

```sh
cargo test --example simulate_wrestling
```

## Simulating another sport

The frame format is the same for every sport. Only the **field layout** and
the **game logic** change. To add, for example, basketball:

1. **Copy** `simulate_wrestling.rs` to `simulate_basketball.rs`. Keep
   `encode_frame`, `Board`, `Rng`, `main` and the `encodes_known_packet` test.

2. **Find the field layout** in `src/sports/<sport>.rs`. The `Simple<Sport>`
   block near the bottom lists the commonly used fields:

   ```text
   (home_team_score, i32, 16, "Home Team Score", 108, 4, R, "")
    accessor         type id  description        item len justify
   ```

   - **item** is the 1-based position in the console buffer. Pass it straight
     to `Board::put` / `Board::flag`.
   - **len** is the field width.
   - **justify** is `L` or `R`. `R` means `right_justify = true`.
   - **bool** fields are one character, with a space meaning off. The "on"
     character is given in the description, e.g. `'s'` for clock stopped.

3. **Set `BUFFER_LEN`** to the last rendered field's item + len - 1. Check
   that `CLOCK_REGION_LEN` (items 1-39) still covers the sport's clock fields.

4. **Replace the game logic** (`Dual`) with the sport's state. Keep its two
   jobs:
   - `tick()` runs every 100 ms and advances the clock and random events.
   - `render()` writes every field into the `Board`.

   The clock fields (items 1-30) share positions in basketball and wrestling,
   so `render_clock` is often reusable. Check it against the sport's file.

5. **Update the round-trip test** to check a few of the new fields. The
   decoded string is 0-indexed, so item N is at `data[N-1..]`.

6. **Register the example** in `Cargo.toml`:

   ```toml
   [[example]]
   name = "simulate_basketball"
   required-features = ["tokio-serial"]
   ```

To read the result back, use the matching reader example, e.g.
`dump_basketball_as_json`.
