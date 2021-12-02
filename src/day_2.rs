use crate::utils::read_input;

pub fn day_2_a() -> u32 {
    let input = read_input(2);
    // split by line
    let lines = input.lines();


    let parts = lines.map(|line| {
        //println!("{}", line);
        let mut components = line.split(' ');
        (components.nth(0).expect("misformatted input"), components.nth(0).expect("misformatted input"))
    });

    let (distance, depth) = parts.fold((0, 0), |acc, val| {
        let (term, amount) = (val.0, val.1);
        return match term {
            "forward" => (acc.0 + amount.parse::<u32>().unwrap(), acc.1),
            "up" => (acc.0, acc.1 - amount.parse::<u32>().unwrap()),
            "down" => (acc.0, acc.1 + amount.parse::<u32>().unwrap()),
            _ => panic!("Poorly formatted input")
        }
    });
    distance * depth
}

pub fn day_2_b() -> u32 {
    let input = read_input(2);
    // split by line
    let lines = input.lines();


    let parts = lines.map(|line| {
        //println!("{}", line);
        let mut components = line.split(' ');
        (components.nth(0).expect("misformatted input"), components.nth(0).expect("misformatted input"))
    });

    let (distance, depth, aim) = parts.fold((0, 0, 0), |acc, val| {
        let (term, amount) = (val.0, val.1.parse::<u32>().expect("misformatted input"));
        return match term {
            "forward" => (acc.0 + amount, acc.1 + acc.2 * amount, acc.2),
            "up" => (acc.0, acc.1, acc.2 - amount),
            "down" => (acc.0, acc.1, acc.2 + amount),
            _ => panic!("Poorly formatted input")
        }
    });
    distance * depth
}