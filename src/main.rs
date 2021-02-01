///
/// considering that input data is correct
/// considering symbol is little sequence

fn w_concat(S: &str) -> String {
    let mut s_concat_two = S.to_string();
    s_concat_two.push_str(&S);
    s_concat_two
}

fn update_seq(cur_seq: &mut String, n_seq: &mut String) {
    if cur_seq.len() < n_seq.len() {
        *cur_seq = n_seq.clone();
    }
}

fn correct_sequence(S: &str) -> String {
    println!("In: {}", S);
    let mut s_concat_two = w_concat(S);
    let mut cur_seq = String::from("");
    let mut t_pr_symbol = String::from("");
    let mut t_pr_seq: (usize, usize) = (0, 0);
    let mut stack_br: Vec<(usize, char)> = Vec::new();
    for (i, ch) in s_concat_two.chars().enumerate() {
        println!(
            "{} : {} cur_seq: {}, pr_seq: {:?}",
            i, ch, cur_seq, t_pr_seq
        );
        match ch {
            '{' => {
                let l = t_pr_symbol.len();
                t_pr_symbol.clear();
                stack_br.push((i - l, ch));
                println!("push");
            }
            '}' => {
                if !stack_br.is_empty() {
                    let br = stack_br.pop().unwrap();
                    println!("br {:?}", br);
                    if br.1 == '{' {
                        if t_pr_seq.1 + 1 == br.0 && t_pr_seq.1 != 0 {
                            t_pr_seq.1 = i;
                        } else {
                            t_pr_seq = (br.0, i);
                        }
                        cur_seq = s_concat_two[t_pr_seq.0..(t_pr_seq.1 + 1)].to_string();
                    }
                } else {
                    //clear all
                    stack_br.clear();
                }
            }
            _ => {
                println!("Symbol");
                if t_pr_seq.1 + 1 == i && t_pr_seq.1 != 0 {
                    t_pr_seq.1 = i;
                } else {
                    t_pr_symbol.push(ch);
                }
                cur_seq = s_concat_two[t_pr_seq.0..(t_pr_seq.1 + 1)].to_string();
            }
        }
        if cur_seq.len() == S.len() {
            println!(
                "{} : {} cur_seq: {}, pr_seq: {:?}",
                i, ch, cur_seq, t_pr_seq
            );
            return "Infinite".to_string();
        }
    }

    let l = t_pr_symbol.len();
    if l != 0 {}
    cur_seq
}

#[test]
fn test9_seq() {
    assert_eq!(correct_sequence("{{}{}}"), "Infinite");
}

#[test]
fn test8_seq() {
    assert_eq!(correct_sequence("{}{}"), "Infinite");
}

#[test]
fn test7_seq() {
    assert_eq!(correct_sequence("{}}"), "{}");
}

#[test]
fn test6_seq() {
    assert_eq!(correct_sequence("{{}"), "{}");
}

#[test]
fn test5_seq() {
    assert_eq!(correct_sequence("{}"), "Infinite");
}

#[test]
fn test4_seq() {
    assert_eq!(correct_sequence("a{a}c{"), "a{a}c");
}

fn main() {
    let mut cur_seq = String::from("cur seq");
    let mut n_seq = String::from("more seq");
    update_seq(&mut cur_seq, &mut n_seq);
    println!("Current seq: {}", cur_seq);
}

#[test]
fn test1_concat() {
    let input = "{}";
    let output = "{}{}";
    assert_eq!(w_concat(input), output);
    let input = "{}";
    let output = "{}{";
    assert_ne!(w_concat(input), output);
}

#[test]
fn test1_seq() {
    assert_eq!(correct_sequence("{a}"), "Infinite");
}

#[test]
fn test2_seq() {
    assert_eq!(correct_sequence("}a}"), "a");
}

#[test]
fn test3_seq() {
    assert_eq!(correct_sequence("bc}ha}kk"), "bc");
}
