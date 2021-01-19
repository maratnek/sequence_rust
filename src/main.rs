fn correct_sequence(S: &str) -> String {
    let mut s_concat_two = S.to_string();
    // s_concat_two.push_str(S);
    // println!("Concat {}", s_concat_two);

    let mut out_string = String::from("");
    let mut cur_seq = String::from("");
    let mut v_sub_seq = vec![0; S.len()];
    if (S.is_empty()) {
        return "empty".to_string();
    }
    let mut current_ind = 0;
    let mut start_ind = 0;
    let mut end_ind = 0;
    let mut stack_bracket: Vec<(usize, char)> = Vec::new();

    for (i, ch) in s_concat_two.chars().enumerate() {
        println!("Current sequence: {} ", cur_seq );
        print!("{} - {}", i, ch);
        if (ch.is_alphabetic()) {
            println!("Char is alpahabetic");
            cur_seq.push(ch);
            continue;
        }
        match ch {
            '[' => {
                // stack_bracket.push((current_ind, ch));
                stack_bracket.push((i, ch));
                // current_ind = i + 1;
                cur_seq.push(ch);
            }
            ']' => {
                if (!stack_bracket.is_empty()) {
                    let brack = stack_bracket.pop().unwrap();
                    if (brack.1 == '[') {
                        cur_seq = s_concat_two[brack.0..(i + 1)].to_string();
                        println!("Current sequence {}", cur_seq);
                    } else {
                        current_ind = i + 1;
                        println!("Incorrect sequence {}", ch);
                        cur_seq.clear();
                        stack_bracket.clear();
                    }
                } else {
                    current_ind = i + 1;
                    println!("Incorrect sequence {}", ch);
                    cur_seq.clear();
                    stack_bracket.clear();
                }
            }
            '{' => {
                // stack_bracket.push((current_ind, ch));
                stack_bracket.push((i, ch));
                // current_ind = i + 1;
                cur_seq.push(ch);
            }
            '}' => {
                if (!stack_bracket.is_empty()) {
                    let brack = stack_bracket.pop().unwrap();
                    if (brack.1 == '{') {
                        // cur_seq = s_concat_two[brack.0..(i + 1)].to_string();
                        cur_seq = s_concat_two[current_ind..(i + 1)].to_string();
                        println!("Current sequence {}", cur_seq);
                    } else {
                        current_ind = i + 1;
                        println!("Incorrect sequence {}", ch);
                        cur_seq.clear();
                        stack_bracket.clear();
                    }
                } else {
                    current_ind = i + 1;
                    println!("Incorrect sequence {}", ch);
                    cur_seq.clear();
                    stack_bracket.clear();
                }
            }
            '(' => {
                // stack_bracket.push((current_ind, ch));
                stack_bracket.push((i, ch));
                // current_ind = i + 1;
                cur_seq.push(ch);
            }
            ')' => {
                if (!stack_bracket.is_empty()) {
                    let brack = stack_bracket.pop().unwrap();
                    if (brack.1 == '(') {
                        // cur_seq = s_concat_two[brack.0..(i + 1)].to_string();
                        cur_seq = s_concat_two[current_ind..(i + 1)].to_string();
                        println!("Current sequence {}", cur_seq);
                    } else {
                        current_ind = i + 1;
                        println!("Incorrect sequence {}", ch);
                        cur_seq.clear();
                        stack_bracket.clear();
                    }
                } else {
                    println!("Incorrect sequence {}", ch);
                    current_ind = i + 1;
                    cur_seq.clear();
                    stack_bracket.clear();
                }
            }
            _ => println!("Not known symbol"),
        }
    }
    // try closure if it possible
    if !stack_bracket.is_empty() {
        println!("stack_bracket is not empty");
        println!("Current index {}", current_ind);
        current_ind = 0;

        let mut cur_seq2 = String::from("");
        for (i, ch) in s_concat_two.chars().enumerate() {
            print!("{} - {}", i, ch);
                            println!("Current sequence {} ", cur_seq );
            if (!stack_bracket.is_empty() && stack_bracket[0].0 == i) {
                break;
            }
            if (ch.is_alphabetic()) {
                println!("Char is alpahabetic");
                cur_seq2.push(ch);
                continue;
            }
            match ch {
                '[' => {
                    stack_bracket.push((current_ind, ch));
                    current_ind = i + 1;
                    // cur_seq.push(ch);
                }
                ']' => {
                    if (!stack_bracket.is_empty()) {
                        let brack = stack_bracket.pop().unwrap();
                        if (brack.1 == '[') {
                            cur_seq = s_concat_two[brack.0..(s_concat_two.len())].to_string();
                            cur_seq.push_str(&s_concat_two[0..(i + 1)]);
                            println!("Current sequence {}", cur_seq);
                        } else {
                            break;
                            current_ind = i + 1;
                            println!("Incorrect sequence {}", ch);
                            cur_seq.clear();
                            stack_bracket.clear();
                        }
                    } else {
                        break;
                    }
                }
                '{' => {
                    stack_bracket.push((i, ch));
                    // current_ind = i + 1;
                    // cur_seq.push(ch);
                }
                '}' => {
                    if (!stack_bracket.is_empty()) {
                        let brack = stack_bracket.pop().unwrap();
                        if (brack.1 == '{') {
                            println!("Current sequence {} {}", cur_seq, brack.0);
                            // cur_seq = s_concat_two[brack.0..(s_concat_two.len())].to_string();
                            // cur_seq.push_str(&s_concat_two[0..(i + 1)]);
                            cur_seq2 = s_concat_two[0..(i + 1)].to_string();
                            // cur_seq.push(ch);
                            println!("Current sequence {}", cur_seq);
                        } else {
                            break;
                        }
                    }
                }
                '(' => {
                    stack_bracket.push((i, ch));
                    // current_ind = i + 1;
                    // cur_seq.push(ch);
                }
                ')' => {
                    if !stack_bracket.is_empty() {
                        let brack = stack_bracket.pop().unwrap();
                        if brack.1 == '(' {
                            // cur_seq = s_concat_two[brack.0..(s_concat_two.len())].to_string();
                            // cur_seq2.push_str(&s_concat_two[0..(i + 1)]);
                            cur_seq2 = s_concat_two[0..(i + 1)].to_string();
                            println!("Current sequence {}", cur_seq);
                        } else {
                            break;
                        }
                    }
                }
                _ => println!("Not known symbol"),
            }
        }

        cur_seq.push_str(&cur_seq2);
    }
    println!("Correct");
    out_string = cur_seq;
    println!("Output string {}", out_string);
    if (out_string.len() == S.len()) {
        "Infinite".to_string()
    } else {
        out_string
    }
}

#[test]
fn test_10() {
    let input = "aa{a(abc)cr";
    let output = "a(abc)cr";
    assert_eq!(correct_sequence(input), output);
}

#[test]
fn test_11() {
    let input = "a})({";
    let output = "Infinite";
    assert_eq!(correct_sequence(input), output);
}

#[test]
fn test_111() {
    let input = "a}){({";
    let output = "({a})";
    assert_eq!(correct_sequence(input), output);
}

#[test]
fn test_112() {
    let input = "{a}bc({c})}{";
    let output = "Infinite";
    assert_eq!(correct_sequence(input), output);
}

#[test]
fn test_113() {
    let input = "{a}";
    let output = "Infinite";
    assert_eq!(correct_sequence(input), output);
}

#[test]
fn test_114() {
    let input = ")(";
    let output = "Infinite";
    assert_eq!(correct_sequence(input), output);
}

#[test]
fn test_115() {
    let input = "{a}b(c({c})}{";
    let output = "c({c})";
    assert_eq!(correct_sequence(input), output);
}

#[test]
fn test_stack() {
    let mut stack: Vec<char> = Vec::new();
    stack.push('[');
    stack.push('(');
    stack.push('{');
    stack.pop();
    if (!stack.is_empty()) {
        let a = stack.pop().unwrap();
        for elem in stack {
            println!("Stack pop {:?}", elem);
        }
    }
}

// if exist pair for one sequence it's infinite
// if not exist pair for at least one ''symbol find sequence
#[test]
fn test_12() {
    let input = "a}{(})[][[";
    let output = "[]";
    assert_eq!(correct_sequence(input), output);
}

fn main() {
    println!("Data: {}", correct_sequence("abc{}"));
}

#[test]
fn test_empty() {
    let input = "";
    let output = "empty";
    assert_eq!(correct_sequence(input), output);
}

/// first test
#[test]
fn test1() {
    let input = "a}](){d}(){";
    let output = "(){d}(){a}";
    assert_eq!(correct_sequence(input), output);
}

#[test]
fn test2() {
    let input = "sh(dh)}";
    let output = "sh(dh)";
    assert_eq!(correct_sequence(input), output);
}

#[test]
fn test3() {
    let input = "]h({hdb}b)[";
    let output = "Infinite";
    assert_eq!(correct_sequence(input), output);
}
