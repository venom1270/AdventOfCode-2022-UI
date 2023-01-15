use app::file_utils::read_lines;
use std::collections::VecDeque;
use std::io;
use std::io::ErrorKind::Other;
use app::unwrap_or_return;
use app::result_unwrap_or_return;

pub fn parse_input(file_path: String) -> Result<([VecDeque<char>; 9], Vec<(u32, u32, u32)>), io::Error> {
    
    let mut stacks: [VecDeque<char>; 9] = Default::default();
    let mut moves: Vec<(u32, u32, u32)> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {

        let mut lines = lines.peekable();

        // Parse stacks
        while lines.peek().is_some() {       
            let l = unwrap_or_return!(lines.next(), "Error")?; //.expect("Error").expect("Error");
            if l.len() >= 1 && l.chars().nth(1).unwrap() == '1' {
                break;
            }
            for i in 0..9 {
                if i*4 < l.len() && l.chars().nth(i*4).unwrap() == '[' {
                    stacks[i].push_front(l.chars().nth(i*4+1).unwrap());
                }
            }
        }
        // Empty line
        lines.next();
        // Parse moves
        for l in lines {
            let l = result_unwrap_or_return!(l, "Error moves"); //l.expect("Error moves");
            let split: Vec<&str> = l.split(' ').collect();
            let m1: u32 = split[1].parse().unwrap();
            let m2: u32 = split[3].parse().unwrap();
            let m3: u32 = split[5].parse().unwrap();
            moves.push((m1, m2-1, m3-1));
        }
    }

    Ok((stacks, moves))

}

pub fn part1(input: ([VecDeque<char>; 9], Vec<(u32, u32, u32)>)) -> Result<String, io::Error> {

    let (mut stacks, moves) = input;

    for m in moves {
        let n = m.0;
        let from = m.1;
        let to = m.2;
        for _ in 0..n {
            match stacks[from as usize].pop_back() {
                Some(e) => stacks[to as usize].push_back(e),
                None => ()
            }
            // So this wouldn't work because of borrowing?? But above does work???
            /*if !stacks[from as usize].is_empty() {
                stacks[to as usize].push_back(stacks[from as usize].pop_back().unwrap());
            }*/
        }
    }

    let mut result = String::from("");
    for mut s in stacks {
        match s.pop_back() {
            Some(c) => result.push(c),
            None => result.push(' '),
        }
    }

    println!("{}", result);
    Ok(result)

}


pub fn part2(input: ([VecDeque<char>; 9], Vec<(u32, u32, u32)>)) -> Result<String, io::Error> {

    let (mut stacks, moves) = input;

    for m in moves {
        let n = m.0;
        let from = m.1;
        let to = m.2;
        let mut tmp: VecDeque<char> = VecDeque::new();
        for _ in 0..n {
            match stacks[from as usize].pop_back() {
                Some(e) => tmp.push_front(e),
                None => ()
            }
        }
        for c in tmp {
            stacks[to as usize].push_back(c);
        }
    }

    let mut result = String::from("");
    for mut s in stacks {
        match s.pop_back() {
            Some(c) => result.push(c),
            None => result.push(' '),
        }
    }

    println!("{}", result);
    Ok(result)

}


