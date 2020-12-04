use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Rule {
    min: usize,
    max: usize,
    check_letter: char
}

fn main() {
    //test();
    if let Ok(lines) = read_lines("./input") {
        let mut count = 0;
        for line in lines {
            if let Ok(rule_password) = line {
                let (rule, password) = parse_rule_password(rule_password.trim());
                if is_valid2(&rule, password) {
                    count += 1;
                }
            }
        }
        println!("{}", count);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_rule_password(rule_password_pair: &str) -> (Rule, &str) {
    let splits = rule_password_pair.split(':').collect::<Vec<&str>>();
    let rule_description = splits[0].trim();
    let check_letter = rule_description.chars().last().expect("Could find not check letter");
    let min_max = rule_description.split('-').collect::<Vec<&str>>();
    let min: usize = min_max[0].trim().parse().expect("Min has to be a number");
    let max: usize = min_max[1].split(' ').collect::<Vec<&str>>()[0].parse().expect("Max has to be a number");
    let password = splits[1].trim();
    let rule = Rule {
        min,
        max,
        check_letter
    };
    (rule, password)
}

fn is_valid(rule: &Rule, password: &str) -> bool {
    let count = password.matches(rule.check_letter).collect::<Vec<&str>>().len();
    count <= rule.max && count >= rule.min
}

fn is_valid2(rule: &Rule, password: &str) -> bool {
    // rules are not 0-indexed
    let min = rule.min - 1;
    let max = rule.max - 1;
    let chars = password.chars().collect::<Vec<char>>();
    if chars[min] == rule.check_letter && chars[max] == rule.check_letter {
        return false;
    }
    chars[min] == rule.check_letter || chars[max] == rule.check_letter 
}

fn test() {
    let data = vec!("1-3 a: abcde","1-3 b: cdefg","2-9 c: ccccccccc");
    let mut count = 0;
    for rule_password in data {
        let (rule, password) = parse_rule_password(rule_password);
        let is_valid = is_valid2(&rule, password);
        println!("{}", is_valid);
        if is_valid {
            count += 1;
        }
        println!("{:?} {} is valid: {}, count {}", rule, password, is_valid, count);
    }
}
