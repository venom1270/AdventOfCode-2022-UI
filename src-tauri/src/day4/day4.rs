use app::file_utils::read_lines;
use std::io;
use std::io::ErrorKind::Other;
use app::result_unwrap_or_return;

pub fn part1(file_path : String) -> Result<i32, io::Error> {
    let mut overlaps = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(l) = line {              
                
                let s: Vec<&str> = l.split(',').collect();
                let s1: Vec<&str> = s[0].split('-').collect();
                let s2: Vec<&str> = s[1].split('-').collect();

                let x1: i32 = result_unwrap_or_return!(s1[0].parse(), "Error x1"); //.expect("Error x1");
                let y1: i32 = result_unwrap_or_return!(s1[1].parse(), "Error y1"); //.expect("Error y1");
                let x2: i32 = result_unwrap_or_return!(s2[0].parse(), "Error x2"); //.expect("Error x2");
                let y2: i32 = result_unwrap_or_return!(s2[1].parse(), "Error y2"); //.expect("Error y2");

                if (x1 >= x2 && y1 <= y2) || (x1 <= x2 && y1 >= y2) {
                    overlaps += 1;
                }
            }
        }
        println!("{}", overlaps);
    }

    Ok(overlaps)

}


pub fn part2(file_path : String) -> Result<i32, io::Error> {
    let mut overlaps = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(l) = line {              
                
                let s: Vec<&str> = l.split(',').collect();
                let s1: Vec<&str> = s[0].split('-').collect();
                let s2: Vec<&str> = s[1].split('-').collect();

                let x1: i32 = result_unwrap_or_return!(s1[0].parse(), "Error x1"); //.expect("Error x1");
                let y1: i32 = result_unwrap_or_return!(s1[1].parse(), "Error y1"); //.expect("Error y1");
                let x2: i32 = result_unwrap_or_return!(s2[0].parse(), "Error x2"); //.expect("Error x2");
                let y2: i32 = result_unwrap_or_return!(s2[1].parse(), "Error y2"); //.expect("Error y2");

                if !(y2 < x1 || x2 > y1) {
                    overlaps += 1;
                }
            }
        }
        println!("{}", overlaps);
    }

    Ok(overlaps)

}


