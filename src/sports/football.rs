//! This sport was generated semi-automatically and may contain errors.
super::sport_builder!(
    Football,
    "Football",
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
        main_clock_time_out_tod,
        &str,
        3,
        "Main Clock/Time Out/TOD (mm:ss/ss.t)",
        14,
        5,
        L,
        ""
    ),
    (
        main_time_out_tod_2,
        &str,
        4,
        "Main Clock/Time Out/TOD (mm:ss.t)",
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
    (home_team_score, &str, 16, "Home Team Score", 108, 4, R, ""),
    (
        guest_team_score,
        &str,
        17,
        "Guest Team Score",
        112,
        4,
        R,
        ""
    ),
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
        "Home Time Out Text (' ' or 'TIME')",
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
        "Guest Time Out Text (' ' or 'TIME')",
        138,
        4,
        L,
        ""
    ),
    (quarter, i32, 30, "Quarter", 142, 2, R, ""),
    (
        quarter_text,
        &str,
        31,
        "Quarter Text ('1st ', 'OT', 'OT/2')",
        144,
        4,
        L,
        "Future",
        deprecate
    ),
    (
        quarter_description,
        &str,
        32,
        "Quarter Description ('End of 1st')",
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
    (
        play_clock_time,
        &str,
        40,
        "Play Clock Time (mm:ss)",
        201,
        8,
        L,
        ""
    ),
    (
        play_clock_horn,
        bool,
        41,
        "Play Clock Horn (' ' or 'h')",
        209,
        1,
        L,
        ""
    ),
    (
        home_possession_indicator,
        bool,
        42,
        "Home Possession Indicator (' ' or '<')",
        210,
        1,
        L,
        ""
    ),
    (
        home_possession_text,
        &str,
        43,
        "Home Possession Text (' ' or 'POSS')",
        211,
        4,
        L,
        ""
    ),
    (
        guest_possession_indicator,
        bool,
        44,
        "Guest Possession Indicator (' ' or '>')",
        215,
        1,
        L,
        ""
    ),
    (
        guest_possession_text,
        &str,
        45,
        "Guest Possession Text (' ' or 'POSS')",
        216,
        4,
        L,
        ""
    ),
    (ball_on, i32, 46, "Ball On", 220, 2, R, ""),
    (
        down,
        &str,
        47,
        "Down ('1st', '2nd', '3rd', '4th')",
        222,
        3,
        L,
        ""
    ),
    (to_go, i32, 48, "To Go", 225, 2, R, ""),
    (
        home_score_period_1,
        i32,
        49,
        "Home Score - Period 1",
        227,
        2,
        R,
        ""
    ),
    (
        home_score_period_2,
        i32,
        50,
        "Home Score - Period 2",
        229,
        2,
        R,
        ""
    ),
    (
        home_score_period_3,
        i32,
        51,
        "Home Score - Period 3",
        231,
        2,
        R,
        ""
    ),
    (
        home_score_period_4,
        i32,
        52,
        "Home Score - Period 4",
        233,
        2,
        R,
        ""
    ),
    (
        home_score_period_5,
        i32,
        53,
        "Home Score - Period 5",
        235,
        2,
        R,
        ""
    ),
    (
        home_score_period_6,
        i32,
        54,
        "Home Score - Period 6",
        237,
        2,
        R,
        ""
    ),
    (
        home_score_period_7,
        i32,
        55,
        "Home Score - Period 7",
        239,
        2,
        R,
        ""
    ),
    (
        home_score_period_8,
        i32,
        56,
        "Home Score - Period 8",
        241,
        2,
        R,
        ""
    ),
    (
        home_score_period_9,
        i32,
        57,
        "Home Score - Period 9",
        243,
        2,
        R,
        ""
    ),
    (
        home_score_current_period,
        i32,
        58,
        "Home Score - Current Period",
        245,
        2,
        R,
        ""
    ),
    (
        guest_score_period_1,
        i32,
        59,
        "Guest Score - Period 1",
        247,
        2,
        R,
        ""
    ),
    (
        guest_score_period_2,
        i32,
        60,
        "Guest Score - Period 2",
        249,
        2,
        R,
        ""
    ),
    (
        guest_score_period_3,
        i32,
        61,
        "Guest Score - Period 3",
        251,
        2,
        R,
        ""
    ),
    (
        guest_score_period_4,
        i32,
        62,
        "Guest Score - Period 4",
        253,
        2,
        R,
        ""
    ),
    (
        guest_score_period_5,
        i32,
        63,
        "Guest Score - Period 5",
        255,
        2,
        R,
        ""
    ),
    (
        guest_score_period_6,
        i32,
        64,
        "Guest Score - Period 6",
        257,
        2,
        R,
        ""
    ),
    (
        guest_score_period_7,
        i32,
        65,
        "Guest Score - Period 7",
        259,
        2,
        R,
        ""
    ),
    (
        guest_score_period_8,
        i32,
        66,
        "Guest Score - Period 8",
        261,
        2,
        R,
        ""
    ),
    (
        guest_score_period_9,
        i32,
        67,
        "Guest Score - Period 9",
        263,
        2,
        R,
        ""
    ),
    (
        guest_score_current_period,
        i32,
        68,
        "Guest Score - Current Period",
        265,
        2,
        R,
        ""
    ),
    (
        home_rushing_yards,
        i32,
        69,
        "Home Rushing Yards",
        267,
        4,
        R,
        ""
    ),
    (
        home_passing_yards,
        i32,
        70,
        "Home Passing Yards",
        271,
        4,
        R,
        ""
    ),
    (home_total_yards, i32, 71, "Home Total Yards", 275, 4, R, ""),
    (
        guest_rushing_yards,
        i32,
        72,
        "Guest Rushing Yards",
        279,
        4,
        R,
        ""
    ),
    (
        guest_passing_yards,
        i32,
        73,
        "Guest Passing Yards",
        283,
        4,
        R,
        ""
    ),
    (
        guest_total_yards,
        i32,
        74,
        "Guest Total Yards",
        287,
        4,
        R,
        ""
    ),
    (home_first_downs, i32, 75, "Home First Downs", 291, 2, R, ""),
    (
        guest_first_downs,
        i32,
        76,
        "Guest First Downs",
        293,
        2,
        R,
        ""
    )
);
