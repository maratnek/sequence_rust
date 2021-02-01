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
                            println!("t_pr_seq {}", i);
                        } else {
                            t_pr_seq = (br.0, i);
                        }
                        let t_cur_seq = s_concat_two[t_pr_seq.0..(t_pr_seq.1 + 1)].to_string();
                        if cur_seq.len() < t_cur_seq.len() {
                            cur_seq = t_cur_seq;
                        }
                    } else {
                        //clear all
                        stack_br.clear();
                        if !t_pr_symbol.is_empty() && cur_seq.len() < t_pr_symbol.len() {
                            cur_seq = t_pr_symbol.clone();
                        }
                        t_pr_symbol.clear();
                    }
                } else {
                    //clear all
                    stack_br.clear();
                    if !t_pr_symbol.is_empty() && cur_seq.len() < t_pr_symbol.len() {
                        cur_seq = t_pr_symbol.clone();
                    }
                    t_pr_symbol.clear();
                }
            }
            _ => {
                if t_pr_seq.1 + 1 == i && t_pr_seq.1 != 0 {
                    t_pr_seq.1 = i;
                    println!("t_pr_seq {}", i);
                    cur_seq = s_concat_two[t_pr_seq.0..(t_pr_seq.1 + 1)].to_string();
                    t_pr_symbol.clear();
                } else {
                    t_pr_symbol.push(ch);
                }

                println!("Symbol cur_seq {}", cur_seq);
            }
        }
        if cur_seq.len() == S.len() || t_pr_symbol.len() == S.len() {
            println!(
                "{} : {} cur_seq: {}, pr_seq: {:?}",
                i, ch, cur_seq, t_pr_seq
            );
            return "Infinite".to_string();
        }
        if i >= S.len() && stack_br.is_empty() && t_pr_symbol.is_empty() {
            break;
        }
    }

    let l = t_pr_symbol.len();
    if l != 0 {
        if cur_seq.is_empty() {
            cur_seq = t_pr_symbol.clone();
        }
        println!("t_pr_symbol {}", t_pr_symbol);
    }
    cur_seq
}

#[test]
fn test19_seq() {
    assert_eq!(correct_sequence("}{bc}}k}ab{}}c{d}y{}}da{{b}{}}}kc{{z"), "kc{{z}{bc}}k");
}

#[test]
fn test18_seq() {
    assert_eq!(correct_sequence("}}k}ab{c{d}c}da}kc{"), "ab{c{d}c}da");
}

#[test]
fn test17_seq() {
    assert_eq!(correct_sequence("abc{dda}kc{"), "abc{dda}kc");
}

#[test]
fn test16_seq() {
    assert_eq!(correct_sequence("zzz"), "Infinite");
}
#[test]
fn test15_seq() {
    assert_eq!(correct_sequence("{"), "");
}

#[test]
fn test14_seq() {
    assert_eq!(correct_sequence("}"), "");
}

#[test]
fn test13_seq() {
    assert_eq!(correct_sequence("}c}bc{k{d"), "Infinite");
}


#[test]
fn test12_seq() {
    assert_eq!(correct_sequence("}}}bc{{"), "bc{{}}");
}

#[test]
fn test11_seq() {
    assert_eq!(correct_sequence("{a{}}}}}{}}"), "{a{}}");
}

#[test]
fn test10_seq() {
    assert_eq!(correct_sequence("{a{}}{}}"), "{a{}}{}");
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
    assert_eq!(correct_sequence("bc}ha}kk"), "kkbc");
}
