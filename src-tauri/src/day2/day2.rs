use app::file_utils::read_lines;
use std::io;
use std::io::ErrorKind::Other;
use app::unwrap_or_return;

pub fn part1(file_path: String) -> Result<i32, io::Error> {
    let mut all_points = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(l) = line {
                let mut tokens = l.split(' ').fuse();
                //let opponent = unwrap_or_return!(tokens.next()); //tokens.next().unwrap()?;
                let opponent = unwrap_or_return!(tokens.next(), "Opponent token empty"); //tokens.next().expect("Opponent token empty");
                let me = unwrap_or_return!(tokens.next(), "My token empty"); //tokens.next().expect("My token empty");

                match me {
                    "X" => all_points += 1,
                    "Y" => all_points += 2,
                    "Z" => all_points += 3,
                    _ => println!("ERROR")
                }

                if opponent == "A" {
                    match me {
                        "Y" => all_points += 6,
                        "X" => all_points += 3,
                        _ => ()
                    }
                } else if opponent == "B" {
                    match me {
                        "Z" => all_points += 6,
                        "Y" => all_points += 3,
                        _ => ()
                    }
                } else {
                    match me {
                        "X" => all_points += 6,
                        "Z" => all_points += 3,
                        _ => ()
                    }
                }
            }
        }

        println!("{}", all_points);
    }

    Ok(all_points)

}


pub fn part2(file_path: String) -> Result<i32, io::Error> {
    let mut all_points = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(l) = line {
                let mut tokens = l.split(' ').fuse();
                let opponent = unwrap_or_return!(tokens.next(), "Opponent token empty"); //tokens.next().expect("Opponent token empty");
                let outcome = unwrap_or_return!(tokens.next(), "Outcome token empty"); //tokens.next().expect("Outcome token empty");

                match outcome {
                    "X" => { // lose
                        match opponent {
                            "A" => all_points += 3,
                            "B" => all_points += 1,
                            "C" => all_points += 2,
                            _ => ()
                        }
                    }
                    "Y" => { // draw
                        all_points += 3;
                        match opponent {
                            "A" => all_points += 1,
                            "B" => all_points += 2,
                            "C" => all_points += 3,
                            _ => ()
                        }
                    }
                    "Z" => { // win
                        all_points += 6;
                        match opponent {
                            "A" => all_points += 2,
                            "B" => all_points += 3,
                            "C" => all_points += 1,
                            _ => ()
                        }
                    }
                    _ => println!("ERROR")
                }

            }
        }

        println!("{}", all_points);

    }

    Ok(all_points)

}


