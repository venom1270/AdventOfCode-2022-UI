use app::file_utils::read_lines;

fn read_input(file_path : String) -> Vec<Option<i32>> {
    let mut input: Vec<Option<i32>> = Default::default();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(l) = line {
                match l.parse::<i32>() {
                    Ok(n) => {
                        input.push(Some(n));
                    },
                    Err(_) => {
                        input.push(None);
                    }
                }
            }
        }
    }
    input
}


pub fn part1(file_path: String) -> i32 {

    let input = read_input(file_path);

    let mut max_calories = 0;
    let mut curr_calories = 0;

    for l in input {
        match l {
            Some(n) => {
                curr_calories += n; 
            },
            None => {
                if max_calories < curr_calories {
                    max_calories = curr_calories;
                } 
                curr_calories = 0;
            }
        }
    }

    println!("{}", max_calories);

    max_calories

}


pub fn part2(file_path: String) -> i32 {

    let input = read_input(file_path);


    let mut elf1 = 0;
    let mut elf2 = 0;
    let mut elf3 = 0;
    let mut curr_calories = 0;

    for line in input {
        match line {
            Some(n) => {
                curr_calories += n; 
            },
            None => {
                if elf3 < curr_calories {
                    if elf2 < curr_calories {
                        elf3 = elf2;
                        if elf1 < curr_calories {
                            elf2 = elf1;
                            elf1 = curr_calories;
                        } else {
                            elf2 = curr_calories;
                        }
                    } else {
                        elf3 = curr_calories;
                    }
                }
                curr_calories = 0;
            }
        }
    }

    let top_elves_calories = elf1 + elf2 + elf3;
    println!("{}", top_elves_calories);

    top_elves_calories

}


