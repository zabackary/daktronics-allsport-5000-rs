//! The struct for viewing Pitch/Speed data streams
//!
//! This sport was generated semi-automatically and may contain errors.
super::sport_builder!(
    PitchAndSpeed,
    "Pitch and Speed",
    true,
    (miles_per_hour, i32, 1, "Miles Per Hour (MPH)", 1, 3, R, ""),
    (pitch_type, &str, 2, "Pitch Type", 4, 16, L, ""),
    (
        kilometers_per_hour,
        i32,
        3,
        "Kilometers Per Hour (KPH)",
        20,
        3,
        R,
        ""
    ),
    (mph_or_kph, &str, 4, "MPH or KPH", 23, 3, R, ""),
    (mph_indicator, bool, 5, "MPH Indicator", 26, 1, N, ""),
    (kph_indicator, bool, 6, "KPH Indicator", 27, 1, N, ""),
    (
        home_ball_pitch_count,
        i32,
        7,
        "Home Ball Pitch Count",
        28,
        3,
        R,
        ""
    ),
    (
        home_strike_pitch_count,
        i32,
        8,
        "Home Strike Pitch Count",
        31,
        3,
        R,
        ""
    ),
    (
        home_total_pitch_count,
        i32,
        9,
        "Home Total Pitch Count",
        34,
        3,
        R,
        ""
    ),
    (
        home_strike_out_count,
        i32,
        10,
        "Home Strike Out Count",
        37,
        3,
        R,
        ""
    ),
    (
        guest_ball_pitch_count,
        i32,
        11,
        "Guest Ball Pitch Count",
        40,
        3,
        R,
        ""
    ),
    (
        guest_strike_pitch_count,
        i32,
        12,
        "Guest Strike Pitch Count",
        43,
        3,
        R,
        ""
    ),
    (
        guest_total_pitch_count,
        i32,
        13,
        "Guest Total Pitch Count",
        46,
        3,
        R,
        ""
    ),
    (
        guest_strike_out_count,
        i32,
        14,
        "Guest Strike Out Count",
        49,
        3,
        R,
        ""
    )
);
