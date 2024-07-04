//! This sport was generated semi-automatically and may contain errors.
super::sport_builder!(
    WaterPolo,
    "Water Polo",
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
        main_clock_time_out_tod_2,
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
    )
    // (period, /* missing */, 30, "Period", 142, 2, R, ""),
    // (period_text, /* missing */, 31, "Period Text ('1st ', 'OT ', 'OT/2')", 144, 4, L, "Future", deprecate),
    // (period_description, /* missing */, 32, "Period Description ('End of 1st ')", 148, 12, L, "Future", deprecate),
    // (internal_relay, /* missing */, 33, "Internal Relay (' ' or 'z', 's', 'h')", 160, 1, L, ""),
    // (ad_panel_caption_power, /* missing */, 34, "Ad Panel / Caption Power ('c')", 161, 1, L, ""),
    // (ad_panel_caption_num1, /* missing */, 35, "Ad Panel / Caption #1 (' ' or 'c')", 162, 1, L, ""),
    // (ad_panel_caption_num2, /* missing */, 36, "Ad Panel / Caption #2 (' ' or 'c')", 163, 1, L, ""),
    // (ad_panel_caption_num3, /* missing */, 37, "Ad Panel / Caption #3 (' ' or 'c')", 164, 1, L, "Future", deprecate),
    // (ad_panel_caption_num4, /* missing */, 38, "Ad Panel / Caption #4 (' ' or 'c')", 165, 1, L, "Future", deprecate),
    // (shot_clock_time, /* missing */, 40, "Shot Clock Time (mm:ss)", 201, 8, L, ""),
    // (shot_clock_horn, /* missing */, 41, "Shot Clock Horn (' ' or 'h')", 209, 1, L, ""),
    // (inverse_time_clock, /* missing */, 42, "Inverse Time Clock (mm:ss)", 210, 8, L, "Future", deprecate),
    // (inverse_main_time_out_tod, /* missing */, 43, "Inverse/Main/Time Out/TOD (mm:ss)", 218, 8, L, "Future", deprecate),
    // (home_player_num1_number, /* missing */, 44, "Home Player #1-Number", 226, 2, R, ""),
    // (home_player_num1_penalty_time, /* missing */, 45, "Home Player #1-Penalty Time (mm:ss)", 228, 8, L, ""),
    // (home_player_num1_penalty_number, /* missing */, 46, "Home Player #1-Penalty Number", 236, 2, R, ""),
    // (home_player_num2_number, /* missing */, 47, "Home Player #2-Number", 238, 2, R, ""),
    // (home_player_num2_penalty_time, /* missing */, 48, "Home Player #2-Penalty Time (mm:ss)", 240, 8, L, ""),
    // (home_player_num2_penalty_number, /* missing */, 49, "Home Player #2-Penalty Number", 248, 2, R, ""),
    // (home_player_num3_number, /* missing */, 50, "Home Player #3-Number", 250, 2, R, ""),
    // (home_player_num3_penalty_time, /* missing */, 51, "Home Player #3-Penalty Time (mm:ss)", 252, 8, L, ""),
    // (home_player_num3_penalty_number, /* missing */, 52, "Home Player #3-Penalty Number", 260, 2, R, ""),
    // (guest_player_num1_number, /* missing */, 53, "Guest Player #1-Number", 262, 2, R, ""),
    // (guest_player_num1_penalty_time, /* missing */, 54, "Guest Player #1-Penalty Time (mm:ss)", 264, 8, L, ""),
    // (guest_player_num1_penalty_number, /* missing */, 55, "Guest Player #1-Penalty Number", 272, 2, R, ""),
    // (guest_player_num2_number, /* missing */, 56, "Guest Player #2-Number", 274, 2, R, ""),
    // (guest_player_num2_penalty_time, /* missing */, 57, "Guest Player #2-Penalty Time (mm:ss)", 276, 8, L, ""),
    // (guest_player_num2_penalty_number, /* missing */, 58, "Guest Player #2-Penalty Number", 284, 2, R, ""),
    // (guest_player_num3_number, /* missing */, 59, "Guest Player #3-Number", 286, 2, R, ""),
    // (guest_player_num3_penalty_time, /* missing */, 60, "Guest Player #3-Penalty Time (mm:ss)", 288, 8, L, ""),
    // (guest_player_num3_penalty_number, /* missing */, 61, "Guest Player #3-Penalty Number", 296, 2, R, "")
);
