use std::fs;

fn main() {
    println!("{}", day_1_b());
}

fn read_input(day: u32) -> String {
    let input = fs::read_to_string(format!("src/input/{}.txt", day))
        .expect("Failed to read file");
    input
}

fn day_1_a() -> u32 {
    let input = read_input(1);
    // split by line
    let lines = input.lines();
    // parse into numbers
    let input_nums = lines.into_iter()
        .map(|line| line.parse::<u32>().unwrap());

    let (num_deeper, _prev) = input_nums.fold((0u32, u32::max_value()),|acc: (u32, u32), val| {
        return if acc.1 < val {
            (acc.0 + 1, val)
        } else {
            (acc.0, val)
        }
    });
    num_deeper
}

fn day_1_b() -> u32 {
    // translate list of nums into 3-item totals
    let input = read_input(1);
    // split by line
    let lines = input.lines();
    // parse into numbers
    let input_nums = lines.into_iter()
        .map(|line| line.parse::<u32>().unwrap());

    let (num_deeper, _prev) = input_nums
        .fold((0u32, (u32::max_value(),u32::max_value(),u32::max_value() )),
              |acc: (u32, (u32, u32, u32)), val| {
        return if acc.1.0 != u32::max_value() && acc.1.0 + acc.1.1 + acc.1.2 < acc.1.1 + acc.1.2 + val {
            (acc.0 + 1, (acc.1.1, acc.1.2, val))
        } else {
            (acc.0, (acc.1.1, acc.1.2, val))
        }
    });
    num_deeper
}