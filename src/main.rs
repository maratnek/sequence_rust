///
/// considering that input data is correct
/// considering symbol is little sequence
use log::info;

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
                    Sequence::close_bracket(
                        i,
                        '[',
                        &mut stack_br,
                        &mut previous_sequnce,
                        &mut current_sequnce,
                        &mut previous_symbol,
                        &seq_concat,
                    );
                }
                ')' => {
                    Sequence::close_bracket(
                        i,
                        '(',
                        &mut stack_br,
                        &mut previous_sequnce,
                        &mut current_sequnce,
                        &mut previous_symbol,
                        &seq_concat,
                    );
                }
                '}' => {
                    Sequence::close_bracket(
                        i,
                        '{',
                        &mut stack_br,
                        &mut previous_sequnce,
                        &mut current_sequnce,
                        &mut previous_symbol,
                        &seq_concat,
                    );
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

    fn close_bracket(
        i: usize,
        output_char: char,
        stack_br: &mut Vec<SymbolPosition>,
        previous_sequnce: &mut SequenceDest,
        current_sequnce: &mut String,
        previous_symbol: &mut String,
        s_concat_two: &String,
    ) {
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
                    &mut s_concat_two[previous_sequnce.start..(previous_sequnce.end + 1)]
                        .to_string(),
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
fn test_for_handle_correct_seq_with_several_close_br() {
    assert_eq!(correct_sequence("[[]((]){}"), "[]");
}

#[test]
fn test_correct_seq_for_all_kind_of_br() {
    assert_eq!(correct_sequence("[[](){}"), "[](){}");
}

#[test]
fn test_find_correct_seq_for_all_kindof_brackets() {
    assert_eq!(
        correct_sequence("}[bc])k)ab{})c(d)y())da([b]()))kc({z"),
        "kc({z}[bc])k"
    );
}


#[test]
fn test_find_correct_seq_from_several_for_round_and_braces() {
    assert_eq!(
        correct_sequence("}(bc))k)ab{})c(d)y())da((b)()))kc({z"),
        "kc({z}(bc))k"
    );
}

#[test]
fn test_find_correct_seq_from_several_for_round_br() {
    assert_eq!(
        correct_sequence(")(bc))k)ab())c(d)y())da((b)()))kc((z"),
        "kc((z)(bc))k"
    );
}

#[test]
fn test_find_correct_seq_from_several() {
    assert_eq!(
        correct_sequence("}{bc}}k}ab{}}c{d}y{}}da{{b}{}}}kc{{z"),
        "kc{{z}{bc}}k"
    );
}

#[test]
fn test_correct_seq_in_center() {
    assert_eq!(correct_sequence("}}k}ab{c{d}c}da}kc{"), "ab{c{d}c}da");
}

#[test]
fn test_left_correct_seq() {
    assert_eq!(correct_sequence("abc{dda}kc{"), "abc{dda}kc");
}

#[test]
fn test_only_one_open_bracket() {
    assert_eq!(correct_sequence("{"), "");
}

#[test]
fn test_only_one_close_bracket() {
    assert_eq!(correct_sequence("}"), "");
}

#[test]
fn test_seq_reverse() {
    assert_eq!(correct_sequence("}}}bc{{"), "bc{{}}");
}

#[test]
fn test_seq_with_diff_br_long() {
    assert_eq!(correct_sequence("{a{}}}}}{}}"), "{a{}}");
}

#[test]
fn test_seq_with_diff_br() {
    assert_eq!(correct_sequence("{a{}}{}}"), "{a{}}{}");
}


#[test]
fn test_seq_with_left_correct_brackets() {
    assert_eq!(correct_sequence("{}}"), "{}");
}

#[test]
fn test_seq_with_right_correct_brackets() {
    assert_eq!(correct_sequence("{{}"), "{}");
}


#[test]
fn test_seq_with_right_correct() {
    assert_eq!(correct_sequence("a{a}c{"), "a{a}c");
}

#[test]
fn test_symbol_seq() {
    assert_eq!(correct_sequence("bc}ha}kk"), "kkbc");
}

#[test]
fn test_seq_only_symbol() {
    assert_eq!(correct_sequence("}a}"), "a");
}

#[test]
fn test_infinite_with_symbol_seq() {
    assert_eq!(correct_sequence("{a}"), "Infinite");
}

#[test]
fn test_infinite_seq() {
    assert_eq!(correct_sequence("{}"), "Infinite");
}
#[test]
fn test_infinite_with_double_seq() {
    assert_eq!(correct_sequence("{}{}"), "Infinite");
}

#[test]
fn test_infinite_with_several_seq() {
    assert_eq!(correct_sequence("{{}{}}"), "Infinite");
}

#[test]
fn test_infinite_with_reverse_seq() {
    assert_eq!(correct_sequence("}c}bc{k{d"), "Infinite");
}

#[test]
fn test_infinite_symbol_seq() {
    assert_eq!(correct_sequence("zzz"), "Infinite");
}
#[test]
fn test_infinite_seq_for_round_and_braces() {
    assert_eq!(correct_sequence("(){}"), "Infinite");
}

fn main() {
    println!(
        "Sequence {} === kc({{z}}[bc])k",
        correct_sequence("}[bc])k)ab[])c(d)y())da((b)()))kc({z")
    );
}
