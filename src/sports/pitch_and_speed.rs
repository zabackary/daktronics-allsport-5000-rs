super::sport_builder!(
    PitchAndSpeed,
    "Pitch And Speech",
    false,
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
    (mph_indicator, bool, 5, "MPH Indicator", 27, 1, N, ""),
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
    ) // missing home strike pitch count, etc.
);
