//! The struct for viewing Volleyball data streams (includes a simple version)
//!
//! The simple version of the struct includes only commonly used fields like the
//! score, team names, clock, etc., but omits extraneous fields casual sports
//! events are unlikely to fill, like player names, advertisements, etc.
//!
//! This sport was generated semi-automatically and may contain errors.
super::sport_builder!(
    Volleyball,
    "Volleyball",
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
    (game, i32, 30, "Game", 142, 2, R, ""),
    (
        game_text,
        &str,
        31,
        "Game Text ('1st ', 'OT ', 'OT/2')",
        144,
        4,
        L,
        "Future",
        deprecate
    ),
    (
        game_description,
        &str,
        32,
        "Game Description ('End of 1st')",
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
        home_serve_indicator,
        bool,
        40,
        "Home Serve Indicator (' ' or '<')",
        201,
        1,
        L,
        ""
    ),
    (
        home_serve_arrow,
        bool,
        41,
        "Home Serve Arrow (' ' or '<')",
        202,
        1,
        L,
        "Future",
        deprecate
    ),
    (
        home_serve_text,
        &str,
        42,
        "Home Serve Text (' ' or 'SERVE')",
        203,
        5,
        L,
        ""
    ),
    (
        guest_serve_indicator,
        bool,
        43,
        "Guest Serve Indicator (' ' or '>')",
        208,
        1,
        L,
        ""
    ),
    (
        guest_serve_arrow,
        bool,
        44,
        "Guest Serve Arrow (' ' or '>')",
        209,
        1,
        L,
        "Future",
        deprecate
    ),
    (
        guest_serve_text,
        &str,
        45,
        "Guest Serve Text (' ' or 'SERVE')",
        210,
        5,
        L,
        ""
    ),
    (home_games_won, i32, 46, "Home Games Won", 215, 2, R, ""),
    (guest_games_won, i32, 47, "Guest Games Won", 217, 2, R, ""),
    (match_number, i32, 48, "Match Number", 219, 3, R, ""),
    (
        home_score_game_1,
        i32,
        49,
        "Home Score - Game 1",
        222,
        2,
        R,
        ""
    ),
    (
        home_score_game_2,
        i32,
        50,
        "Home Score - Game 2",
        224,
        2,
        R,
        ""
    ),
    (
        home_score_game_3,
        i32,
        51,
        "Home Score - Game 3",
        226,
        2,
        R,
        ""
    ),
    (
        home_score_game_4,
        i32,
        52,
        "Home Score - Game 4",
        228,
        2,
        R,
        ""
    ),
    (
        home_score_game_5,
        i32,
        53,
        "Home Score - Game 5",
        230,
        2,
        R,
        ""
    ),
    (
        home_score_game_6,
        i32,
        54,
        "Home Score - Game 6",
        232,
        2,
        R,
        ""
    ),
    (
        home_score_game_7,
        i32,
        55,
        "Home Score - Game 7",
        234,
        2,
        R,
        ""
    ),
    (
        home_score_game_8,
        i32,
        56,
        "Home Score - Game 8",
        236,
        2,
        R,
        ""
    ),
    (
        home_score_game_9,
        i32,
        57,
        "Home Score - Game 9",
        238,
        2,
        R,
        ""
    ),
    (
        home_score_current_game,
        i32,
        58,
        "Home Score - Current Game",
        240,
        2,
        R,
        ""
    ),
    (
        guest_score_game_1,
        i32,
        59,
        "Guest Score - Game 1",
        242,
        2,
        R,
        ""
    ),
    (
        guest_score_game_2,
        i32,
        60,
        "Guest Score - Game 2",
        244,
        2,
        R,
        ""
    ),
    (
        guest_score_game_3,
        i32,
        61,
        "Guest Score - Game 3",
        246,
        2,
        R,
        ""
    ),
    (
        guest_score_game_4,
        i32,
        62,
        "Guest Score - Game 4",
        248,
        2,
        R,
        ""
    ),
    (
        guest_score_game_5,
        i32,
        63,
        "Guest Score - Game 5",
        250,
        2,
        R,
        ""
    ),
    (
        guest_score_game_6,
        i32,
        64,
        "Guest Score - Game 6",
        252,
        2,
        R,
        ""
    ),
    (
        guest_score_game_7,
        i32,
        65,
        "Guest Score - Game 7",
        254,
        2,
        R,
        ""
    ),
    (
        guest_score_game_8,
        i32,
        66,
        "Guest Score - Game 8",
        256,
        2,
        R,
        ""
    ),
    (
        guest_score_game_9,
        i32,
        67,
        "Guest Score - Game 9",
        258,
        2,
        R,
        ""
    ),
    (
        guest_score_current_game,
        i32,
        68,
        "Guest Score - Current Game",
        260,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_1_status,
        bool,
        69,
        "Home In-game Plyr 1-Status (' ' or '>')",
        262,
        1,
        L,
        ""
    ),
    (
        home_in_game_plyr_1_number,
        i32,
        70,
        "Home In-game Plyr 1-Number",
        263,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_1_user_defined_1,
        &str,
        71,
        "Home In-game Plyr 1-User Defined 1",
        265,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_1_user_defined_2,
        &str,
        72,
        "Home In-game Plyr 1-User Defined 2",
        267,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_2_status,
        bool,
        73,
        "Home In-game Plyr 2-Status (' ' or '>')",
        269,
        1,
        L,
        ""
    ),
    (
        home_in_game_plyr_2_number,
        i32,
        74,
        "Home In-game Plyr 2-Number",
        270,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_2_user_defined_1,
        &str,
        75,
        "Home In-game Plyr 2-User Defined 1",
        272,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_2_user_defined_2,
        &str,
        76,
        "Home In-game Plyr 2-User Defined 2",
        274,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_3_status,
        bool,
        77,
        "Home In-game Plyr 3-Status (' ' or '>')",
        276,
        1,
        L,
        ""
    ),
    (
        home_in_game_plyr_3_number,
        i32,
        78,
        "Home In-game Plyr 3-Number",
        277,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_3_user_defined_1,
        &str,
        79,
        "Home In-game Plyr 3-User Defined 1",
        279,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_3_user_defined_2,
        &str,
        80,
        "Home In-game Plyr 3-User Defined 2",
        281,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_4_status,
        bool,
        81,
        "Home In-game Plyr 4-Status (' ' or '>')",
        283,
        1,
        L,
        ""
    ),
    (
        home_in_game_plyr_4_number,
        i32,
        82,
        "Home In-game Plyr 4-Number",
        284,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_4_user_defined_1,
        &str,
        83,
        "Home In-game Plyr 4-User Defined 1",
        286,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_4_user_defined_2,
        &str,
        84,
        "Home In-game Plyr 4-User Defined 2",
        288,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_5_status,
        bool,
        85,
        "Home In-game Plyr 5-Status (' ' or '>')",
        290,
        1,
        L,
        ""
    ),
    (
        home_in_game_plyr_5_number,
        i32,
        86,
        "Home In-game Plyr 5-Number",
        291,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_5_user_defined_1,
        &str,
        87,
        "Home In-game Plyr 5-User Defined 1",
        293,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_5_user_defined_2,
        &str,
        88,
        "Home In-game Plyr 5-User Defined 2",
        295,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_6_status,
        bool,
        89,
        "Home In-game Plyr 6-Status (' ' or '>')",
        297,
        1,
        L,
        ""
    ),
    (
        home_in_game_plyr_6_number,
        i32,
        90,
        "Home In-game Plyr 6-Number",
        298,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_6_user_defined_1,
        &str,
        91,
        "Home In-game Plyr 6-User Defined 1",
        300,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_6_user_defined_2,
        &str,
        92,
        "Home In-game Plyr 6-User Defined 2",
        302,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_1_status,
        bool,
        93,
        "Home Roster Plyr 1-Status (' ' or '>')",
        304,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_1_number,
        i32,
        94,
        "Home Roster Plyr 1-Number",
        305,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_1_user_defined_1,
        &str,
        95,
        "Home Roster Plyr 1-User Defined 1",
        307,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_1_user_defined_2,
        &str,
        96,
        "Home Roster Plyr 1-User Defined 2",
        309,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_2_status,
        bool,
        97,
        "Home Roster Plyr 2-Status (' ' or '>')",
        311,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_2_number,
        i32,
        98,
        "Home Roster Plyr 2-Number",
        312,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_2_user_defined_1,
        &str,
        99,
        "Home Roster Plyr 2-User Defined 1",
        314,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_2_user_defined_2,
        &str,
        100,
        "Home Roster Plyr 2-User Defined 2",
        316,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_3_status,
        bool,
        101,
        "Home Roster Plyr 3-Status (' ' or '>')",
        318,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_3_number,
        i32,
        102,
        "Home Roster Plyr 3-Number",
        319,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_3_user_defined_1,
        &str,
        103,
        "Home Roster Plyr 3-User Defined 1",
        321,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_3_user_defined_2,
        &str,
        104,
        "Home Roster Plyr 3-User Defined 2",
        323,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_4_status,
        bool,
        105,
        "Home Roster Plyr 4-Status (' ' or '>')",
        325,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_4_number,
        i32,
        106,
        "Home Roster Plyr 4-Number",
        326,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_4_user_defined_1,
        &str,
        107,
        "Home Roster Plyr 4-User Defined 1",
        328,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_4_user_defined_2,
        &str,
        108,
        "Home Roster Plyr 4-User Defined 2",
        330,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_5_status,
        bool,
        109,
        "Home Roster Plyr 5-Status (' ' or '>')",
        332,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_5_number,
        i32,
        110,
        "Home Roster Plyr 5-Number",
        333,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_5_user_defined_1,
        &str,
        111,
        "Home Roster Plyr 5-User Defined 1",
        335,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_5_user_defined_2,
        &str,
        112,
        "Home Roster Plyr 5-User Defined 2",
        337,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_6_status,
        bool,
        113,
        "Home Roster Plyr 6-Status (' ' or '>')",
        339,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_6_number,
        i32,
        114,
        "Home Roster Plyr 6-Number",
        340,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_6_user_defined_1,
        &str,
        115,
        "Home Roster Plyr 6-User Defined 1",
        342,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_6_user_defined_2,
        &str,
        116,
        "Home Roster Plyr 6-User Defined 2",
        344,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_7_status,
        bool,
        117,
        "Home Roster Plyr 7-Status (' ' or '>')",
        346,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_7_number,
        i32,
        118,
        "Home Roster Plyr 7-Number",
        347,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_7_user_defined_1,
        &str,
        119,
        "Home Roster Plyr 7-User Defined 1",
        349,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_7_user_defined_2,
        &str,
        120,
        "Home Roster Plyr 7-User Defined 2",
        351,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_8_status,
        bool,
        121,
        "Home Roster Plyr 8-Status (' ' or '>')",
        353,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_8_number,
        i32,
        122,
        "Home Roster Plyr 8-Number",
        354,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_8_user_defined_1,
        &str,
        123,
        "Home Roster Plyr 8-User Defined 1",
        356,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_8_user_defined_2,
        &str,
        124,
        "Home Roster Plyr 8-User Defined 2",
        358,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_9_status,
        bool,
        125,
        "Home Roster Plyr 9-Status (' ' or '>')",
        360,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_9_number,
        i32,
        126,
        "Home Roster Plyr 9-Number",
        361,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_9_user_defined_1,
        &str,
        127,
        "Home Roster Plyr 9-User Defined 1",
        363,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_9_user_defined_2,
        &str,
        128,
        "Home Roster Plyr 9-User Defined 2",
        365,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_10_status,
        bool,
        129,
        "Home Roster Plyr 10-Status (' ' or '>')",
        367,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_10_number,
        i32,
        130,
        "Home Roster Plyr 10-Number",
        368,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_10_user_defined_1,
        &str,
        131,
        "Home Roster Plyr 10-User Defined 1",
        370,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_10_user_defined_2,
        &str,
        132,
        "Home Roster Plyr 10-User Defined 2",
        372,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_11_status,
        bool,
        133,
        "Home Roster Plyr 11-Status (' ' or '>')",
        374,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_11_number,
        i32,
        134,
        "Home Roster Plyr 11-Number",
        375,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_11_user_defined_1,
        &str,
        135,
        "Home Roster Plyr 11-User Defined 1",
        377,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_11_user_defined_2,
        &str,
        136,
        "Home Roster Plyr 11-User Defined 2",
        379,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_12_status,
        bool,
        137,
        "Home Roster Plyr 12-Status (' ' or '>')",
        381,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_12_number,
        i32,
        138,
        "Home Roster Plyr 12-Number",
        382,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_12_user_defined_1,
        &str,
        139,
        "Home Roster Plyr 12-User Defined 1",
        384,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_12_user_defined_2,
        &str,
        140,
        "Home Roster Plyr 12-User Defined 2",
        386,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_13_status,
        bool,
        141,
        "Home Roster Plyr 13-Status (' ' or '>')",
        388,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_13_number,
        i32,
        142,
        "Home Roster Plyr 13-Number",
        389,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_13_user_defined_1,
        &str,
        143,
        "Home Roster Plyr 13-User Defined 1",
        391,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_13_user_defined_2,
        &str,
        144,
        "Home Roster Plyr 13-User Defined 2",
        393,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_14_status,
        bool,
        145,
        "Home Roster Plyr 14-Status (' ' or '>')",
        395,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_14_number,
        i32,
        146,
        "Home Roster Plyr 14-Number",
        396,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_14_user_defined_1,
        &str,
        147,
        "Home Roster Plyr 14-User Defined 1",
        398,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_14_user_defined_2,
        &str,
        148,
        "Home Roster Plyr 14-User Defined 2",
        400,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_15_status,
        bool,
        149,
        "Home Roster Plyr 15-Status (' ' or '>')",
        402,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_15_number,
        i32,
        150,
        "Home Roster Plyr 15-Number",
        403,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_15_user_defined_1,
        &str,
        151,
        "Home Roster Plyr 15-User Defined 1",
        405,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_15_user_defined_2,
        &str,
        152,
        "Home Roster Plyr 15-User Defined 2",
        407,
        2,
        R,
        ""
    ),
    (home_aces, i32, 153, "Home Aces", 409, 4, R, ""),
    (home_kills, i32, 154, "Home Kills", 413, 4, R, ""),
    (home_blocks, i32, 155, "Home Blocks", 417, 4, R, ""),
    (home_digs, i32, 156, "Home Digs", 421, 4, R, ""),
    (
        home_total_hustle,
        i32,
        157,
        "Home Total Hustle",
        425,
        4,
        R,
        ""
    ),
    (
        guest_in_game_plyr_1_status,
        bool,
        158,
        "Guest In-game Plyr 1-Status (' ' or '>')",
        429,
        1,
        L,
        ""
    ),
    (
        guest_in_game_plyr_1_number,
        i32,
        159,
        "Guest In-game Plyr 1-Number",
        430,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_1_user_defined_1,
        &str,
        160,
        "Guest In-game Plyr 1-User Defined 1",
        432,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_1_user_defined_2,
        &str,
        161,
        "Guest In-game Plyr 1-User Defined 2",
        434,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_2_status,
        bool,
        162,
        "Guest In-game Plyr 2-Status (' ' or '>')",
        436,
        1,
        L,
        ""
    ),
    (
        guest_in_game_plyr_2_number,
        i32,
        163,
        "Guest In-game Plyr 2-Number",
        437,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_2_user_defined_1,
        &str,
        164,
        "Guest In-game Plyr 2-User Defined 1",
        439,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_2_user_defined_2,
        &str,
        165,
        "Guest In-game Plyr 2-User Defined 2",
        441,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_3_status,
        bool,
        166,
        "Guest In-game Plyr 3-Status (' ' or '>')",
        443,
        1,
        L,
        ""
    ),
    (
        guest_in_game_plyr_3_number,
        i32,
        167,
        "Guest In-game Plyr 3-Number",
        444,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_3_user_defined_1,
        &str,
        168,
        "Guest In-game Plyr 3-User Defined 1",
        446,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_3_user_defined_2,
        &str,
        169,
        "Guest In-game Plyr 3-User Defined 2",
        448,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_4_status,
        bool,
        170,
        "Guest In-game Plyr 4-Status (' ' or '>')",
        450,
        1,
        L,
        ""
    ),
    (
        guest_in_game_plyr_4_number,
        i32,
        171,
        "Guest In-game Plyr 4-Number",
        451,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_4_user_defined_1,
        &str,
        172,
        "Guest In-game Plyr 4-User Defined 1",
        453,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_4_user_defined_2,
        &str,
        173,
        "Guest In-game Plyr 4-User Defined 2",
        455,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_5_status,
        bool,
        174,
        "Guest In-game Plyr 5-Status (' ' or '>')",
        457,
        1,
        L,
        ""
    ),
    (
        guest_in_game_plyr_5_number,
        i32,
        175,
        "Guest In-game Plyr 5-Number",
        458,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_5_user_defined_1,
        &str,
        176,
        "Guest In-game Plyr 5-User Defined 1",
        460,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_5_user_defined_2,
        &str,
        177,
        "Guest In-game Plyr 5-User Defined 2",
        462,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_6_status,
        bool,
        178,
        "Guest In-game Plyr 6-Status (' ' or '>')",
        464,
        1,
        L,
        ""
    ),
    (
        guest_in_game_plyr_6_number,
        i32,
        179,
        "Guest In-game Plyr 6-Number",
        465,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_6_user_defined_1,
        &str,
        180,
        "Guest In-game Plyr 6-User Defined 1",
        467,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_6_user_defined_2,
        &str,
        181,
        "Guest In-game Plyr 6-User Defined 2",
        469,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_1_status,
        bool,
        182,
        "Guest Roster Plyr 1-Status (' ' or '>')",
        471,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_1_number,
        i32,
        183,
        "Guest Roster Plyr 1-Number",
        472,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_1_user_defined_1,
        &str,
        184,
        "Guest Roster Plyr 1-User Defined 1",
        474,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_1_user_defined_2,
        &str,
        185,
        "Guest Roster Plyr 1-User Defined 2",
        476,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_2_status,
        bool,
        186,
        "Guest Roster Plyr 2-Status (' ' or '>')",
        478,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_2_number,
        i32,
        187,
        "Guest Roster Plyr 2-Number",
        479,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_2_user_defined_1,
        &str,
        188,
        "Guest Roster Plyr 2-User Defined 1",
        481,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_2_user_defined_2,
        &str,
        189,
        "Guest Roster Plyr 2-User Defined 2",
        483,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_3_status,
        bool,
        190,
        "Guest Roster Plyr 3-Status (' ' or '>')",
        485,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_3_number,
        i32,
        191,
        "Guest Roster Plyr 3-Number",
        486,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_3_user_defined_1,
        &str,
        192,
        "Guest Roster Plyr 3-User Defined 1",
        488,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_3_user_defined_2,
        &str,
        193,
        "Guest Roster Plyr 3-User Defined 2",
        490,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_4_status,
        bool,
        194,
        "Guest Roster Plyr 4-Status (' ' or '>')",
        492,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_4_number,
        i32,
        195,
        "Guest Roster Plyr 4-Number",
        493,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_4_user_defined_1,
        &str,
        196,
        "Guest Roster Plyr 4-User Defined 1",
        495,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_4_user_defined_2,
        &str,
        197,
        "Guest Roster Plyr 4-User Defined 2",
        497,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_5_status,
        bool,
        198,
        "Guest Roster Plyr 5-Status (' ' or '>')",
        499,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_5_number,
        i32,
        199,
        "Guest Roster Plyr 5-Number",
        500,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_5_user_defined_1,
        &str,
        200,
        "Guest Roster Plyr 5-User Defined 1",
        502,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_5_user_defined_2,
        &str,
        201,
        "Guest Roster Plyr 5-User Defined 2",
        504,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_6_status,
        bool,
        202,
        "Guest Roster Plyr 6-Status (' ' or '>')",
        506,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_6_number,
        i32,
        203,
        "Guest Roster Plyr 6-Number",
        507,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_6_user_defined_1,
        &str,
        204,
        "Guest Roster Plyr 6-User Defined 1",
        509,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_6_user_defined_2,
        &str,
        205,
        "Guest Roster Plyr 6-User Defined 2",
        511,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_7_status,
        bool,
        206,
        "Guest Roster Plyr 7-Status (' ' or '>')",
        513,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_7_number,
        i32,
        207,
        "Guest Roster Plyr 7-Number",
        514,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_7_user_defined_1,
        &str,
        208,
        "Guest Roster Plyr 7-User Defined 1",
        516,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_7_user_defined_2,
        &str,
        209,
        "Guest Roster Plyr 7-User Defined 2",
        518,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_8_status,
        bool,
        210,
        "Guest Roster Plyr 8-Status (' ' or '>')",
        520,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_8_number,
        i32,
        211,
        "Guest Roster Plyr 8-Number",
        521,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_8_user_defined_1,
        &str,
        212,
        "Guest Roster Plyr 8-User Defined 1",
        523,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_8_user_defined_2,
        &str,
        213,
        "Guest Roster Plyr 8-User Defined 2",
        525,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_9_status,
        bool,
        214,
        "Guest Roster Plyr 9-Status (' ' or '>')",
        527,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_9_number,
        i32,
        215,
        "Guest Roster Plyr 9-Number",
        528,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_9_user_defined_1,
        &str,
        216,
        "Guest Roster Plyr 9-User Defined 1",
        530,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_9_user_defined_2,
        &str,
        217,
        "Guest Roster Plyr 9-User Defined 2",
        532,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_10_status,
        bool,
        218,
        "Guest Roster Plyr 10-Status (' ' or '>')",
        534,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_10_number,
        i32,
        219,
        "Guest Roster Plyr 10-Number",
        535,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_10_user_defined_1,
        &str,
        220,
        "Guest Roster Plyr 10-User Defined 1",
        537,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_10_user_defined_2,
        &str,
        221,
        "Guest Roster Plyr 10-User Defined 2",
        539,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_11_status,
        bool,
        222,
        "Guest Roster Plyr 11-Status (' ' or '>')",
        541,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_11_number,
        i32,
        223,
        "Guest Roster Plyr 11-Number",
        542,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_11_user_defined_1,
        &str,
        224,
        "Guest Roster Plyr 11-User Defined 1",
        544,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_11_user_defined_2,
        &str,
        225,
        "Guest Roster Plyr 11-User Defined 2",
        546,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_12_status,
        bool,
        226,
        "Guest Roster Plyr 12-Status (' ' or '>')",
        548,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_12_number,
        i32,
        227,
        "Guest Roster Plyr 12-Number",
        549,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_12_user_defined_1,
        &str,
        228,
        "Guest Roster Plyr 12-User Defined 1",
        551,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_12_user_defined_2,
        &str,
        229,
        "Guest Roster Plyr 12-User Defined 2",
        553,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_13_status,
        bool,
        230,
        "Guest Roster Plyr 13-Status (' ' or '>')",
        555,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_13_number,
        i32,
        231,
        "Guest Roster Plyr 13-Number",
        556,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_13_user_defined_1,
        &str,
        232,
        "Guest Roster Plyr 13-User Defined 1",
        558,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_13_user_defined_2,
        &str,
        233,
        "Guest Roster Plyr 13-User Defined 2",
        560,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_14_status,
        bool,
        234,
        "Guest Roster Plyr 14-Status (' ' or '>')",
        562,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_14_number,
        i32,
        235,
        "Guest Roster Plyr 14-Number",
        563,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_14_user_defined_1,
        &str,
        236,
        "Guest Roster Plyr 14-User Defined 1",
        565,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_14_user_defined_2,
        &str,
        237,
        "Guest Roster Plyr 14-User Defined 2",
        567,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_15_status,
        bool,
        238,
        "Guest Roster Plyr 15-Status (' ' or '>')",
        569,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_15_number,
        i32,
        239,
        "Guest Roster Plyr 15-Number",
        570,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_15_user_defined_1,
        &str,
        240,
        "Guest Roster Plyr 15-User Defined 1",
        572,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_15_user_defined_2,
        &str,
        241,
        "Guest Roster Plyr 15-User Defined 2",
        574,
        2,
        R,
        ""
    ),
    (guest_aces, i32, 242, "Guest Aces", 576, 4, R, ""),
    (guest_kills, i32, 243, "Guest Kills", 580, 4, R, ""),
    (guest_blocks, i32, 244, "Guest Blocks", 584, 4, R, ""),
    (guest_digs, i32, 245, "Guest Digs", 588, 4, R, ""),
    (
        guest_total_hustle,
        i32,
        246,
        "Guest Total Hustle",
        592,
        4,
        R,
        ""
    )
);

super::sport_builder!(
    SimpleVolleyball,
    "Simple Volleyball",
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
    (game, i32, 30, "Game", 142, 2, R, ""),
    (
        game_text,
        &str,
        31,
        "Game Text ('1st ', 'OT ', 'OT/2')",
        144,
        4,
        L,
        "Future",
        deprecate
    ),
    (
        game_description,
        &str,
        32,
        "Game Description ('End of 1st')",
        148,
        12,
        L,
        "Future",
        deprecate
    ),
    (
        home_serve_indicator,
        bool,
        40,
        "Home Serve Indicator (' ' or '<')",
        201,
        1,
        L,
        ""
    ),
    (
        guest_serve_indicator,
        bool,
        43,
        "Guest Serve Indicator (' ' or '>')",
        208,
        1,
        L,
        ""
    ),
    (home_games_won, i32, 46, "Home Games Won", 215, 2, R, ""),
    (guest_games_won, i32, 47, "Guest Games Won", 217, 2, R, ""),
    (match_number, i32, 48, "Match Number", 219, 3, R, ""),
    (
        home_score_game_1,
        i32,
        49,
        "Home Score - Game 1",
        222,
        2,
        R,
        ""
    ),
    (
        home_score_game_2,
        i32,
        50,
        "Home Score - Game 2",
        224,
        2,
        R,
        ""
    ),
    (
        home_score_game_3,
        i32,
        51,
        "Home Score - Game 3",
        226,
        2,
        R,
        ""
    ),
    (
        home_score_game_4,
        i32,
        52,
        "Home Score - Game 4",
        228,
        2,
        R,
        ""
    ),
    (
        home_score_game_5,
        i32,
        53,
        "Home Score - Game 5",
        230,
        2,
        R,
        ""
    ),
    (
        home_score_game_6,
        i32,
        54,
        "Home Score - Game 6",
        232,
        2,
        R,
        ""
    ),
    (
        home_score_game_7,
        i32,
        55,
        "Home Score - Game 7",
        234,
        2,
        R,
        ""
    ),
    (
        home_score_game_8,
        i32,
        56,
        "Home Score - Game 8",
        236,
        2,
        R,
        ""
    ),
    (
        home_score_game_9,
        i32,
        57,
        "Home Score - Game 9",
        238,
        2,
        R,
        ""
    ),
    (
        home_score_current_game,
        i32,
        58,
        "Home Score - Current Game",
        240,
        2,
        R,
        ""
    ),
    (
        guest_score_game_1,
        i32,
        59,
        "Guest Score - Game 1",
        242,
        2,
        R,
        ""
    ),
    (
        guest_score_game_2,
        i32,
        60,
        "Guest Score - Game 2",
        244,
        2,
        R,
        ""
    ),
    (
        guest_score_game_3,
        i32,
        61,
        "Guest Score - Game 3",
        246,
        2,
        R,
        ""
    ),
    (
        guest_score_game_4,
        i32,
        62,
        "Guest Score - Game 4",
        248,
        2,
        R,
        ""
    ),
    (
        guest_score_game_5,
        i32,
        63,
        "Guest Score - Game 5",
        250,
        2,
        R,
        ""
    ),
    (
        guest_score_game_6,
        i32,
        64,
        "Guest Score - Game 6",
        252,
        2,
        R,
        ""
    ),
    (
        guest_score_game_7,
        i32,
        65,
        "Guest Score - Game 7",
        254,
        2,
        R,
        ""
    ),
    (
        guest_score_game_8,
        i32,
        66,
        "Guest Score - Game 8",
        256,
        2,
        R,
        ""
    ),
    (
        guest_score_game_9,
        i32,
        67,
        "Guest Score - Game 9",
        258,
        2,
        R,
        ""
    ),
    (
        guest_score_current_game,
        i32,
        68,
        "Guest Score - Current Game",
        260,
        2,
        R,
        ""
    )
);
