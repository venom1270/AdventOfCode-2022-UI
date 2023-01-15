use app::file_utils::read_lines;
use std::collections::HashMap;
use std::io;

#[derive(Clone)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide
}
#[derive(Clone)]
enum Activity {
    Number(i64),
    Operation(String, Op, String)
}

fn parse_input(file_path: String) -> Result<HashMap<String, Activity>, io::Error> {

    let mut monkeys: HashMap<String, Activity> = Default::default(); 

    if let Ok(lines) = read_lines(file_path) {
        
        for line in lines {
            let l = line.expect("Error getting line");

            let mut s = l.split(' ').fuse();
            let name_s = s.next().expect("Error getting monkey name");
            let name = &name_s[..name_s.len()-1];
            let x = s.next().expect("Error");
            if let Some(operator) = s.next() {
                let mut oper = Op::Add;
                let y = s.next().expect("Error y");
                match operator {
                    "+" => oper = Op::Add,
                    "-" => oper = Op::Subtract,
                    "*" => oper = Op::Multiply,
                    "/" => oper = Op::Divide,
                    _ => println!("Error parsing operator")
                };
                monkeys.insert(name.to_string(), Activity::Operation(x.to_string(), oper, y.to_string()));
            } else {
                monkeys.insert(name.to_string(), Activity::Number(x.parse().expect("Error parsing number")));
            }

        }
    }

    Ok(monkeys)

}

fn solve(current: &String, monkeys: &HashMap<String, Activity>) -> i64 {
    if let Some(monkey) = monkeys.get(current) {
        match monkey {
            Activity::Number(n) => return *n,
            Activity::Operation(x_name, op, y_name) => {
                let x = solve(x_name, monkeys);
                let y = solve(y_name, monkeys);
                let result;
                match op {
                    Op::Add => result = x + y,
                    Op::Subtract => result = x - y,
                    Op::Multiply => result = x * y,
                    Op::Divide => result = x / y
                }
                return result;
            }
        }
    }

    0
}


pub fn solution(file_path: String) -> Result<(i64, i64), io::Error> {
    let mut monkeys = parse_input(file_path)?;

    let root = solve(&String::from("root"), &monkeys);
    println!("Root number (part 1): {}", root); 

    let mut found_human = 0;
    if let Some(a) = monkeys.get("root").clone() {
        if let Activity::Operation(x, _, y) = (*a).clone() {
            let mut left: i64 = 0;
            let mut right: i64 = std::i64::MAX / 100;
            
            while left < right {
                let mid = (left+right)/2;
                let mid1 = (left+mid)/2;
                let mid2 = (right+mid)/2;
                //println!("Testing ({}, {}) {}", left, right, mid1);
                monkeys.insert("humn".to_string(), Activity::Number(mid1));
                let l = solve(&x, &monkeys) - solve(&y, &monkeys);
                monkeys.insert("humn".to_string(), Activity::Number(mid2));
                let r = solve(&x, &monkeys) - solve(&y, &monkeys);
                //println!("{} ({},{}) {} ({},{})", l, l1, l2, r, r1, r2);
                if l == 0 {
                    println!("Found humn number (part 2) mid1: {}", mid1);
                    found_human = mid1;
                    break;
                } else if r == 0 {
                    println!("Found humn number (part 2): {}", mid2);
                    found_human = mid2;
                    break;
                }
                if l.abs() < r.abs() {
                    right = mid-1;
                } else {
                    left = mid+1;
                }
            }
        }
    }

    Ok((root, found_human))

}



