use super::*;

//Tests grabbing the rulebook.
#[test]
fn test_rule_book() {
    let filename = "day21_2_test_input.txt";
    let contents = read_file_to_string(filename.to_string());
    let rulebook = get_rulebook(&contents);

    let test_input = vec![vec![".", "."], vec![".", "#"]];

    let test_output = vec![
        vec!["#", "#", "."],
        vec!["#", ".", "."],
        vec![".", ".", "."],
    ];
    assert_eq!(rulebook[0].input, test_input);
    assert_eq!(rulebook[0].output, test_output);
}

//Tests the transpose function.
#[test]
fn test_transpose() {
    let test_transpose = transpose(&vec![
        vec![".".to_string(), "#".to_string(), ".".to_string()],
        vec![".".to_string(), ".".to_string(), "#".to_string()],
        vec!["#".to_string(), "#".to_string(), "#".to_string()],
    ]);

    let result1 = vec![
        vec![".".to_string(), ".".to_string(), "#".to_string()],
        vec!["#".to_string(), ".".to_string(), "#".to_string()],
        vec![".".to_string(), "#".to_string(), "#".to_string()],
    ];

    assert_eq!(test_transpose, result1);
}

//Tests the flip function.
#[test]
fn test_flip() {
    let test_flip = flip(&vec![
        vec![".".to_string(), ".".to_string()],
        vec![".".to_string(), "#".to_string()],
    ]);
    let result = vec![
        vec![".".to_string(), ".".to_string()],
        vec!["#".to_string(), ".".to_string()],
    ];
    assert_eq!(result, test_flip);
}

//Tests the rotation functionality with transpose and flip.
#[test]
fn test_rotate() {
    let mut test_rotate = Vec::new();
    let mut test_rotate_list = rotate(&vec![
        vec![".".to_string(), "#".to_string(), ".".to_string()],
        vec![".".to_string(), ".".to_string(), "#".to_string()],
        vec!["#".to_string(), "#".to_string(), "#".to_string()],
    ]);

    let result1 = vec![
        vec!["#".to_string(), ".".to_string(), ".".to_string()],
        vec!["#".to_string(), ".".to_string(), "#".to_string()],
        vec!["#".to_string(), "#".to_string(), ".".to_string()],
    ];

    let result2 = vec![
        vec!["#".to_string(), "#".to_string(), "#".to_string()],
        vec!["#".to_string(), ".".to_string(), ".".to_string()],
        vec![".".to_string(), "#".to_string(), ".".to_string()],
    ];

    let result3 = vec![
        vec![".".to_string(), "#".to_string(), "#".to_string()],
        vec!["#".to_string(), ".".to_string(), "#".to_string()],
        vec![".".to_string(), ".".to_string(), "#".to_string()],
    ];

    let result4 = vec![
        vec![".".to_string(), "#".to_string(), ".".to_string()],
        vec![".".to_string(), ".".to_string(), "#".to_string()],
        vec!["#".to_string(), "#".to_string(), "#".to_string()],
    ];

    for _i in 0..4 {
        test_rotate.push(test_rotate_list.clone());
        test_rotate_list = rotate(&test_rotate_list);
    }

    assert_eq!(test_rotate[0], result1);
    assert_eq!(test_rotate[1], result2);
    assert_eq!(test_rotate[2], result3);
    assert_eq!(test_rotate[3], result4);
}

//Tests the find rule function.
#[test]
fn test_find_rule() {
    let filename = "day21_2_test_input.txt";
    let contents = read_file_to_string(filename.to_string());
    let rulebook = get_rulebook(&contents);
    let test_pattern = vec![
        vec![".".to_string(), "#".to_string(), ".".to_string()],
        vec![".".to_string(), ".".to_string(), "#".to_string()],
        vec!["#".to_string(), "#".to_string(), "#".to_string()],
    ];

    let new_pattern = find_match(&test_pattern, &rulebook);

    let result_rule = vec![
        vec![
            "#".to_string(),
            ".".to_string(),
            ".".to_string(),
            "#".to_string(),
        ],
        vec![
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
        ],
        vec![
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
        ],
        vec![
            "#".to_string(),
            ".".to_string(),
            ".".to_string(),
            "#".to_string(),
        ],
    ];

    assert_eq!(new_pattern.unwrap(), result_rule);
}

//Tests the fractal art number of pixels.
#[test]
fn test_fractal_art() {
    let filename = "day21_2_test_input.txt";
    let contents = read_file_to_string(filename.to_string());
    let starting_pattern = vec![
        vec![".".to_string(), "#".to_string(), ".".to_string()],
        vec![".".to_string(), ".".to_string(), "#".to_string()],
        vec!["#".to_string(), "#".to_string(), "#".to_string()],
    ];
    let rulebook = get_rulebook(&contents);
    let num_pixels_on = get_num_pixels_on(&starting_pattern, &rulebook, 2);
    let result_num_on_pixels = 12;
    assert_eq!(num_pixels_on, result_num_on_pixels);
}
