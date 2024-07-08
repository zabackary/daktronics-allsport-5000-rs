//! The struct for viewing simple Date/Time data streams

super::super::sport_builder!(
    DateTimeEventCounter,
    "Date and Time Event Countdown",
    true,
    (days_remaining, i32, 1, "Days Remaining", 1, 3, R, ""),
    (time_remaining, &str, 2, "Time Remaining", 4, 8, R, "")
);
