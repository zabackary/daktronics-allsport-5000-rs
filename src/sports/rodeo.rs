//! The struct for viewing Rodeo data streams
//!
//! This sport was generated semi-automatically and may contain errors.
super::sport_builder!(
    Rodeo,
    "Rodeo",
    true,
    (now_up_time, &str, 1, "Now-Up Time (mm:ss.tht)", 1, 9, L, ""),
    (
        now_up_time_score,
        i32,
        3,
        "Now-Up Time/Score (ss.tht/SC)",
        12,
        6,
        L,
        ""
    ),
    (
        now_up_time_score_2,
        i32,
        4,
        "Now-Up Time/Score (mm:ss.tht/SC)",
        18,
        9,
        L,
        ""
    ),
    (
        main_clock_is_zero,
        bool,
        5,
        "Main Clock =0 (' ' or 'z')",
        27,
        1,
        N,
        ""
    ),
    (
        main_clock_stopped,
        bool,
        6,
        "Main Clock Stopped (' ' or 's')",
        28,
        1,
        N,
        ""
    ),
    (
        main_clock_time_out_horn,
        bool,
        7,
        "Main Clock/Time Out Horn (' ' or 'h')",
        29,
        1,
        N,
        ""
    ),
    (
        main_clock_horn,
        bool,
        8,
        "Main Clock Horn (' ' or 'h')",
        30,
        1,
        N,
        ""
    ),
    (now_up_comp_num, i32, 9, "Now-Up Comp #", 31, 4, R, ""),
    (now_up_score, &str, 10, "Now-Up Score (123.4)", 35, 5, L, ""),
    (
        penalty_text,
        &str,
        11,
        "Penalty Text (' ' or 'PENALTY')",
        40,
        7,
        L,
        ""
    ),
    (
        reride_text,
        &str,
        12,
        "Reride Text (' ' or 'RERIDE')",
        47,
        6,
        R,
        ""
    ),
    (
        leader_time_score,
        &str,
        13,
        "Leader Time/Score (mm:ss:tht/SC)",
        53,
        9,
        L,
        ""
    ),
    (leader_comp_num, i32, 14, "Leader Comp #", 62, 4, R, "")
);
