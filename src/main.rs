///
/// considering that input data is correct
/// considering symbol is little sequence
use log::{info, debug};

struct Sequence {
    s: String
}

fn w_concat(s: &str) -> String {
    let mut s_concat_two = s.to_string();
    s_concat_two.push_str(&s);
    s_concat_two
}

#[derive(Copy, Clone)]
struct SymbolPosition {
    pos: usize,
    ch: char
}

#[derive(Debug)]
struct SequenceDest {
    start: usize,
    end: usize,
}

fn correct_sequence(s: &str) -> String {
    info!("In: {}", s);
    let s_concat_two = w_concat(s);
    let mut cur_seq = String::from("");
    let mut t_pr_symbol = String::from("");
    let mut t_pr_seq = SequenceDest{start: 0, end: 0};
    let mut stack_br: Vec<SymbolPosition> = Vec::new();

    for (i, ch) in s_concat_two.chars().enumerate() {
        info!(
            "{} : {} cur_seq: {}, pr_seq: {:?}",
            i, ch, cur_seq, t_pr_seq
        );

        let mut m_lamda_close = |i: usize, output_char: char| {
                let optional = stack_br.pop();
                if !optional.is_none() && optional.unwrap().ch == output_char {
                    let br = optional.unwrap();
                    if t_pr_seq.end + 1 == br.pos && t_pr_seq.end != 0 {
                        t_pr_seq.end = i;
                    } else {
                        t_pr_seq = SequenceDest{start: br.pos, end: i};
                    }
                    let t_cur_seq = s_concat_two[t_pr_seq.start..(t_pr_seq.end + 1)].to_string();
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
        };

        match ch {
            '(' | '{' | '[' => {
                stack_br.push(SymbolPosition{
                    pos: i - t_pr_symbol.len(), 
                    ch: ch});
                t_pr_symbol.clear();
            }
            ']' => {
                m_lamda_close(i, '[');
            }
            ')' => {
                m_lamda_close(i, '(');
            }
            '}' => {
                m_lamda_close(i, '{');
            }
            _ => {
                if t_pr_seq.end + 1 == i && t_pr_seq.end != 0 {
                    info!("t_pr_seq {}", i);
                    t_pr_seq.end = i;
                    cur_seq = s_concat_two[t_pr_seq.start..(t_pr_seq.end + 1)].to_string();
                    t_pr_symbol.clear();
                } else {
                    t_pr_symbol.push(ch);
                }

                info!("Symbol cur_seq {}", cur_seq);
            }
        }
        if cur_seq.len() == s.len() || t_pr_symbol.len() == s.len() {
            return "Infinite".to_string();
        }
        if i >= s.len() && stack_br.is_empty() && t_pr_symbol.is_empty() {
            break;
        }
    }

    if t_pr_symbol.len() != 0 {
        if cur_seq.is_empty() {
            cur_seq = t_pr_symbol;
        }
    }
    cur_seq
}

#[test]
fn test25_seq() {
    assert_eq!(correct_sequence("[[]((]){}"), "[]");
}


#[test]
fn test24_seq() {
    assert_eq!(correct_sequence("[[](){}"), "[](){}");
}

#[test]
fn test23_seq() {
    assert_eq!(
        correct_sequence("}[bc])k)ab{})c(d)y())da([b]()))kc({z"),
        "kc({z}[bc])k"
    );
}

#[test]
fn test22_seq() {
    assert_eq!(correct_sequence("(){}"), "Infinite");
}

#[test]
fn test21_seq() {
    assert_eq!(
        correct_sequence("}(bc))k)ab{})c(d)y())da((b)()))kc({z"),
        "kc({z}(bc))k"
    );
}

#[test]
fn test20_seq() {
    assert_eq!(
        correct_sequence(")(bc))k)ab())c(d)y())da((b)()))kc((z"),
        "kc((z)(bc))k"
    );
}

#[test]
fn test19_seq() {
    assert_eq!(
        correct_sequence("}{bc}}k}ab{}}c{d}y{}}da{{b}{}}}kc{{z"),
        "kc{{z}{bc}}k"
    );
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

#[test]
fn test3_seq() {
    assert_eq!(correct_sequence("bc}ha}kk"), "kkbc");
}

#[test]
fn test2_seq() {
    assert_eq!(correct_sequence("}a}"), "a");
}

#[test]
fn test1_seq() {
    assert_eq!(correct_sequence("{a}"), "Infinite");
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

fn main() {
    println!("Sequence {} === kc({{z}}[bc])k",correct_sequence("}[bc])k)ab[])c(d)y())da((b)()))kc({z"));
}
