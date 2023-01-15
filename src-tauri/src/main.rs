#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error)
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

mod day1 {
  pub mod day1;
}
mod day2 {
  pub mod day2;
}
mod day3 {
  pub mod day3;
}
mod day4 {
  pub mod day4;
}
mod day5 {
  pub mod day5;
}
mod day6 {
  pub mod day6;
}
mod day7 {
  pub mod day7;
}
mod day8 {
  pub mod day8;
}
mod day9 {
  pub mod day9;
}
mod day10 {
  pub mod day10;
}
mod day11 {
  pub mod day11;
}
mod day12 {
  pub mod day12;
}
mod day13 {
  pub mod day13;
}
mod day14 {
  pub mod day14;
}
mod day15 {
  pub mod day15;
}
mod day16 {
  pub mod day16;
}
mod day17 {
  pub mod day17;
}
mod day18 {
  pub mod day18;
}
mod day19 {
  pub mod day19;
}
mod day20 {
  pub mod day20;
}
mod day21 {
  pub mod day21;
}
mod day22 {
  pub mod day22;
}
mod day23 {
  pub mod day23;
}
mod day24 {
  pub mod day24;
}
mod day25 {
  pub mod day25;
}

use std::{thread, time};

static mut BACKEND_DELAY: bool = true;

#[tauri::command]
async fn update_backend_delay(delay: bool) {
  unsafe {
    BACKEND_DELAY = !delay;
  }
}

#[tauri::command]
async fn get_backend_delay() -> bool {
  unsafe {
    BACKEND_DELAY
  }
}

#[tauri::command]
async fn day1(file_path: &str) -> Result<(i32, i32), ()> {
  let part1 = day1::day1::part1(String::from(file_path));
  let part2 = day1::day1::part2(String::from(file_path));
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok((part1, part2))
}

#[tauri::command]
async fn day2(file_path: &str) -> Result<(i32, i32), Error> {
  let part1 = day2::day2::part1(String::from(file_path))?;
  let part2 = day2::day2::part2(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok((part1, part2))
}

#[tauri::command]
async fn day3(file_path: &str) -> Result<(u32, u32), Error> {
  let part1 = day3::day3::part1(String::from(file_path))?;
  let part2 = day3::day3::part2(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok((part1, part2))
}

#[tauri::command]
async fn day4(file_path: &str) -> Result<(i32, i32), Error> {
  let part1 = day4::day4::part1(String::from(file_path))?;
  let part2 = day4::day4::part2(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok((part1, part2))
}

#[tauri::command]
async fn day5(file_path: &str) -> Result<(String, String), Error> {
  let input = day5::day5::parse_input(String::from(file_path))?;
  let part1 = day5::day5::part1(input.clone())?;
  let part2 = day5::day5::part2(input)?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok((part1, part2))
}

#[tauri::command]
async fn day6(file_path: &str) -> Result<(u32, u32), Error> {
  let part1 = day6::day6::part1(String::from(file_path))?;
  let part2 = day6::day6::part2(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok((part1, part2))
}

#[tauri::command]
async fn day7(file_path: &str) -> Result<(u32, u32), Error> {
  let mut input = day7::day7::parse_input(String::from(file_path))?;
  let result = day7::day7::part1and2(&mut input)?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day8(file_path: &str) -> Result<(i32, i32), Error> {
  let input = day8::day8::parse_input(String::from(file_path))?;
  let result = day8::day8::solution(&input)?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day9(file_path: &str) -> Result<(i32, i32), Error> {
  let result = day9::day9::solution(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day10(file_path: &str) -> Result<(i32, String), Error> {
  let result = day10::day10::solution(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok((result.0, result.1.replace("\n", "<br>")))
}

#[tauri::command]
async fn day11(file_path: &str) -> Result<(u64, u64), Error> {
  let result = day11::day11::solution(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day12(file_path: &str) -> Result<(u32, u32), Error> {
  let (start, end, grid) = day12::day12::parse_input(String::from(file_path))?;
  let result = day12::day12::solution(start, end, grid);
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day13(file_path: &str) -> Result<(u32, u32), Error> {
  let result = day13::day13::solution(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day14(file_path: &str) -> Result<(u32, u32), Error> {
  let result = day14::day14::solution(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day15(file_path: &str) -> Result<(u32, u64), Error> {
  let result = day15::day15::solution(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day16(file_path: &str) -> Result<(u32, u32), Error> {
  let result = day16::day16::solution(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day17(file_path: &str) -> Result<(i64, i64), Error> {
  let result = day17::day17::solution(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day18(file_path: &str) -> Result<(i32, i32), Error> {
  let result = day18::day18::solution(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day19(file_path: &str) -> Result<(u32, u32), Error> {
  let result = day19::day19::solution(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day20(file_path: &str) -> Result<(i64, i64), Error> {
  let result = day20::day20::solution(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day21(file_path: &str) -> Result<(i64, i64), Error> {
  let result = day21::day21::solution(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day22(file_path: &str) -> Result<(u32, u32), Error> {
  let result = day22::day22::solution(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day23(file_path: &str) -> Result<(i32, i32), Error> {
  let result = day23::day23::solution(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day24(file_path: &str) -> Result<(u32, u32), Error> {
  let result = day24::day24::solution(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}

#[tauri::command]
async fn day25(file_path: &str) -> Result<(i64, String), Error> {
  let result = day25::day25::solution(String::from(file_path))?;
  unsafe {
    if BACKEND_DELAY {
      thread::sleep(time::Duration::from_secs(1));
    }
  }
  Ok(result)
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, 
                                            day12, day13, day14, day15, day16, day17, day18, day19, day20, day21, 
                                            day22, day23, day24, day25,
                                            get_backend_delay, update_backend_delay])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
