//! The wrestling struct with accessors, along with a simple version
//!
//! This sport was generated semi-automatically and may contain errors.
super::sport_builder!(
    Wrestling,
    "Wrestling",
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
        main_clock_time_out_tod_2,
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
        "Period Description ('End of 1st')",
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
        advantage_time,
        &str,
        40,
        "Advantage Time (mm:ss)",
        201,
        8,
        L,
        ""
    ),
    (
        home_advantage_time,
        &str,
        41,
        "Home Advantage Time (mm:ss)",
        209,
        8,
        L,
        ""
    ),
    (
        guest_advantage_time,
        &str,
        42,
        "Guest Advantage Time (mm:ss)",
        217,
        8,
        L,
        ""
    ),
    (
        home_blood_time,
        &str,
        43,
        "Home Blood Time (mm:ss)",
        225,
        8,
        L,
        ""
    ),
    (
        home_injury_time,
        &str,
        44,
        "Home Injury Time (mm:ss)",
        233,
        8,
        L,
        ""
    ),
    (
        guest_blood_time,
        &str,
        45,
        "Guest Blood Time (mm:ss)",
        241,
        8,
        L,
        ""
    ),
    (
        guest_injury_time,
        &str,
        46,
        "Guest Injury Time (mm:ss)",
        249,
        8,
        L,
        ""
    ),
    (
        home_advantage_indicator,
        bool,
        47,
        "Home Advantage Indicator (' ' or '<')",
        257,
        1,
        L,
        ""
    ),
    (
        home_advantage_arrow,
        bool,
        48,
        "Home Advantage Arrow (' ' or '<')",
        258,
        1,
        L,
        "Future",
        deprecate
    ),
    (
        home_advantage_text,
        &str,
        49,
        "Home Advantage Text ('ADVANTAGE')",
        259,
        9,
        L,
        ""
    ),
    (
        guest_advantage_indicator,
        bool,
        50,
        "Guest Advantage Indicator (' ' or '>')",
        268,
        1,
        L,
        ""
    ),
    (
        guest_advantage_arrow,
        bool,
        51,
        "Guest Advantage Arrow (' ' or '>')",
        269,
        1,
        L,
        "Future",
        deprecate
    ),
    (
        guest_advantage_text,
        &str,
        52,
        "Guest Advantage Text ('ADVANTAGE')",
        270,
        9,
        L,
        ""
    ),
    (home_match_score, i32, 53, "Home Match Score", 279, 2, R, ""),
    (
        guest_match_score,
        i32,
        54,
        "Guest Match Score",
        281,
        2,
        R,
        ""
    ),
    (match_number, i32, 55, "Match Number", 283, 3, R, ""),
    (
        team_score_advan_time,
        &str,
        56,
        "Team Score/Advan Time (hh gg/mm:ss)",
        286,
        8,
        L,
        ""
    ),
    (
        home_bonus_indicator,
        bool,
        57,
        "Home Bonus Indicator (' ' or '<')",
        294,
        1,
        L,
        ""
    ),
    (
        guest_bonus_indicator,
        bool,
        58,
        "Guest Bonus Indicator (' ' or '>')",
        295,
        1,
        L,
        ""
    ),
    (
        class_1_home_points,
        i32,
        59,
        "Class 1 - Home Points",
        296,
        1,
        L,
        ""
    ),
    (
        class_1_guest_points,
        i32,
        60,
        "Class 1 - Guest Points",
        297,
        1,
        L,
        ""
    ),
    (
        class_1_weight_class,
        i32,
        61,
        "Class 1 - Weight Class",
        298,
        3,
        R,
        ""
    ),
    (
        class_2_home_points,
        i32,
        62,
        "Class 2 - Home Points",
        301,
        1,
        L,
        ""
    ),
    (
        class_2_guest_points,
        i32,
        63,
        "Class 2 - Guest Points",
        302,
        1,
        L,
        ""
    ),
    (
        class_2_weight_class,
        i32,
        64,
        "Class 2 - Weight Class",
        303,
        3,
        R,
        ""
    ),
    (
        class_3_home_points,
        i32,
        65,
        "Class 3 - Home Points",
        306,
        1,
        L,
        ""
    ),
    (
        class_3_guest_points,
        i32,
        66,
        "Class 3 - Guest Points",
        307,
        1,
        L,
        ""
    ),
    (
        class_3_weight_class,
        i32,
        67,
        "Class 3 - Weight Class",
        308,
        3,
        R,
        ""
    ),
    (
        class_4_home_points,
        i32,
        68,
        "Class 4 - Home Points",
        311,
        1,
        L,
        ""
    ),
    (
        class_4_guest_points,
        i32,
        69,
        "Class 4 - Guest Points",
        312,
        1,
        L,
        ""
    ),
    (
        class_4_weight_class,
        i32,
        70,
        "Class 4 - Weight Class",
        313,
        3,
        R,
        ""
    ),
    (
        class_5_home_points,
        i32,
        71,
        "Class 5 - Home Points",
        316,
        1,
        L,
        ""
    ),
    (
        class_5_guest_points,
        i32,
        72,
        "Class 5 - Guest Points",
        317,
        1,
        L,
        ""
    ),
    (
        class_5_weight_class,
        i32,
        73,
        "Class 5 - Weight Class",
        318,
        3,
        R,
        ""
    ),
    (
        class_6_home_points,
        i32,
        74,
        "Class 6 - Home Points",
        321,
        1,
        L,
        ""
    ),
    (
        class_6_guest_points,
        i32,
        75,
        "Class 6 - Guest Points",
        322,
        1,
        L,
        ""
    ),
    (
        class_6_weight_class,
        i32,
        76,
        "Class 6 - Weight Class",
        323,
        3,
        R,
        ""
    ),
    (
        class_7_home_points,
        i32,
        77,
        "Class 7 - Home Points",
        326,
        1,
        L,
        ""
    ),
    (
        class_7_guest_points,
        i32,
        78,
        "Class 7 - Guest Points",
        327,
        1,
        L,
        ""
    ),
    (
        class_7_weight_class,
        i32,
        79,
        "Class 7 - Weight Class",
        328,
        3,
        R,
        ""
    ),
    (
        class_8_home_points,
        i32,
        80,
        "Class 8 - Home Points",
        331,
        1,
        L,
        ""
    ),
    (
        class_8_guest_points,
        i32,
        81,
        "Class 8 - Guest Points",
        332,
        1,
        L,
        ""
    ),
    (
        class_8_weight_class,
        i32,
        82,
        "Class 8 - Weight Class",
        333,
        3,
        R,
        ""
    ),
    (
        class_9_home_points,
        i32,
        83,
        "Class 9 - Home Points",
        336,
        1,
        L,
        ""
    ),
    (
        class_9_guest_points,
        i32,
        84,
        "Class 9 - Guest Points",
        337,
        1,
        L,
        ""
    ),
    (
        class_9_weight_class,
        i32,
        85,
        "Class 9 - Weight Class",
        338,
        3,
        R,
        ""
    ),
    (
        class_10_home_points,
        i32,
        86,
        "Class 10 - Home Points",
        341,
        1,
        L,
        ""
    ),
    (
        class_10_guest_points,
        i32,
        87,
        "Class 10 - Guest Points",
        342,
        1,
        L,
        ""
    ),
    (
        class_10_weight_class,
        i32,
        88,
        "Class 10 - Weight Class",
        343,
        3,
        R,
        ""
    ),
    (
        class_11_home_points,
        i32,
        89,
        "Class 11 - Home Points",
        346,
        1,
        L,
        ""
    ),
    (
        class_11_guest_points,
        i32,
        90,
        "Class 11 - Guest Points",
        347,
        1,
        L,
        ""
    ),
    (
        class_11_weight_class,
        i32,
        91,
        "Class 11 - Weight Class",
        348,
        3,
        R,
        ""
    ),
    (
        class_12_home_points,
        i32,
        92,
        "Class 12 - Home Points",
        351,
        1,
        L,
        ""
    ),
    (
        class_12_guest_points,
        i32,
        93,
        "Class 12 - Guest Points",
        352,
        1,
        L,
        ""
    ),
    (
        class_12_weight_class,
        i32,
        94,
        "Class 12 - Weight Class",
        353,
        3,
        R,
        ""
    )
);

super::sport_builder!(
    SimpleWrestling,
    "Simple Wrestling",
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
        main_clock_time_out_tod_2,
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
        guest_time_out_indicator,
        bool,
        28,
        "Guest Time Out Indicator (' ' or '>')",
        137,
        1,
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
        "Period Description ('End of 1st')",
        148,
        12,
        L,
        "Future",
        deprecate
    ),
    (
        advantage_time,
        &str,
        40,
        "Advantage Time (mm:ss)",
        201,
        8,
        L,
        ""
    ),
    (
        home_advantage_time,
        &str,
        41,
        "Home Advantage Time (mm:ss)",
        209,
        8,
        L,
        ""
    ),
    (
        guest_advantage_time,
        &str,
        42,
        "Guest Advantage Time (mm:ss)",
        217,
        8,
        L,
        ""
    ),
    (
        home_blood_time,
        &str,
        43,
        "Home Blood Time (mm:ss)",
        225,
        8,
        L,
        ""
    ),
    (
        home_injury_time,
        &str,
        44,
        "Home Injury Time (mm:ss)",
        233,
        8,
        L,
        ""
    ),
    (
        guest_blood_time,
        &str,
        45,
        "Guest Blood Time (mm:ss)",
        241,
        8,
        L,
        ""
    ),
    (
        guest_injury_time,
        &str,
        46,
        "Guest Injury Time (mm:ss)",
        249,
        8,
        L,
        ""
    ),
    (
        home_advantage_indicator,
        bool,
        47,
        "Home Advantage Indicator (' ' or '<')",
        257,
        1,
        L,
        ""
    ),
    (
        guest_advantage_indicator,
        bool,
        50,
        "Guest Advantage Indicator (' ' or '>')",
        268,
        1,
        L,
        ""
    ),
    (home_match_score, i32, 53, "Home Match Score", 279, 2, R, ""),
    (
        guest_match_score,
        i32,
        54,
        "Guest Match Score",
        281,
        2,
        R,
        ""
    ),
    (match_number, i32, 55, "Match Number", 283, 3, R, "")
);
