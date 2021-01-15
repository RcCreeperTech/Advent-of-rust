// NOTE: also contains challenge 04
const CHALLENGE_NO: i32 = 4;

pub fn init() {
  let string = std::fs::read_to_string("policies").expect("File could not be read");
  let data: Vec<(&str, &str)> = string
    .split("\n")
    .filter(|val| !val.is_empty())
    .map(|val| {
      let tmp: Vec<&str> = val.split(":").collect();
      // println!("Val: {:?}", (tmp[0], tmp[1]));
      (tmp[0], tmp[1].trim_start())
    })
    .collect();
  let mut count = 0;
  for (predicate, value) in data {
    let format = predicate.replace('-', " ");
    let parts: Vec<&str> = format.split_whitespace().collect();
    let character: char = parts[2].chars().next().expect("A char");
    let low = parts[0].parse::<i32>().expect("A number");
    let high = parts[1].parse::<i32>().expect("A number");
    match CHALLENGE_NO {
      3 => {
        let mut n = 0;
        for c in value.chars() {
          if c == character {
            n += 1;
          }
        }
        if n >= low && n <= high {
          count += 1;
          println!(
            "VALID => Input: {:?}. lower: {}. higher: {}. character: {}. count: {}",
            (predicate, value),
            low,
            high,
            character,
            n
          );
        }
      }
      4 => {
        let mut a = false;
        let mut b = false;
        for (idx, c) in value.chars().enumerate() {
          if idx + 1 == low as usize && c == character {
            a = true;
          }
          if idx + 1 == high as usize && c == character {
            b = true;
          }
        }
        if a ^ b {
          count += 1;
          println!(
            "VALID => Input: {:?}. lower: {}. higher: {}. character: {}.",
            (predicate, value),
            low,
            high,
            character
          );
        }
      }
      _ => {}
    }
  }
  println!("Challenge {} Result: {}", CHALLENGE_NO, count);
}
