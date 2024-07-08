//! The struct for viewing Auto Racing/AMB data streams
//!
//! This sport was generated semi-automatically and may contain errors.
super::sport_builder!(
    AutoRacing,
    "Auto Racing/AMB Interface",
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
    (
        current_driver_name,
        &str,
        40,
        "Current Driver Name",
        201,
        12,
        L,
        ""
    ),
    (current_lap_num, i32, 41, "Current Lap #", 213, 5, R, ""),
    (current_car_num, i32, 42, "Current Car #", 218, 3, R, ""),
    (current_position, i32, 43, "Current Position", 221, 2, R, ""),
    (
        current_speed,
        &str,
        44,
        "Current Speed (xxx.xxx)",
        223,
        7,
        R,
        ""
    ),
    (
        current_lap_time,
        &str,
        45,
        "Current Lap Time (hh:mm:ss.dcm)",
        230,
        12,
        R,
        ""
    ),
    (
        current_lap_num_lap_time_black_flag,
        &str,
        46,
        "Current Lap #/Lap Time/Black Flag",
        242,
        12,
        R,
        ""
    ),
    (
        lap_indicator,
        bool,
        47,
        "Lap Indicator (' ' or 'L')",
        254,
        1,
        R,
        ""
    ),
    (
        time_indicator,
        bool,
        48,
        "Time Indicator (' ' or 'T')",
        255,
        1,
        R,
        ""
    ),
    (
        black_flag_indicator,
        bool,
        49,
        "Black Flag Indicator (' ' or 'B')",
        256,
        1,
        R,
        ""
    ),
    (
        black_flag_car_num,
        i32,
        50,
        "Black Flag Car #",
        257,
        3,
        R,
        ""
    ),
    (
        red_status,
        bool,
        51,
        "Red Status (' ' or 'R')",
        260,
        1,
        R,
        ""
    ),
    (
        yellow_status,
        bool,
        52,
        "Yellow Status (' ' or 'Y')",
        261,
        1,
        R,
        ""
    ),
    (
        green_status,
        bool,
        53,
        "Green Status (' ' or 'G')",
        262,
        1,
        R,
        ""
    ),
    (
        current_running_time,
        &str,
        54,
        "Current Running Time (hh:mm:ss.dcm)",
        263,
        12,
        R,
        "Photocell Mode Only"
    ),
    (
        best_lap_time,
        &str,
        55,
        "Best Lap Time (hh:mm:ss.dcm)",
        275,
        12,
        R,
        "Photocell Mode Only"
    ),
    (
        position_1_car_num,
        i32,
        56,
        "Position 1 Car #",
        287,
        3,
        R,
        ""
    ),
    (
        position_2_car_num,
        i32,
        57,
        "Position 2 Car #",
        290,
        3,
        R,
        ""
    ),
    (
        position_3_car_num,
        i32,
        58,
        "Position 3 Car #",
        293,
        3,
        R,
        ""
    ),
    (
        position_4_car_num,
        i32,
        59,
        "Position 4 Car #",
        296,
        3,
        R,
        ""
    ),
    (
        position_5_car_num,
        i32,
        60,
        "Position 5 Car #",
        299,
        3,
        R,
        ""
    ),
    (
        position_6_car_num,
        i32,
        61,
        "Position 6 Car #",
        302,
        3,
        R,
        ""
    ),
    (
        position_7_car_num,
        i32,
        62,
        "Position 7 Car #",
        305,
        3,
        R,
        ""
    ),
    (
        position_8_car_num,
        i32,
        63,
        "Position 8 Car #",
        308,
        3,
        R,
        ""
    ),
    (
        position_9_car_num,
        i32,
        64,
        "Position 9 Car #",
        311,
        3,
        R,
        ""
    ),
    (
        position_10_car_num,
        i32,
        65,
        "Position 10 Car #",
        314,
        3,
        R,
        ""
    ),
    (
        position_11_car_num,
        i32,
        66,
        "Position 11 Car #",
        317,
        3,
        R,
        ""
    ),
    (
        position_12_car_num,
        i32,
        67,
        "Position 12 Car #",
        320,
        3,
        R,
        ""
    ),
    (
        position_13_car_num,
        i32,
        68,
        "Position 13 Car #",
        323,
        3,
        R,
        ""
    ),
    (
        position_14_car_num,
        i32,
        69,
        "Position 14 Car #",
        326,
        3,
        R,
        ""
    ),
    (
        position_15_car_num,
        i32,
        70,
        "Position 15 Car #",
        329,
        3,
        R,
        ""
    ),
    (
        position_16_car_num,
        i32,
        71,
        "Position 16 Car #",
        332,
        3,
        R,
        ""
    ),
    (
        position_17_car_num,
        i32,
        72,
        "Position 17 Car #",
        335,
        3,
        R,
        ""
    ),
    (
        position_18_car_num,
        i32,
        73,
        "Position 18 Car #",
        338,
        3,
        R,
        ""
    ),
    (
        position_19_car_num,
        i32,
        74,
        "Position 19 Car #",
        341,
        3,
        R,
        ""
    ),
    (
        position_20_car_num,
        i32,
        75,
        "Position 20 Car #",
        344,
        3,
        R,
        ""
    ),
    (
        position_21_car_num,
        i32,
        76,
        "Position 21 Car #",
        347,
        3,
        R,
        ""
    ),
    (
        position_22_car_num,
        i32,
        77,
        "Position 22 Car #",
        350,
        3,
        R,
        ""
    ),
    (
        position_23_car_num,
        i32,
        78,
        "Position 23 Car #",
        353,
        3,
        R,
        ""
    ),
    (
        position_24_car_num,
        i32,
        79,
        "Position 24 Car #",
        356,
        3,
        R,
        ""
    ),
    (
        position_25_car_num,
        i32,
        80,
        "Position 25 Car #",
        359,
        3,
        R,
        ""
    ),
    (
        position_26_car_num,
        i32,
        81,
        "Position 26 Car #",
        362,
        3,
        R,
        ""
    ),
    (
        position_27_car_num,
        i32,
        82,
        "Position 27 Car #",
        365,
        3,
        R,
        ""
    ),
    (
        position_28_car_num,
        i32,
        83,
        "Position 28 Car #",
        368,
        3,
        R,
        ""
    ),
    (
        position_29_car_num,
        i32,
        84,
        "Position 29 Car #",
        371,
        3,
        R,
        ""
    ),
    (
        position_30_car_num,
        i32,
        85,
        "Position 30 Car #",
        374,
        3,
        R,
        ""
    ),
    (
        position_31_car_num,
        i32,
        86,
        "Position 31 Car #",
        377,
        3,
        R,
        ""
    ),
    (
        position_32_car_num,
        i32,
        87,
        "Position 32 Car #",
        380,
        3,
        R,
        ""
    ),
    (
        position_33_car_num,
        i32,
        88,
        "Position 33 Car #",
        383,
        3,
        R,
        ""
    ),
    (
        position_34_car_num,
        i32,
        89,
        "Position 34 Car #",
        386,
        3,
        R,
        ""
    ),
    (
        position_35_car_num,
        i32,
        90,
        "Position 35 Car #",
        389,
        3,
        R,
        ""
    ),
    (
        position_36_car_num,
        i32,
        91,
        "Position 36 Car #",
        392,
        3,
        R,
        ""
    ),
    (
        position_37_car_num,
        i32,
        92,
        "Position 37 Car #",
        395,
        3,
        R,
        ""
    ),
    (
        position_38_car_num,
        i32,
        93,
        "Position 38 Car #",
        398,
        3,
        R,
        ""
    ),
    (
        position_39_car_num,
        i32,
        94,
        "Position 39 Car #",
        401,
        3,
        R,
        ""
    ),
    (
        position_40_car_num,
        i32,
        95,
        "Position 40 Car #",
        404,
        3,
        R,
        ""
    ),
    (
        position_41_car_num,
        i32,
        96,
        "Position 41 Car #",
        407,
        3,
        R,
        ""
    ),
    (
        position_42_car_num,
        i32,
        97,
        "Position 42 Car #",
        410,
        3,
        R,
        ""
    ),
    (
        position_43_car_num,
        i32,
        98,
        "Position 43 Car #",
        413,
        3,
        R,
        ""
    ),
    (
        position_44_car_num,
        i32,
        99,
        "Position 44 Car #",
        416,
        3,
        R,
        ""
    ),
    (
        position_45_car_num,
        i32,
        100,
        "Position 45 Car #",
        419,
        3,
        R,
        ""
    ),
    (
        position_1_pos_num,
        i32,
        102,
        "Position 1 Pos #",
        467,
        2,
        R,
        ""
    ),
    (
        position_1_lap_num,
        i32,
        103,
        "Position 1 Lap #",
        469,
        5,
        R,
        ""
    ),
    (
        position_1_time,
        &str,
        104,
        "Position 1 Time (hh:mm:ss.dcm)",
        474,
        12,
        R,
        ""
    ),
    (
        position_1_speed,
        &str,
        105,
        "Position 1 Speed (xxx.xxx)",
        486,
        7,
        R,
        ""
    ),
    (
        position_1_driver_name,
        &str,
        106,
        "Position 1 Driver Name",
        493,
        12,
        L,
        ""
    ),
    (
        position_2_pos_num,
        i32,
        107,
        "Position 2 Pos #",
        505,
        2,
        R,
        ""
    ),
    (
        position_2_lap_num,
        i32,
        108,
        "Position 2 Lap #",
        507,
        5,
        R,
        ""
    ),
    (
        position_2_time,
        &str,
        109,
        "Position 2 Time (hh:mm:ss.dcm)",
        512,
        12,
        R,
        ""
    ),
    (
        position_2_speed,
        &str,
        110,
        "Position 2 Speed (xxx.xxx)",
        524,
        7,
        R,
        ""
    ),
    (
        position_2_driver_name,
        &str,
        111,
        "Position 2 Driver Name",
        531,
        12,
        L,
        ""
    ),
    (
        position_3_pos_num,
        i32,
        112,
        "Position 3 Pos #",
        543,
        2,
        R,
        ""
    ),
    (
        position_3_lap_num,
        i32,
        113,
        "Position 3 Lap #",
        545,
        5,
        R,
        ""
    ),
    (
        position_3_time,
        &str,
        114,
        "Position 3 Time (hh:mm:ss.dcm)",
        550,
        12,
        R,
        ""
    ),
    (
        position_3_speed,
        &str,
        115,
        "Position 3 Speed (xxx.xxx)",
        562,
        7,
        R,
        ""
    ),
    (
        position_3_driver_name,
        &str,
        116,
        "Position 3 Driver Name",
        569,
        12,
        L,
        ""
    ),
    (
        position_4_pos_num,
        i32,
        117,
        "Position 4 Pos #",
        581,
        2,
        R,
        ""
    ),
    (
        position_4_lap_num,
        i32,
        118,
        "Position 4 Lap #",
        583,
        5,
        R,
        ""
    ),
    (
        position_4_time,
        &str,
        119,
        "Position 4 Time (hh:mm:ss.dcm)",
        588,
        12,
        R,
        ""
    ),
    (
        position_4_speed,
        &str,
        120,
        "Position 4 Speed (xxx.xxx)",
        600,
        7,
        R,
        ""
    ),
    (
        position_4_driver_name,
        &str,
        121,
        "Position 4 Driver Name",
        607,
        12,
        L,
        ""
    ),
    (
        position_5_pos_num,
        i32,
        122,
        "Position 5 Pos #",
        619,
        2,
        R,
        ""
    ),
    (
        position_5_lap_num,
        i32,
        123,
        "Position 5 Lap #",
        621,
        5,
        R,
        ""
    ),
    (
        position_5_time,
        &str,
        124,
        "Position 5 Time (hh:mm:ss.dcm)",
        626,
        12,
        R,
        ""
    ),
    (
        position_5_speed,
        &str,
        125,
        "Position 5 Speed (xxx.xxx)",
        638,
        7,
        N,
        ""
    ),
    (
        position_5_driver_name,
        &str,
        126,
        "Position 5 Driver Name",
        645,
        12,
        L,
        ""
    ),
    (
        position_6_pos_num,
        i32,
        127,
        "Position 6 Pos #",
        657,
        2,
        R,
        ""
    ),
    (
        position_6_lap_num,
        i32,
        128,
        "Position 6 Lap #",
        659,
        5,
        R,
        ""
    ),
    (
        position_6_time,
        &str,
        129,
        "Position 6 Time (hh:mm:ss.dcm)",
        664,
        12,
        R,
        ""
    ),
    (
        position_6_speed,
        &str,
        130,
        "Position 6 Speed (xxx.xxx)",
        676,
        7,
        R,
        ""
    ),
    (
        position_6_driver_name,
        &str,
        131,
        "Position 6 Driver Name",
        683,
        12,
        L,
        ""
    ),
    (
        position_7_pos_num,
        i32,
        132,
        "Position 7 Pos #",
        695,
        2,
        R,
        ""
    ),
    (
        position_7_lap_num,
        i32,
        133,
        "Position 7 Lap #",
        697,
        5,
        R,
        ""
    ),
    (
        position_7_time,
        &str,
        134,
        "Position 7 Time (hh:mm:ss.dcm)",
        702,
        12,
        R,
        ""
    ),
    (
        position_7_speed,
        &str,
        135,
        "Position 7 Speed (xxx.xxx)",
        714,
        7,
        R,
        ""
    ),
    (
        position_7_driver_name,
        &str,
        136,
        "Position 7 Driver Name",
        721,
        12,
        L,
        ""
    ),
    (
        position_8_pos_num,
        i32,
        137,
        "Position 8 Pos #",
        733,
        2,
        R,
        ""
    ),
    (
        position_8_lap_num,
        i32,
        138,
        "Position 8 Lap #",
        735,
        5,
        R,
        ""
    ),
    (
        position_8_time,
        &str,
        139,
        "Position 8 Time (hh:mm:ss.dcm)",
        740,
        12,
        R,
        ""
    ),
    (
        position_8_speed,
        &str,
        140,
        "Position 8 Speed (xxx.xxx)",
        752,
        7,
        R,
        ""
    ),
    (
        position_8_driver_name,
        &str,
        141,
        "Position 8 Driver Name",
        759,
        12,
        L,
        ""
    ),
    (
        position_9_pos_num,
        i32,
        142,
        "Position 9 Pos #",
        771,
        2,
        R,
        ""
    ),
    (
        position_9_lap_num,
        i32,
        143,
        "Position 9 Lap #",
        773,
        5,
        R,
        ""
    ),
    (
        position_9_time,
        &str,
        144,
        "Position 9 Time (hh:mm:ss.dcm)",
        778,
        12,
        R,
        ""
    ),
    (
        position_9_speed,
        &str,
        145,
        "Position 9 Speed (xxx.xxx)",
        790,
        7,
        R,
        ""
    ),
    (
        position_9_driver_name,
        &str,
        146,
        "Position 9 Driver Name",
        797,
        12,
        L,
        ""
    ),
    (
        position_10_pos_num,
        i32,
        147,
        "Position 10 Pos #",
        809,
        2,
        R,
        ""
    ),
    (
        position_10_lap_num,
        i32,
        148,
        "Position 10 Lap #",
        811,
        5,
        R,
        ""
    ),
    (
        position_10_time,
        &str,
        149,
        "Position 10 Time (hh:mm:ss.dcm)",
        816,
        12,
        R,
        ""
    ),
    (
        position_10_speed,
        &str,
        150,
        "Position 10 Speed (xxx.xxx)",
        828,
        7,
        R,
        ""
    ),
    (
        position_10_driver_name,
        &str,
        151,
        "Position 10 Driver Name",
        835,
        12,
        L,
        ""
    ),
    (
        position_11_pos_num,
        i32,
        152,
        "Position 11 Pos #",
        847,
        2,
        R,
        ""
    ),
    (
        position_11_lap_num,
        i32,
        153,
        "Position 11 Lap #",
        849,
        5,
        R,
        ""
    ),
    (
        position_11_time,
        &str,
        154,
        "Position 11 Time (hh:mm:ss.dcm)",
        854,
        12,
        R,
        ""
    ),
    (
        position_11_speed,
        &str,
        155,
        "Position 11 Speed (xxx.xxx)",
        866,
        7,
        R,
        ""
    ),
    (
        position_11_driver_name,
        &str,
        156,
        "Position 11 Driver Name",
        873,
        12,
        L,
        ""
    ),
    (
        position_12_pos_num,
        i32,
        157,
        "Position 12 Pos #",
        885,
        2,
        R,
        ""
    ),
    (
        position_12_lap_num,
        i32,
        158,
        "Position 12 Lap #",
        887,
        5,
        R,
        ""
    ),
    (
        position_12_time,
        &str,
        159,
        "Position 12 Time (hh:mm:ss.dcm)",
        892,
        12,
        R,
        ""
    ),
    (
        position_12_speed,
        &str,
        160,
        "Position 12 Speed (xxx.xxx)",
        904,
        7,
        R,
        ""
    ),
    (
        position_12_driver_name,
        &str,
        161,
        "Position 12 Driver Name",
        911,
        12,
        L,
        ""
    ),
    (
        variable_num1_pos_num,
        i32,
        162,
        "Variable #1 Pos #",
        923,
        2,
        R,
        ""
    ),
    (
        variable_num1_car_num,
        i32,
        163,
        "Variable #1 Car #",
        925,
        3,
        R,
        ""
    ),
    (
        variable_num2_pos_num,
        i32,
        164,
        "Variable #2 Pos #",
        928,
        2,
        R,
        ""
    ),
    (
        variable_num2_car_num,
        i32,
        165,
        "Variable #2 Car #",
        930,
        3,
        R,
        ""
    ),
    (
        variable_num3_pos_num,
        i32,
        166,
        "Variable #3 Pos #",
        933,
        2,
        R,
        "Future",
        deprecate
    ),
    (
        variable_num3_car_num,
        i32,
        167,
        "Variable #3 Car #",
        935,
        3,
        R,
        "Future",
        deprecate
    ),
    (
        variable_num4_pos_num,
        i32,
        168,
        "Variable #4 Pos #",
        938,
        2,
        R,
        "Future",
        deprecate
    ),
    (
        variable_num4_car_num,
        i32,
        169,
        "Variable #4 Car #",
        940,
        3,
        R,
        "Future",
        deprecate
    ),
    (
        variable_num5_pos_num,
        i32,
        170,
        "Variable #5 Pos #",
        943,
        2,
        R,
        "Future",
        deprecate
    ),
    (
        variable_num5_car_num,
        i32,
        171,
        "Variable #5 Car #",
        945,
        3,
        R,
        "Future",
        deprecate
    )
);
