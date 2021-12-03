use crate::utils::read_input;
use std::vec;
use std::ops::Add;
use std::cmp::max;

pub fn day_3_a() -> u32 {
    let input = read_input(3);
    let line_len = input.lines().into_iter().peekable().peek().expect("failed to read file").len();

    // split by line
    let lines = input.lines();

    let binary_string : String = lines.into_iter()
        .map(|line| line.chars())
        .fold(vec![(0,0); line_len], |mut acc, val| {
            val.enumerate().for_each(|(i, digit)| {
                let current_total: (u32, u32) = acc[i];
                match digit.to_digit(10).expect("failed to parse char") {
                    0 => acc[i] = (current_total.0 + 1, current_total.1),
                    1 => acc[i] = (current_total.0, current_total.1 + 1),
                    _ => panic!("Unexpected number seen")
                }
            });
            acc
        })
        .iter().map(|column | if column.0 > column.1 {'0'} else {'1'})
        .fold(String::new(), |mut acc: String, digit: char| {
            acc.push(digit);
            acc
        });
    let binary_val = u32::from_str_radix(&binary_string, 2).expect("failed to parse final string");
    let inverse_binary_val = (2u32.pow(line_len as u32) -1) ^ binary_val;
    binary_val * inverse_binary_val

}

pub fn day_3_b() -> u32 {
    let input = read_input(3);

    let mut lines: Vec<String> = Vec::new();
    input.lines().for_each(|line| lines.push(line.to_string()));
    let pos_case = filter_lines(lines.clone(), true);
    let neg_case = filter_lines(lines.clone(), false);

    let pos_val = u32::from_str_radix(&pos_case[0], 2).expect("parse error");
    let neg_val = u32::from_str_radix(&neg_case[0], 2).expect("parse error");

    pos_val * neg_val
}

fn filter_lines(mut viable_lines: Vec<String>, positive_case: bool) -> Vec<String> {
    let mut i = 0u32;
    while viable_lines.len() > 1 {
        let (zeros, ones) = viable_lines.iter().fold((0, 0), |acc, val| {
            match val.chars().nth(i as usize).expect("misformatted string") {
                '0' => (acc.0 + 1, acc.1),
                '1' => (acc.0, acc.1 + 1),
                _ => panic!("bad formatting in input")
            }
        });
        viable_lines.retain(|line|{
            let positive_char_to_find = if zeros > ones { '0' } else if zeros < ones { '1' } else { '1' };
            let char_to_find = if positive_case {positive_char_to_find} else {if positive_char_to_find == '1' {'0'} else {'1'}};
            line.chars().nth(i as usize).unwrap() == char_to_find
        });
        i = i + 1;
    }
    viable_lines
}