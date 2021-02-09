///
/// considering that input data is correct
/// considering symbol is little sequence
use log::{info};

fn concat_sequence(s: &str) -> String {
    let mut s_concat_two = s.to_string();
    s_concat_two.push_str(&s);
    s_concat_two
}

#[derive(Copy, Clone)]
struct SymbolPosition {
    pos: usize,
    ch: char,
}

#[derive(Debug)]
struct SequenceDest {
    start: usize,
    end: usize,
}


struct Sequence {
    s: String,
}


impl Sequence {
    fn new(s: String) -> Sequence {
        Self { s }
    }

    fn get_correct_sequence(&self) -> String {
        let seq_concat = concat_sequence(&self.s);
        let mut current_sequnce = String::from("");
        let mut previous_symbol = String::from("");
        let mut previous_sequnce = SequenceDest { start: 0, end: 0 };
        let mut stack_br: Vec<SymbolPosition> = Vec::new();

        for (i, ch) in seq_concat.chars().enumerate() {

            match ch {
                '(' | '{' | '[' => {
                    stack_br.push(SymbolPosition {
                        pos: i - previous_symbol.len(),
                        ch: ch,
                    });
                    previous_symbol.clear();
                }
                ']' => {
                    Sequence::close_bracket(i, '[', &mut stack_br, &mut previous_sequnce, &mut current_sequnce, &mut previous_symbol, &s_concat_two);
                }
                ')' => {
                    Sequence::close_bracket(i, '(', &mut stack_br, &mut previous_sequnce, &mut current_sequnce, &mut previous_symbol, &s_concat_two);
                }
                '}' => {
                    Sequence::close_bracket(i, '{', &mut stack_br, &mut previous_sequnce, &mut current_sequnce, &mut previous_symbol, &s_concat_two);
                }
                _ => {
                    if previous_sequnce.end + 1 == i && previous_sequnce.end != 0 {
                        info!("t_pr_seq {}", i);
                        previous_sequnce.end = i;
                        current_sequnce = seq_concat 
                            [previous_sequnce.start..(previous_sequnce.end + 1)]
                            .to_string();
                        previous_symbol.clear();
                    } else {
                        previous_symbol.push(ch);
                    }
                }
            }
            // return if found max possible sequence
            if current_sequnce.len() == self.s.len() || previous_symbol.len() == self.s.len() {
                return "Infinite".to_string();
            }
            if i >= self.s.len() && stack_br.is_empty() && previous_symbol.is_empty() {
                break;
            }
        }

        // symbol sequence if sequence not found
        if previous_symbol.len() != 0 && current_sequnce.is_empty() {
            current_sequnce = previous_symbol;
        }
        current_sequnce
    }

fn close_bracket(i:usize, output_char:char, 
        stack_br: &mut Vec<SymbolPosition>, previous_sequnce: &mut SequenceDest, current_sequnce: &mut String,
        previous_symbol: &mut String, s_concat_two: &String)
{
        let optional = stack_br.pop();
        if !optional.is_none() && optional.unwrap().ch == output_char {
            let br = optional.unwrap();
            if previous_sequnce.end + 1 == br.pos && previous_sequnce.end != 0 {
                previous_sequnce.end = i;
            } else {
                previous_sequnce.start = br.pos;
                previous_sequnce.end = i;
            }
            let previous_seq_len = previous_sequnce.end + 1 - previous_sequnce.start;
            if current_sequnce.len() < previous_seq_len {
                current_sequnce.clear();
                current_sequnce.push_str(
                    &mut s_concat_two[previous_sequnce.start..(previous_sequnce.end + 1)].to_string()
                );
            }
        } else {
            //clear all
            stack_br.clear();
            if !previous_symbol.is_empty() && current_sequnce.len() < previous_symbol.len() {
                current_sequnce.clear();
                current_sequnce.push_str(previous_symbol);
            }
            previous_symbol.clear();
        }
}
}

fn correct_sequence(s: &str) -> String {
    info!("In: {}", s);
    let seq = Sequence::new(s.to_string());
    seq.get_correct_sequence()
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
    println!(
        "Sequence {} === kc({{z}}[bc])k",
        correct_sequence("}[bc])k)ab[])c(d)y())da((b)()))kc({z")
    );
}
