//! The struct for viewing Track data streams
//!
//! This sport was generated semi-automatically and may contain errors.
super::sport_builder!(
    Track,
    "Track",
    true,
    (running_time, &str, 1, "Running Time", 1, 9, L, ""),
    (cumulative_split, &str, 2, "Cumulative Split", 10, 9, R, ""),
    (
        subtractive_split,
        &str,
        3,
        "Subtractive Split",
        19,
        9,
        R,
        ""
    ),
    (time_of_day, &str, 5, "Time of Day (hh:mm:ss)", 40, 8, L, ""),
    (home_team_name, &str, 6, "Home Team Name", 48, 20, N, ""),
    (guest_team_name, &str, 7, "Guest Team Name", 68, 20, L, ""),
    (event_number, i32, 9, "Event Number", 118, 3, R, ""),
    (
        event_number_alpha,
        i32,
        10,
        "Event Number - Alpha",
        121,
        1,
        L,
        ""
    ),
    (heat_number, i32, 11, "Heat Number", 122, 2, R, ""),
    (
        heat_number_alpha,
        i32,
        12,
        "Heat Number - Alpha",
        124,
        20,
        L,
        ""
    ),
    (round_number, i32, 13, "Round Number", 144, 1, L, ""),
    (splits_completed, i32, 15, "Splits Completed", 147, 2, R, ""),
    (record_1_name, &str, 16, "Record 1 Name", 149, 12, L, ""),
    (record_1_code, &str, 17, "Record 1 Code", 161, 2, L, ""),
    (record_1_time, &str, 18, "Record 1 Time", 163, 9, L, ""),
    (record_2_name, &str, 19, "Record 2 Name", 172, 12, L, ""),
    (record_2_code, &str, 20, "Record 2 Code", 184, 2, L, ""),
    (record_2_time, &str, 21, "Record 2 Time", 186, 9, L, ""),
    (record_3_name, &str, 22, "Record 3 Name", 195, 12, L, ""),
    (record_3_code, &str, 23, "Record 3 Code", 207, 2, L, ""),
    (record_3_time, &str, 24, "Record 3 Time", 209, 9, L, ""),
    (record_4_name, &str, 25, "Record 4 Name", 218, 12, L, ""),
    (record_4_code, &str, 26, "Record 4 Code", 230, 2, L, ""),
    (record_4_time, &str, 27, "Record 4 Time", 232, 9, L, ""),
    (
        line_1_runner_name,
        &str,
        28,
        "Line 1 Runner Name",
        241,
        15,
        L,
        ""
    ),
    (
        line_1_team_name,
        &str,
        29,
        "Line 1 Team Name",
        256,
        5,
        L,
        ""
    ),
    (
        line_1_lane_number,
        i32,
        30,
        "Line 1 Lane Number",
        261,
        2,
        R,
        ""
    ),
    (
        line_1_place_number,
        i32,
        31,
        "Line 1 Place Number",
        263,
        3,
        R,
        ""
    ),
    (
        line_1_split_finish_time,
        &str,
        32,
        "Line 1 Split/Finish Time",
        266,
        9,
        L,
        ""
    ),
    (
        line_1_splits_completed,
        i32,
        33,
        "Line 1 Splits Completed",
        275,
        2,
        R,
        ""
    ),
    (
        line_2_runner_name,
        &str,
        34,
        "Line 2 Runner Name",
        277,
        15,
        L,
        ""
    ),
    (
        line_2_team_name,
        &str,
        35,
        "Line 2 Team Name",
        292,
        5,
        L,
        ""
    ),
    (
        line_2_lane_number,
        i32,
        36,
        "Line 2 Lane Number",
        297,
        2,
        R,
        ""
    ),
    (
        line_2_place_number,
        i32,
        37,
        "Line 2 Place Number",
        299,
        3,
        R,
        ""
    ),
    (
        line_2_split_finish_time,
        &str,
        38,
        "Line 2 Split/Finish Time",
        302,
        9,
        L,
        ""
    ),
    (
        line_2_splits_completed,
        i32,
        39,
        "Line 2 Splits Completed",
        311,
        2,
        R,
        ""
    ),
    (
        line_3_runner_name,
        &str,
        40,
        "Line 3 Runner Name",
        313,
        15,
        L,
        ""
    ),
    (
        line_3_team_name,
        &str,
        41,
        "Line 3 Team Name",
        328,
        5,
        L,
        ""
    ),
    (
        line_3_lane_number,
        i32,
        42,
        "Line 3 Lane Number",
        333,
        2,
        R,
        ""
    ),
    (
        line_3_place_number,
        i32,
        43,
        "Line 3 Place Number",
        335,
        3,
        R,
        ""
    ),
    (
        line_3_split_finish_time,
        &str,
        44,
        "Line 3 Split/Finish Time",
        338,
        9,
        L,
        ""
    ),
    (
        line_3_splits_completed,
        i32,
        45,
        "Line 3 Splits Completed",
        347,
        2,
        R,
        ""
    ),
    (
        line_4_runner_name,
        &str,
        46,
        "Line 4 Runner Name",
        349,
        15,
        L,
        ""
    ),
    (
        line_4_team_name,
        &str,
        47,
        "Line 4 Team Name",
        364,
        5,
        L,
        ""
    ),
    (
        line_4_lane_number,
        i32,
        48,
        "Line 4 Lane Number",
        369,
        2,
        R,
        ""
    ),
    (
        line_4_place_number,
        i32,
        49,
        "Line 4 Place Number",
        371,
        3,
        R,
        ""
    ),
    (
        line_4_split_finish_time,
        &str,
        50,
        "Line 4 Split/Finish Time",
        374,
        9,
        L,
        ""
    ),
    (
        line_4_splits_completed,
        i32,
        51,
        "Line 4 Splits Completed",
        383,
        2,
        R,
        ""
    ),
    (
        line_5_runner_name,
        &str,
        52,
        "Line 5 Runner Name",
        385,
        15,
        L,
        ""
    ),
    (
        line_5_team_name,
        &str,
        53,
        "Line 5 Team Name",
        400,
        5,
        L,
        ""
    ),
    (
        line_5_lane_number,
        i32,
        54,
        "Line 5 Lane Number",
        405,
        2,
        R,
        ""
    ),
    (
        line_5_place_number,
        i32,
        55,
        "Line 5 Place Number",
        407,
        3,
        R,
        ""
    ),
    (
        line_5_split_finish_time,
        &str,
        56,
        "Line 5 Split/Finish Time",
        410,
        9,
        L,
        ""
    ),
    (
        line_5_splits_completed,
        i32,
        57,
        "Line 5 Splits Completed",
        419,
        2,
        R,
        ""
    ),
    (
        line_6_runner_name,
        &str,
        58,
        "Line 6 Runner Name",
        421,
        15,
        L,
        ""
    ),
    (
        line_6_team_name,
        &str,
        59,
        "Line 6 Team Name",
        436,
        5,
        L,
        ""
    ),
    (
        line_6_lane_number,
        i32,
        60,
        "Line 6 Lane Number",
        441,
        2,
        R,
        ""
    ),
    (
        line_6_place_number,
        i32,
        61,
        "Line 6 Place Number",
        443,
        3,
        R,
        ""
    ),
    (
        line_6_split_finish_time,
        &str,
        62,
        "Line 6 Split/Finish Time",
        446,
        9,
        L,
        ""
    ),
    (
        line_6_splits_completed,
        i32,
        63,
        "Line 6 Splits Completed",
        455,
        2,
        R,
        ""
    ),
    (
        line_7_runner_name,
        &str,
        64,
        "Line 7 Runner Name",
        457,
        15,
        L,
        ""
    ),
    (
        line_7_team_name,
        &str,
        65,
        "Line 7 Team Name",
        472,
        5,
        L,
        ""
    ),
    (
        line_7_lane_number,
        i32,
        66,
        "Line 7 Lane Number",
        477,
        2,
        R,
        ""
    ),
    (
        line_7_place_number,
        i32,
        67,
        "Line 7 Place Number",
        479,
        3,
        R,
        ""
    ),
    (
        line_7_split_finish_time,
        &str,
        68,
        "Line 7 Split/Finish Time",
        482,
        9,
        L,
        ""
    ),
    (
        line_7_splits_completed,
        i32,
        69,
        "Line 7 Splits Completed",
        491,
        2,
        R,
        ""
    ),
    (
        line_8_runner_name,
        &str,
        70,
        "Line 8 Runner Name",
        493,
        15,
        L,
        ""
    ),
    (
        line_8_team_name,
        &str,
        71,
        "Line 8 Team Name",
        508,
        5,
        L,
        ""
    ),
    (
        line_8_lane_number,
        i32,
        72,
        "Line 8 Lane Number",
        513,
        2,
        R,
        ""
    ),
    (
        line_8_place_number,
        i32,
        73,
        "Line 8 Place Number",
        515,
        3,
        R,
        ""
    ),
    (
        line_8_split_finish_time,
        &str,
        74,
        "Line 8 Split/Finish Time",
        518,
        9,
        L,
        ""
    ),
    (
        line_8_splits_completed,
        i32,
        75,
        "Line 8 Splits Completed",
        527,
        2,
        R,
        ""
    ),
    (
        line_9_runner_name,
        &str,
        76,
        "Line 9 Runner Name",
        529,
        15,
        L,
        ""
    ),
    (
        line_9_team_name,
        &str,
        77,
        "Line 9 Team Name",
        544,
        5,
        L,
        ""
    ),
    (
        line_9_lane_number,
        i32,
        78,
        "Line 9 Lane Number",
        549,
        2,
        R,
        ""
    ),
    (
        line_9_place_number,
        i32,
        79,
        "Line 9 Place Number",
        551,
        3,
        R,
        ""
    ),
    (
        line_9_split_finish_time,
        &str,
        80,
        "Line 9 Split/Finish Time",
        554,
        9,
        L,
        ""
    ),
    (
        line_9_splits_completed,
        i32,
        81,
        "Line 9 Splits Completed",
        563,
        2,
        R,
        ""
    ),
    (
        line_10_runner_name,
        &str,
        82,
        "Line 10 Runner Name",
        565,
        15,
        L,
        ""
    ),
    (
        line_10_team_name,
        &str,
        83,
        "Line 10 Team Name",
        580,
        5,
        L,
        ""
    ),
    (
        line_10_lane_number,
        i32,
        84,
        "Line 10 Lane Number",
        585,
        2,
        R,
        ""
    ),
    (
        line_10_place_number,
        i32,
        85,
        "Line 10 Place Number",
        587,
        3,
        R,
        ""
    ),
    (
        line_10_split_finish_time,
        &str,
        86,
        "Line 10 Split/Finish Time",
        590,
        9,
        L,
        ""
    ),
    (
        line_10_splits_completed,
        i32,
        87,
        "Line 10 Splits Completed",
        599,
        2,
        R,
        ""
    ),
    (home_score, i32, 89, "Home Score", 965, 3, R, ""),
    (guest_score, i32, 90, "Guest Score", 968, 3, R, ""),
    (
        single_line_lane_number,
        i32,
        91,
        "Single Line Lane Number",
        971,
        2,
        R,
        ""
    ),
    (
        single_line_place_number,
        i32,
        92,
        "Single Line Place Number",
        973,
        3,
        R,
        ""
    ),
    (
        single_line_split_finish_time,
        &str,
        93,
        "Single Line Split/Finish Time",
        976,
        9,
        L,
        ""
    ),
    (
        event_guest_2_score,
        i32,
        94,
        "Event/Guest 2 score",
        985,
        3,
        R,
        ""
    ),
    (
        heat_guest_3_score,
        i32,
        95,
        "Heat/Guest 3 score",
        988,
        3,
        L,
        ""
    )
);
