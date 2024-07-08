//! The struct for viewing Basketball data streams (includes a simple version)
//!
//! The simple version of the struct includes only commonly used fields like the
//! score, team names, clock, etc., but omits extraneous fields casual sports
//! events are unlikely to fill, like player names, advertisements, etc.
//!
//! This sport was generated semi-automatically and may contain errors.
super::sport_builder!(
    Basketball,
    "Basketball",
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
    (period, i32, 30, "Period", 142, 2, R, ""),
    (
        period_text,
        &str,
        31,
        "Period Text ('1st ', 'OT ', 'OT/2')",
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
        shot_clock_time,
        &str,
        40,
        "Shot Clock Time (mm:ss)",
        201,
        8,
        L,
        ""
    ),
    (
        shot_clock_horn,
        bool,
        41,
        "Shot Clock Horn (' ' or 'h')",
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
        home_possession_arrow,
        bool,
        43,
        "Home Possession Arrow (' ' or '<')",
        211,
        1,
        L,
        "Future",
        deprecate
    ),
    (
        home_possession_text,
        &str,
        44,
        "Home Possession Text (' ' or 'POSS')",
        212,
        4,
        L,
        ""
    ),
    (
        guest_possession_indicator,
        bool,
        45,
        "Guest Possession Indicator (' ' or '>')",
        216,
        1,
        L,
        ""
    ),
    (
        guest_possession_arrow,
        bool,
        46,
        "Guest Possession Arrow (' ' or '>')",
        217,
        1,
        L,
        "Future",
        deprecate
    ),
    (
        guest_possession_text,
        &str,
        47,
        "Guest Possession Text (' ' or 'POSS')",
        218,
        4,
        L,
        ""
    ),
    (
        home_1_on_1_bonus_indicator,
        bool,
        48,
        "Home 1-on-1 Bonus Indicator (' ' or '<')",
        222,
        1,
        L,
        ""
    ),
    (
        home_2_shot_bonus_indicator,
        bool,
        49,
        "Home 2-shot Bonus Indicator (' ' or '<')",
        223,
        1,
        L,
        ""
    ),
    (
        home_bonus_text,
        &str,
        50,
        "Home Bonus Text (' ' or 'BONUS')",
        224,
        5,
        L,
        ""
    ),
    (
        guest_1_on_1_bonus_indicator,
        bool,
        51,
        "Guest 1-on-1 Bonus Indicator (' ' or '>')",
        229,
        1,
        L,
        ""
    ),
    (
        guest_2_shot_bonus_indicator,
        bool,
        52,
        "Guest 2-shot Bonus Indicator (' ' or '>')",
        230,
        1,
        L,
        ""
    ),
    (
        guest_bonus_text,
        &str,
        53,
        "Guest Bonus Text (' ' or 'BONUS')",
        231,
        5,
        L,
        ""
    ),
    (home_team_fouls, i32, 54, "Home Team Fouls", 236, 2, R, ""),
    (guest_team_fouls, i32, 55, "Guest Team Fouls", 238, 2, R, ""),
    (
        home_player_foul_points,
        &str,
        56,
        "Home Player-Foul-Points ('nn-nn-nn')",
        240,
        8,
        L,
        ""
    ),
    (
        guest_player_foul_points,
        &str,
        57,
        "Guest Player-Foul-Points ('nn-nn-nn')",
        248,
        8,
        L,
        ""
    ),
    (player_foul, &str, 58, "Player-Foul ('nnn')", 256, 3, L, ""),
    (
        player_foul_points,
        &str,
        59,
        "Player-Foul-Points ('nnnnn')",
        259,
        5,
        N,
        ""
    ),
    (
        home_score_period_1,
        i32,
        60,
        "Home Score - Period 1",
        264,
        2,
        R,
        ""
    ),
    (
        home_score_period_2,
        i32,
        61,
        "Home Score - Period 2",
        266,
        2,
        R,
        ""
    ),
    (
        home_score_period_3,
        i32,
        62,
        "Home Score - Period 3",
        268,
        2,
        R,
        ""
    ),
    (
        home_score_period_4,
        i32,
        63,
        "Home Score - Period 4",
        270,
        2,
        R,
        ""
    ),
    (
        home_score_period_5,
        i32,
        64,
        "Home Score - Period 5",
        272,
        2,
        R,
        ""
    ),
    (
        home_score_period_7,
        i32,
        66,
        "Home Score - Period 7",
        276,
        2,
        R,
        ""
    ),
    (
        home_score_period_8,
        i32,
        67,
        "Home Score - Period 8",
        278,
        2,
        R,
        ""
    ),
    (
        home_score_period_9,
        i32,
        68,
        "Home Score - Period 9",
        280,
        2,
        R,
        ""
    ),
    (
        home_score_current_period,
        i32,
        69,
        "Home Score - Current Period",
        282,
        2,
        R,
        ""
    ),
    (
        guest_score_period_1,
        i32,
        70,
        "Guest Score - Period 1",
        284,
        2,
        R,
        ""
    ),
    (
        guest_score_period_2,
        i32,
        71,
        "Guest Score - Period 2",
        286,
        2,
        R,
        ""
    ),
    (
        guest_score_period_3,
        i32,
        72,
        "Guest Score - Period 3",
        288,
        2,
        R,
        ""
    ),
    (
        guest_score_period_4,
        i32,
        73,
        "Guest Score - Period 4",
        290,
        2,
        R,
        ""
    ),
    (
        guest_score_period_5,
        i32,
        74,
        "Guest Score - Period 5",
        292,
        2,
        R,
        ""
    ),
    (
        guest_score_period_6,
        i32,
        75,
        "Guest Score - Period 6",
        294,
        2,
        R,
        ""
    ),
    (
        guest_score_period_7,
        i32,
        76,
        "Guest Score - Period 7",
        296,
        2,
        R,
        ""
    ),
    (
        guest_score_period_8,
        i32,
        77,
        "Guest Score - Period 8",
        298,
        2,
        R,
        ""
    ),
    (
        guest_score_period_9,
        i32,
        78,
        "Guest Score - Period 9",
        300,
        2,
        R,
        ""
    ),
    (
        guest_score_current_period,
        i32,
        79,
        "Guest Score - Current Period",
        302,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_1_status,
        bool,
        80,
        "Home In-game Plyr 1-Status (' ' or '>')",
        304,
        1,
        L,
        ""
    ),
    (
        home_in_game_plyr_1_number,
        i32,
        81,
        "Home In-game Plyr 1-Number",
        305,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_1_fouls,
        i32,
        82,
        "Home In-game Plyr 1-Fouls",
        307,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_1_points,
        i32,
        83,
        "Home In-game Plyr 1-Points",
        309,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_2_status,
        bool,
        84,
        "Home In-game Plyr 2-Status (' ' or '>')",
        311,
        1,
        L,
        ""
    ),
    (
        home_in_game_plyr_2_number,
        i32,
        85,
        "Home In-game Plyr 2-Number",
        312,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_2_fouls,
        i32,
        86,
        "Home In-game Plyr 2-Fouls",
        314,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_2_points,
        i32,
        87,
        "Home In-game Plyr 2-Points",
        316,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_3_status,
        bool,
        88,
        "Home In-game Plyr 3-Status (' ' or '>')",
        318,
        1,
        L,
        ""
    ),
    (
        home_in_game_plyr_3_number,
        i32,
        89,
        "Home In-game Plyr 3-Number",
        319,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_3_fouls,
        i32,
        90,
        "Home In-game Plyr 3-Fouls",
        321,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_3_points,
        i32,
        91,
        "Home In-game Plyr 3-Points",
        323,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_4_status,
        bool,
        92,
        "Home In-game Plyr 4-Status (' ' or '>')",
        325,
        1,
        L,
        ""
    ),
    (
        home_in_game_plyr_4_number,
        i32,
        93,
        "Home In-game Plyr 4-Number",
        326,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_4_fouls,
        i32,
        94,
        "Home In-game Plyr 4-Fouls",
        328,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_4_points,
        i32,
        95,
        "Home In-game Plyr 4-Points",
        330,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_5_status,
        bool,
        96,
        "Home In-game Plyr 5-Status (' ' or '>')",
        332,
        1,
        L,
        ""
    ),
    (
        home_in_game_plyr_5_number,
        i32,
        97,
        "Home In-game Plyr 5-Number",
        333,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_5_fouls,
        i32,
        98,
        "Home In-game Plyr 5-Fouls",
        335,
        2,
        R,
        ""
    ),
    (
        home_in_game_plyr_5_points,
        i32,
        99,
        "Home In-game Plyr 5-Points",
        337,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_3_points,
        i32,
        112,
        "Home Roster Plyr 3-Points",
        365,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_4_status,
        bool,
        113,
        "Home Roster Plyr 4-Status (' ' or '>')",
        367,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_4_number,
        i32,
        114,
        "Home Roster Plyr 4-Number",
        368,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_4_fouls,
        i32,
        115,
        "Home Roster Plyr 4-Fouls",
        370,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_4_points,
        i32,
        116,
        "Home Roster Plyr 4-Points",
        372,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_5_status,
        bool,
        117,
        "Home Roster Plyr 5-Status (' ' or '>')",
        374,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_5_number,
        i32,
        118,
        "Home Roster Plyr 5-Number",
        375,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_5_fouls,
        i32,
        119,
        "Home Roster Plyr 5-Fouls",
        377,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_5_points,
        i32,
        120,
        "Home Roster Plyr 5-Points",
        379,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_6_status,
        bool,
        121,
        "Home Roster Plyr 6-Status (' ' or '>')",
        381,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_6_number,
        i32,
        122,
        "Home Roster Plyr 6-Number",
        382,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_6_fouls,
        i32,
        123,
        "Home Roster Plyr 6-Fouls",
        384,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_6_points,
        i32,
        124,
        "Home Roster Plyr 6-Points",
        386,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_7_status,
        bool,
        125,
        "Home Roster Plyr 7-Status (' ' or '>')",
        388,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_7_number,
        i32,
        126,
        "Home Roster Plyr 7-Number",
        389,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_7_fouls,
        i32,
        127,
        "Home Roster Plyr 7-Fouls",
        391,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_7_points,
        i32,
        128,
        "Home Roster Plyr 7-Points",
        393,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_8_status,
        bool,
        129,
        "Home Roster Plyr 8-Status (' ' or '>')",
        395,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_8_number,
        i32,
        130,
        "Home Roster Plyr 8-Number",
        396,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_8_fouls,
        i32,
        131,
        "Home Roster Plyr 8-Fouls",
        398,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_8_points,
        i32,
        132,
        "Home Roster Plyr 8-Points",
        400,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_9_status,
        bool,
        133,
        "Home Roster Plyr 9-Status (' ' or '>')",
        402,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_9_number,
        i32,
        134,
        "Home Roster Plyr 9-Number",
        403,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_9_fouls,
        i32,
        135,
        "Home Roster Plyr 9-Fouls",
        405,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_9_points,
        i32,
        136,
        "Home Roster Plyr 9-Points",
        407,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_10_status,
        bool,
        137,
        "Home Roster Plyr 10-Status (' ' or '>')",
        409,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_10_number,
        i32,
        138,
        "Home Roster Plyr 10-Number",
        410,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_10_fouls,
        i32,
        139,
        "Home Roster Plyr 10-Fouls",
        412,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_10_points,
        i32,
        140,
        "Home Roster Plyr 10-Points",
        414,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_11_status,
        bool,
        141,
        "Home Roster Plyr 11-Status (' ' or '>')",
        416,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_11_number,
        i32,
        142,
        "Home Roster Plyr 11-Number",
        417,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_11_fouls,
        i32,
        143,
        "Home Roster Plyr 11-Fouls",
        419,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_11_points,
        i32,
        144,
        "Home Roster Plyr 11-Points",
        421,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_12_status,
        bool,
        145,
        "Home Roster Plyr 12-Status (' ' or '>')",
        423,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_12_number,
        i32,
        146,
        "Home Roster Plyr 12-Number",
        424,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_12_fouls,
        i32,
        147,
        "Home Roster Plyr 12-Fouls",
        426,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_12_points,
        i32,
        148,
        "Home Roster Plyr 12-Points",
        428,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_13_status,
        bool,
        149,
        "Home Roster Plyr 13-Status (' ' or '>')",
        430,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_13_number,
        i32,
        150,
        "Home Roster Plyr 13-Number",
        431,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_13_fouls,
        i32,
        151,
        "Home Roster Plyr 13-Fouls",
        433,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_13_points,
        i32,
        152,
        "Home Roster Plyr 13-Points",
        435,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_14_status,
        bool,
        153,
        "Home Roster Plyr 14-Status (' ' or '>')",
        437,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_14_number,
        i32,
        154,
        "Home Roster Plyr 14-Number",
        438,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_14_fouls,
        i32,
        155,
        "Home Roster Plyr 14-Fouls",
        440,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_14_points,
        i32,
        156,
        "Home Roster Plyr 14-Points",
        442,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_15_status,
        bool,
        157,
        "Home Roster Plyr 15-Status (' ' or '>')",
        444,
        1,
        L,
        ""
    ),
    (
        home_roster_plyr_15_number,
        i32,
        158,
        "Home Roster Plyr 15-Number",
        445,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_15_fouls,
        i32,
        159,
        "Home Roster Plyr 15-Fouls",
        447,
        2,
        R,
        ""
    ),
    (
        home_roster_plyr_15_points,
        i32,
        160,
        "Home Roster Plyr 15-Points",
        449,
        2,
        R,
        ""
    ),
    (
        home_assists,
        i32,
        161,
        "Home Assists",
        451,
        4,
        R,
        "From DSTI Application Only"
    ),
    (
        home_rebounds,
        i32,
        162,
        "Home Rebounds",
        455,
        4,
        R,
        "From DSTI Application Only"
    ),
    (
        home_blocked_shots,
        i32,
        163,
        "Home Blocked Shots",
        459,
        4,
        R,
        "From DSTI Application Only"
    ),
    (
        home_steals,
        i32,
        164,
        "Home Steals",
        463,
        4,
        R,
        "From DSTI Application Only"
    ),
    (
        home_total_hustle_arbs,
        &str,
        165,
        "Home Total Hustle (A,R,B,S)",
        467,
        4,
        R,
        "From DSTI Application Only"
    ),
    (
        home_total_hustle_rbs,
        &str,
        166,
        "Home Total Hustle (R,B,S)",
        471,
        4,
        R,
        "From DSTI Application Only"
    ),
    (
        guest_in_game_plyr_1_status,
        bool,
        167,
        "Guest In-game Plyr 1-Status (' ' or '>')",
        475,
        1,
        L,
        ""
    ),
    (
        guest_in_game_plyr_1_number,
        i32,
        168,
        "Guest In-game Plyr 1-Number",
        476,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_1_fouls,
        i32,
        169,
        "Guest In-game Plyr 1-Fouls",
        478,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_1_points,
        i32,
        170,
        "Guest In-game Plyr 1-Points",
        480,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_2_status,
        bool,
        171,
        "Guest In-game Plyr 2-Status (' ' or '>')",
        482,
        1,
        L,
        ""
    ),
    (
        guest_in_game_plyr_2_number,
        i32,
        172,
        "Guest In-game Plyr 2-Number",
        483,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_2_fouls,
        i32,
        173,
        "Guest In-game Plyr 2-Fouls",
        485,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_2_points,
        i32,
        174,
        "Guest In-game Plyr 2-Points",
        487,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_3_status,
        bool,
        175,
        "Guest In-game Plyr 3-Status (' ' or '>')",
        489,
        1,
        L,
        ""
    ),
    (
        guest_in_game_plyr_3_number,
        i32,
        176,
        "Guest In-game Plyr 3-Number",
        490,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_3_fouls,
        i32,
        177,
        "Guest In-game Plyr 3-Fouls",
        492,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_3_points,
        i32,
        178,
        "Guest In-game Plyr 3-Points",
        494,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_4_status,
        bool,
        179,
        "Guest In-game Plyr 4-Status (' ' or '>')",
        496,
        1,
        L,
        ""
    ),
    (
        guest_in_game_plyr_4_number,
        i32,
        180,
        "Guest In-game Plyr 4-Number",
        497,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_4_fouls,
        i32,
        181,
        "Guest In-game Plyr 4-Fouls",
        499,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_4_points,
        i32,
        182,
        "Guest In-game Plyr 4-Points",
        501,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_5_status,
        bool,
        183,
        "Guest In-game Plyr 5-Status (' ' or '>')",
        503,
        1,
        L,
        ""
    ),
    (
        guest_in_game_plyr_5_number,
        i32,
        184,
        "Guest In-game Plyr 5-Number",
        504,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_5_fouls,
        i32,
        185,
        "Guest In-game Plyr 5-Fouls",
        506,
        2,
        R,
        ""
    ),
    (
        guest_in_game_plyr_5_points,
        i32,
        186,
        "Guest In-game Plyr 5-Points",
        508,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_1_status,
        bool,
        188,
        "Guest Roster Plyr 1-Status (' ' or '>')",
        517,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_1_number,
        i32,
        189,
        "Guest Roster Plyr 1-Number",
        518,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_1_fouls,
        i32,
        190,
        "Guest Roster Plyr 1-Fouls",
        520,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_1_points,
        i32,
        191,
        "Guest Roster Plyr 1-Points",
        522,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_2_status,
        bool,
        192,
        "Guest Roster Plyr 2-Status (' ' or '>')",
        524,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_2_number,
        i32,
        193,
        "Guest Roster Plyr 2-Number",
        525,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_2_fouls,
        i32,
        194,
        "Guest Roster Plyr 2-Fouls",
        527,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_2_points,
        i32,
        195,
        "Guest Roster Plyr 2-Points",
        529,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_3_status,
        bool,
        196,
        "Guest Roster Plyr 3-Status (' ' or '>')",
        531,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_3_number,
        i32,
        197,
        "Guest Roster Plyr 3-Number",
        532,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_3_fouls,
        i32,
        198,
        "Guest Roster Plyr 3-Fouls",
        534,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_3_points,
        i32,
        199,
        "Guest Roster Plyr 3-Points",
        536,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_4_status,
        bool,
        200,
        "Guest Roster Plyr 4-Status (' ' or '>')",
        538,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_4_number,
        i32,
        201,
        "Guest Roster Plyr 4-Number",
        539,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_4_fouls,
        i32,
        202,
        "Guest Roster Plyr 4-Fouls",
        541,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_4_points,
        i32,
        203,
        "Guest Roster Plyr 4-Points",
        543,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_5_status,
        bool,
        204,
        "Guest Roster Plyr 5-Status (' ' or '>')",
        545,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_5_number,
        i32,
        205,
        "Guest Roster Plyr 5-Number",
        546,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_5_fouls,
        i32,
        206,
        "Guest Roster Plyr 5-Fouls",
        548,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_5_points,
        i32,
        207,
        "Guest Roster Plyr 5-Points",
        550,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_6_status,
        bool,
        208,
        "Guest Roster Plyr 6-Status (' ' or '>')",
        552,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_6_number,
        i32,
        209,
        "Guest Roster Plyr 6-Number",
        553,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_6_fouls,
        i32,
        210,
        "Guest Roster Plyr 6-Fouls",
        555,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_6_points,
        i32,
        211,
        "Guest Roster Plyr 6-Points",
        557,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_7_status,
        bool,
        212,
        "Guest Roster Plyr 7-Status (' ' or '>')",
        559,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_7_number,
        i32,
        213,
        "Guest Roster Plyr 7-Number",
        560,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_7_fouls,
        i32,
        214,
        "Guest Roster Plyr 7-Fouls",
        562,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_7_points,
        i32,
        215,
        "Guest Roster Plyr 7-Points",
        564,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_8_status,
        bool,
        216,
        "Guest Roster Plyr 8-Status (' ' or '>')",
        566,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_8_number,
        i32,
        217,
        "Guest Roster Plyr 8-Number",
        567,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_8_fouls,
        i32,
        218,
        "Guest Roster Plyr 8-Fouls",
        569,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_8_points,
        i32,
        219,
        "Guest Roster Plyr 8-Points",
        571,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_9_status,
        bool,
        220,
        "Guest Roster Plyr 9-Status (' ' or '>')",
        573,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_9_number,
        i32,
        221,
        "Guest Roster Plyr 9-Number",
        574,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_9_fouls,
        i32,
        222,
        "Guest Roster Plyr 9-Fouls",
        576,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_9_points,
        i32,
        223,
        "Guest Roster Plyr 9-Points",
        578,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_10_status,
        bool,
        224,
        "Guest Roster Plyr 10-Status (' ' or '>')",
        580,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_10_number,
        i32,
        225,
        "Guest Roster Plyr 10-Number",
        581,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_10_fouls,
        i32,
        226,
        "Guest Roster Plyr 10-Fouls",
        583,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_10_points,
        i32,
        227,
        "Guest Roster Plyr 10-Points",
        585,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_11_status,
        bool,
        228,
        "Guest Roster Plyr 11-Status (' ' or '>')",
        587,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_11_number,
        i32,
        229,
        "Guest Roster Plyr 11-Number",
        588,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_11_fouls,
        i32,
        230,
        "Guest Roster Plyr 11-Fouls",
        590,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_11_points,
        i32,
        231,
        "Guest Roster Plyr 11-Points",
        592,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_12_status,
        bool,
        232,
        "Guest Roster Plyr 12-Status (' ' or '>')",
        594,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_12_number,
        i32,
        233,
        "Guest Roster Plyr 12-Number",
        595,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_12_fouls,
        i32,
        234,
        "Guest Roster Plyr 12-Fouls",
        597,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_12_points,
        i32,
        235,
        "Guest Roster Plyr 12-Points",
        599,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_13_status,
        bool,
        236,
        "Guest Roster Plyr 13-Status (' ' or '>')",
        601,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_13_number,
        i32,
        237,
        "Guest Roster Plyr 13-Number",
        602,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_13_fouls,
        i32,
        238,
        "Guest Roster Plyr 13-Fouls",
        604,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_13_points,
        i32,
        239,
        "Guest Roster Plyr 13-Points",
        606,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_14_status,
        bool,
        240,
        "Guest Roster Plyr 14-Status (' ' or '>')",
        608,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_14_number,
        i32,
        241,
        "Guest Roster Plyr 14-Number",
        609,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_14_fouls,
        i32,
        242,
        "Guest Roster Plyr 14-Fouls",
        611,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_14_points,
        i32,
        243,
        "Guest Roster Plyr 14-Points",
        613,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_15_status,
        bool,
        244,
        "Guest Roster Plyr 15-Status (' ' or '>')",
        615,
        1,
        L,
        ""
    ),
    (
        guest_roster_plyr_15_number,
        i32,
        245,
        "Guest Roster Plyr 15-Number",
        616,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_15_fouls,
        i32,
        246,
        "Guest Roster Plyr 15-Fouls",
        618,
        2,
        R,
        ""
    ),
    (
        guest_roster_plyr_15_points,
        i32,
        247,
        "Guest Roster Plyr 15-Points",
        620,
        2,
        R,
        ""
    ),
    (
        guest_assists,
        i32,
        248,
        "Guest Assists",
        622,
        4,
        R,
        "From DSTI Application Only"
    ),
    (
        guest_rebounds,
        i32,
        249,
        "Guest Rebounds",
        626,
        4,
        R,
        "From DSTI Application Only"
    ),
    (
        guest_blocked_shots,
        i32,
        250,
        "Guest Blocked Shots",
        630,
        4,
        R,
        "From DSTI Application Only"
    ),
    (
        guest_steals,
        i32,
        251,
        "Guest Steals",
        634,
        4,
        R,
        "From DSTI Application Only"
    ),
    (
        guest_total_hustle_arbs,
        &str,
        252,
        "Guest Total Hustle (A,R,B,S)",
        638,
        4,
        R,
        "From DSTI Application Only"
    ),
    (
        guest_total_hustle_rbs,
        &str,
        253,
        "Guest Total Hustle (R,B,S)",
        642,
        4,
        R,
        "From DSTI Application Only"
    )
);

super::sport_builder!(
    SimpleBasketball,
    "Simple Basketball",
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
    (home_team_score, i32, 16, "Home Team Score", 108, 4, R, ""),
    (guest_team_score, i32, 17, "Guest Team Score", 112, 4, R, ""),
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
        "Period Text ('1st ', 'OT ', 'OT/2')",
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
    (home_team_fouls, i32, 54, "Home Team Fouls", 236, 2, R, ""),
    (guest_team_fouls, i32, 55, "Guest Team Fouls", 238, 2, R, ""),
    (
        home_score_period_1,
        i32,
        60,
        "Home Score - Period 1",
        264,
        2,
        R,
        ""
    ),
    (
        home_score_period_2,
        i32,
        61,
        "Home Score - Period 2",
        266,
        2,
        R,
        ""
    ),
    (
        home_score_period_3,
        i32,
        62,
        "Home Score - Period 3",
        268,
        2,
        R,
        ""
    ),
    (
        home_score_period_4,
        i32,
        63,
        "Home Score - Period 4",
        270,
        2,
        R,
        ""
    ),
    (
        home_score_period_5,
        i32,
        64,
        "Home Score - Period 5",
        272,
        2,
        R,
        ""
    ),
    (
        home_score_period_7,
        i32,
        66,
        "Home Score - Period 7",
        276,
        2,
        R,
        ""
    ),
    (
        home_score_period_8,
        i32,
        67,
        "Home Score - Period 8",
        278,
        2,
        R,
        ""
    ),
    (
        home_score_period_9,
        i32,
        68,
        "Home Score - Period 9",
        280,
        2,
        R,
        ""
    ),
    (
        home_score_current_period,
        i32,
        69,
        "Home Score - Current Period",
        282,
        2,
        R,
        ""
    ),
    (
        guest_score_period_1,
        i32,
        70,
        "Guest Score - Period 1",
        284,
        2,
        R,
        ""
    ),
    (
        guest_score_period_2,
        i32,
        71,
        "Guest Score - Period 2",
        286,
        2,
        R,
        ""
    ),
    (
        guest_score_period_3,
        i32,
        72,
        "Guest Score - Period 3",
        288,
        2,
        R,
        ""
    ),
    (
        guest_score_period_4,
        i32,
        73,
        "Guest Score - Period 4",
        290,
        2,
        R,
        ""
    ),
    (
        guest_score_period_5,
        i32,
        74,
        "Guest Score - Period 5",
        292,
        2,
        R,
        ""
    ),
    (
        guest_score_period_6,
        i32,
        75,
        "Guest Score - Period 6",
        294,
        2,
        R,
        ""
    ),
    (
        guest_score_period_7,
        i32,
        76,
        "Guest Score - Period 7",
        296,
        2,
        R,
        ""
    ),
    (
        guest_score_period_8,
        i32,
        77,
        "Guest Score - Period 8",
        298,
        2,
        R,
        ""
    ),
    (
        guest_score_period_9,
        i32,
        78,
        "Guest Score - Period 9",
        300,
        2,
        R,
        ""
    ),
    (
        guest_score_current_period,
        i32,
        79,
        "Guest Score - Current Period",
        302,
        2,
        R,
        ""
    )
);
