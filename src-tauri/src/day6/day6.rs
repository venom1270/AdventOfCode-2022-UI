use app::file_utils::read_lines;
use std::io;

pub fn part1(file_path : String) -> Result<u32, io::Error> {
    let mut left = 0;
    let mut right = 1;
    // I need mut here for some reason
    if let Ok(mut lines) = read_lines(file_path) {
        let line = lines.next().unwrap().unwrap();
        while right-left < 4 && right < line.len() {
            let c = line.chars().nth(right).unwrap();
            let subs = &line[left..right];
            if subs.contains(c) {
                left += subs.find(c).unwrap() + 1;
            }
            right += 1;
        }
        println!("{}", right);
    }
    Ok(right as u32)
}


pub fn part2(file_path : String) -> Result<u32, io::Error> {
    let mut left = 0;
    let mut right = 1;
    if let Ok(mut lines) = read_lines(file_path) {
        let line = lines.next().unwrap().unwrap();
        while right-left < 14 && right < line.len() {
            let c = line.chars().nth(right).unwrap();
            let subs = &line[left..right];
            if subs.contains(c) {
                left += subs.find(c).unwrap() + 1;
            }
            right += 1;
        }
        println!("{}", right);
    }

    Ok(right as u32)

}


