//! The struct for viewing Taekwondo data streams
//!
//! This sport was generated semi-automatically and may contain errors.
super::sport_builder!(
    Taekwondo,
    "Taekwondo",
    true,
    (
        main_clock_time,
        &str,
        1,
        "Main Clock Time (mm:ss/ss.t)",
        1,
        5,
        L,
        ""
    ),
    (
        main_clock_time_2,
        &str,
        2,
        "Main Clock Time (mm:ss.t)",
        6,
        8,
        L,
        ""
    ),
    (
        main_blood_injury_tod,
        &str,
        3,
        "Main/Blood/Injury/TOD (mm:ss/ss.t)",
        14,
        5,
        L,
        ""
    ),
    (
        main_blood_injury_tod_2,
        &str,
        4,
        "Main/Blood/Injury/TOD (mm:ss.t)",
        19,
        8,
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
        L,
        ""
    ),
    (
        main_clock_stopped,
        bool,
        6,
        "Main Clock Stopped (' ' or 's')",
        28,
        1,
        L,
        ""
    ),
    (
        main_clock_time_out_horn,
        bool,
        7,
        "Main Clock/Time Out Horn (' ' or 'h')",
        29,
        1,
        L,
        ""
    ),
    (
        main_clock_horn,
        bool,
        8,
        "Main Clock Horn (' ' or 'h')",
        30,
        1,
        L,
        ""
    ),
    (
        time_out_horn,
        bool,
        9,
        "Time Out Horn (' ' or 'h')",
        31,
        1,
        L,
        ""
    ),
    (
        time_out_time,
        &str,
        10,
        "Time Out Time (mm:ss)",
        32,
        8,
        L,
        ""
    ),
    (
        time_of_day,
        &str,
        11,
        "Time of Day (hh:mm:ss)",
        40,
        8,
        L,
        ""
    ),
    (home_team_name, &str, 12, "Home Team Name", 48, 20, L, ""),
    (guest_team_name, &str, 13, "Guest Team Name", 68, 20, L, ""),
    (
        home_team_abbreviation,
        &str,
        14,
        "Home Team Abbreviation",
        88,
        10,
        L,
        "Future",
        deprecate
    ),
    (
        guest_team_abbreviation,
        &str,
        15,
        "Guest Team Abbreviation",
        98,
        10,
        L,
        "Future",
        deprecate
    ),
    (home_team_score, i32, 16, "Home Team Score", 108, 4, R, ""),
    (guest_team_score, i32, 17, "Guest Team Score", 112, 4, R, ""),
    (
        home_time_outs_left_full,
        i32,
        18,
        "Home Time Outs Left - Full",
        116,
        2,
        R,
        ""
    ),
    (
        home_time_outs_left_partial,
        i32,
        19,
        "Home Time Outs Left - Partial",
        118,
        2,
        R,
        ""
    ),
    (
        home_time_outs_left_television,
        i32,
        20,
        "Home Time Outs Left - Television",
        120,
        2,
        R,
        "Future",
        deprecate
    ),
    (
        home_time_outs_left_total,
        i32,
        21,
        "Home Time Outs Left - Total",
        122,
        2,
        R,
        ""
    ),
    (
        guest_time_outs_left_full,
        i32,
        22,
        "Guest Time Outs Left - Full",
        124,
        2,
        R,
        ""
    ),
    (
        guest_time_outs_left_partial,
        i32,
        23,
        "Guest Time Outs Left - Partial",
        126,
        2,
        R,
        ""
    ),
    (
        guest_time_outs_left_television,
        i32,
        24,
        "Guest Time Outs Left - Television",
        128,
        2,
        R,
        "Future",
        deprecate
    ),
    (
        guest_time_outs_left_total,
        i32,
        25,
        "Guest Time Outs Left - Total",
        130,
        2,
        R,
        ""
    ),
    (
        home_time_out_indicator,
        bool,
        26,
        "Home Time Out Indicator (' ' or '<')",
        132,
        1,
        L,
        ""
    ),
    (
        home_time_out_text,
        &str,
        27,
        "Home Time Out Text (‘ ‘ or ‘TIME’)",
        133,
        4,
        L,
        ""
    ),
    (
        guest_time_out_indicator,
        bool,
        28,
        "Guest Time Out Indicator (' ' or '>')",
        137,
        1,
        L,
        ""
    ),
    (
        guest_time_out_text,
        &str,
        29,
        "Guest Time Out Text (‘ ‘ or ‘TIME’)",
        138,
        4,
        L,
        ""
    ),
    (period, i32, 30, "Period", 142, 2, R, ""),
    (
        period_text,
        &str,
        31,
        "Period Text ('1st ', 'OT', 'OT/2')",
        144,
        4,
        L,
        "Future",
        deprecate
    ),
    (
        period_description,
        &str,
        32,
        "Period Description ('End of 1st ')",
        148,
        12,
        L,
        "Future",
        deprecate
    ),
    (
        internal_relay,
        &str,
        33,
        "Internal Relay (' ' or 'z', 's', 'h')",
        160,
        1,
        L,
        ""
    ),
    (
        ad_panel_caption_power,
        bool,
        34,
        "Ad Panel / Caption Power ('c')",
        161,
        1,
        L,
        ""
    ),
    (
        ad_panel_caption_num1,
        bool,
        35,
        "Ad Panel / Caption #1 (' ' or 'c')",
        162,
        1,
        L,
        ""
    ),
    (
        ad_panel_caption_num2,
        bool,
        36,
        "Ad Panel / Caption #2 (' ' or 'c')",
        163,
        1,
        L,
        ""
    ),
    (
        ad_panel_caption_num3,
        bool,
        37,
        "Ad Panel / Caption #3 (' ' or 'c')",
        164,
        1,
        L,
        "Future",
        deprecate
    ),
    (
        ad_panel_caption_num4,
        bool,
        38,
        "Ad Panel / Caption #4 (' ' or 'c')",
        165,
        1,
        L,
        "Future",
        deprecate
    ),
    (weight_class, i32, 40, "Weight Class", 201, 2, L, ""),
    (red_d, &str, 41, "Red 'D'", 203, 1, R, ""),
    (red_g, &str, 42, "Red 'G'", 204, 1, R, ""),
    (red_k, &str, 43, "Red 'K'", 205, 1, R, ""),
    (white_d, &str, 44, "White 'D'", 206, 1, R, ""),
    (white_g, &str, 45, "White 'G'", 207, 1, R, ""),
    (white_k, &str, 46, "White 'K'", 208, 1, R, ""),
    (
        red_check_indicator,
        bool,
        47,
        "Red Check Indicator (' ' or '>')",
        209,
        1,
        L,
        ""
    ),
    (
        red_win_indicator,
        bool,
        48,
        "Red Win Indicator (' ' or '>')",
        210,
        1,
        L,
        ""
    ),
    (
        white_check_indicator,
        bool,
        49,
        "White Check Indicator (' ' or '<')",
        211,
        1,
        L,
        ""
    ),
    (
        white_win_indicator,
        bool,
        50,
        "White Win Indicator (' ' or '<')",
        212,
        1,
        L,
        ""
    )
);
