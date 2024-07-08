//! The struct for viewing Strike Out Count data streams
//!
//! This sport was generated semi-automatically and may contain errors.
super::sport_builder!(
    StrikeOutCount,
    "Strike Out Count",
    true,
    (game_strikeouts, i32, 2, "Game Strikeouts", 201, 3, R, ""),
    (
        season_strikeouts,
        i32,
        3,
        "Season Strikeouts",
        204,
        4,
        R,
        ""
    )
);
