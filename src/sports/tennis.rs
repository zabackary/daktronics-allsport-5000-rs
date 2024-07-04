//! This sport was generated semi-automatically and may contain errors.
super::sport_builder!(
    Tennis,
    "Tennis",
    true,
    (
        main_clock_time,
        &str,
        1,
        "Main Clock Time (hh:mm/mm:ss/ss.t)",
        1,
        5,
        L,
        ""
    ),
    (
        main_clock_time_2,
        &str,
        2,
        "Main Clock Time (hh:mm:ss/mm:ss.t)",
        6,
        8,
        L,
        ""
    ),
    (
        main_time_out_tod,
        &str,
        3,
        "Main/Time Out/TOD (hh:mm/mm:ss/ss.t)",
        14,
        5,
        L,
        ""
    ),
    (
        main_time_out_tod_2,
        &str,
        4,
        "Main/Time Out/TOD (hh:mm:ss/mm:ss.t)",
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
    (
        top_team_player_num1_name,
        &str,
        12,
        "Top Team/Player #1 Name",
        48,
        20,
        L,
        ""
    ),
    (
        bottom_team_player_num1_name,
        &str,
        13,
        "Bottom Team/Player #1 Name",
        68,
        20,
        L,
        ""
    ),
    (
        top_player_num2_name,
        &str,
        14,
        "Top Player #2 Name",
        88,
        10,
        L,
        ""
    ),
    (
        bottom_player_num2_name,
        &str,
        15,
        "Bottom Player #2 Name",
        98,
        10,
        L,
        ""
    ),
    (top_game_score, &str, 16, "Top Game Score", 108, 4, R, ""),
    (
        bottom_game_score,
        &str,
        17,
        "Bottom Game Score",
        112,
        4,
        R,
        ""
    ),
    (
        top_time_outs_left_full,
        i32,
        18,
        "Top Time Outs Left - Full",
        116,
        2,
        R,
        ""
    ),
    (
        top_time_outs_left_partial,
        i32,
        19,
        "Top Time Outs Left - Partial",
        118,
        2,
        R,
        ""
    ),
    (
        top_time_outs_left_television,
        i32,
        20,
        "Top Time Outs Left - Television",
        120,
        2,
        R,
        "Future",
        deprecate
    ),
    (
        top_time_outs_left_total,
        i32,
        21,
        "Top Time Outs Left - Total",
        122,
        2,
        R,
        ""
    ),
    (
        bottom_time_outs_left_full,
        i32,
        22,
        "Bottom Time Outs Left - Full",
        124,
        2,
        R,
        ""
    ),
    (
        bottom_time_outs_left_partial,
        i32,
        23,
        "Bottom Time Outs Left - Partial",
        126,
        2,
        R,
        ""
    ),
    (
        bottom_time_outs_left_television,
        i32,
        24,
        "Bottom Time Outs Left - Television",
        128,
        2,
        R,
        "Future",
        deprecate
    ),
    (
        bottom_time_outs_left_total,
        i32,
        25,
        "Bottom Time Outs Left - Total",
        130,
        2,
        R,
        ""
    ),
    (
        top_time_out_indicator,
        bool,
        26,
        "Top Time Out Indicator (' ' or '<')",
        132,
        1,
        L,
        ""
    ),
    (
        top_time_out_text,
        &str,
        27,
        "Top Time Out Text (‘ ‘ or ‘TIME’)",
        133,
        4,
        L,
        ""
    ),
    (
        bottom_time_out_indicator,
        bool,
        28,
        "Bottom Time Out Indicator (' ' or '>')",
        137,
        1,
        L,
        ""
    ),
    (
        bottom_time_out_text,
        &str,
        29,
        "Bottom Time Out Text (‘ ‘ or ‘TIME’)",
        138,
        4,
        L,
        ""
    ),
    (set_number, i32, 30, "Set Number", 142, 2, R, ""),
    (
        set_number_text,
        &str,
        31,
        "Set Number Text ('1st ', 'OT', 'OT/2')",
        144,
        4,
        L,
        "Future",
        deprecate
    ),
    (
        set_number_description,
        &str,
        32,
        "Set Number Description ('End of 1st ')",
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
        top_team_player_num1_serve,
        bool,
        40,
        "Top Team/Player #1 Serve (' ' or '>')",
        201,
        1,
        L,
        ""
    ),
    (
        top_player_num2_serve,
        bool,
        41,
        "Top Player #2 Serve (' ' or '>')",
        202,
        1,
        L,
        ""
    ),
    (
        top_serve_text,
        &str,
        42,
        "Top Serve Text (' ' or 'SERVE')",
        203,
        5,
        L,
        ""
    ),
    (
        bottom_team_player_num1_serve,
        bool,
        43,
        "Bottom Team/Player #1 Serve (' ' or '>')",
        208,
        1,
        L,
        ""
    ),
    (
        bottom_player_num2_serve,
        bool,
        44,
        "Bottom Player #2 Serve (' ' or '>')",
        209,
        1,
        L,
        ""
    ),
    (
        bottom_serve_text,
        &str,
        45,
        "Bottom Serve Text (' ' or 'SERVE')",
        210,
        5,
        L,
        ""
    ),
    (
        match_number,
        i32,
        46,
        "Match Number",
        215,
        4,
        R,
        "Future",
        deprecate
    ),
    (
        tie_break_text,
        &str,
        47,
        "Tie Break Text (' ' or 'TIE BREAK')",
        219,
        9,
        L,
        ""
    ),
    (top_matches_won, i32, 48, "Top Matches Won", 228, 2, R, ""),
    (
        bottom_matches_won,
        i32,
        49,
        "Bottom Matches Won",
        230,
        2,
        R,
        ""
    ),
    (top_sets_won, i32, 50, "Top Sets Won", 232, 2, R, ""),
    (
        top_games_won_1st_set_court_1,
        i32,
        51,
        "Top Games Won - 1st Set - Court 1",
        234,
        2,
        R,
        ""
    ),
    (
        top_games_won_2nd_set_court_1,
        i32,
        52,
        "Top Games Won - 2nd Set - Court 1",
        236,
        2,
        R,
        ""
    ),
    (
        top_games_won_3rd_set_court_1,
        i32,
        53,
        "Top Games Won - 3rd Set - Court 1",
        238,
        2,
        R,
        ""
    ),
    (
        top_games_won_4th_set_court_1,
        i32,
        54,
        "Top Games Won - 4th Set - Court 1",
        240,
        2,
        R,
        ""
    ),
    (
        top_games_won_5th_set_court_1,
        i32,
        55,
        "Top Games Won - 5th Set - Court 1",
        242,
        2,
        R,
        ""
    ),
    (
        top_games_won_current_set_court_1,
        i32,
        56,
        "Top Games Won - Current Set - Court 1",
        244,
        2,
        R,
        ""
    ),
    (bottom_sets_won, i32, 57, "Bottom Sets Won", 246, 2, R, ""),
    (
        bottom_games_won_1st_set_court_1,
        i32,
        58,
        "Bottom Games Won - 1st Set - Court 1",
        248,
        2,
        R,
        ""
    ),
    (
        bottom_games_won_2nd_set_court_1,
        i32,
        59,
        "Bottom Games Won - 2nd Set - Court 1",
        250,
        2,
        R,
        ""
    ),
    (
        bottom_games_won_3rd_set_court_1,
        i32,
        60,
        "Bottom Games Won - 3rd Set - Court 1",
        252,
        2,
        R,
        ""
    ),
    (
        bottom_games_won_4th_set_court_1,
        i32,
        61,
        "Bottom Games Won - 4th Set - Court 1",
        254,
        2,
        R,
        ""
    ),
    (
        bottom_games_won_5th_set_court_1,
        i32,
        62,
        "Bottom Games Won - 5th Set - Court 1",
        256,
        2,
        R,
        ""
    ),
    (
        bottom_games_won_current_set_court_1,
        i32,
        63,
        "Bottom Games Won - Current Set - Court 1",
        258,
        2,
        R,
        ""
    )
);
