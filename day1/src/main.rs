use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let elves = get_elves(args[1].clone());

    println!("first solution: {}", elves[0]);
    println!("second solution: {}", elves[0] + elves[1] + elves[2]);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_elves(input: String) -> Vec<i32> {
    let mut elves = Vec::new();
    let mut current_cals = 0;
    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(l) = line {
                if l == "" {
                    elves.push(current_cals);
                    current_cals = 0;
                    continue;
                }
                let snack_cals = l.parse::<i32>().unwrap();
                current_cals += snack_cals;
            }
        }
    }

    elves.sort_by(|a, b| b.cmp(a));

    return elves;
}
