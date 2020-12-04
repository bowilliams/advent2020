use std::fs;
use std::io::{self, BufRead};

fn main() {
    let product = find_product();
    println!("found {}", product)
}

fn find_product() -> i32 {
    if let Ok(file) = fs::File::open("./data/input") {
        let lines = io::BufReader::new(file).lines();
        let mut expenses: Vec<i32> = Vec::new();
        for line in lines {
            if let Ok(expense) = line {          
                println!("{}", expense);
                let expense: i32 = expense.trim().parse().expect("Expenses have to be numbers!");
                expenses.push(expense);
            }
        }
        for exp1 in &expenses {
            for exp2 in &expenses {
                for exp3 in &expenses {
                    if exp1 + exp2 + exp3 == 2020 {
                        println!("{} {} {}", exp1, exp2, exp3);
                        return exp1*exp2*exp3
                    }
                }
            }
        }
    }
    // gross, should return error
    0
}

fn test_sumtwo() {
    let expenses = vec!(1721, 979, 366, 299, 675, 1456);

    for exp1 in &expenses {
        for exp2 in &expenses {
            if exp1 + exp2 == 2020 {
                println!("{} {}", exp1, exp2);
                println!("{}", exp1*exp2);
                std::process::exit(0);
            }
        }
    }
}

fn test_sumthree() {
    let expenses = vec!(1721, 979, 366, 299, 675, 1456);

    for exp1 in &expenses {
        for exp2 in &expenses {
            for exp3 in &expenses {
                if exp1 + exp2 + exp3 == 2020 {
                    println!("{} {} {}", exp1, exp2, exp3);
                    println!("{}", exp1*exp2*exp3);
                }
            }
        }
    }   
}
