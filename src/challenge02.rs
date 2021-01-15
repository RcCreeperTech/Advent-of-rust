pub fn init() {
    let string = std::fs::read_to_string("numbers").expect("File could not be read");
    let string_vec = string.split("\n").collect::<Vec<&str>>();
    // let num_vec: Vec<i32> = string_vec
    //     .into_iter()
    //     .map(|elem| match elem.parse::<i32>() {
    //         Ok(number) => number,
    //         _ => -1,
    //     })
    //     .filter(|elem| *elem != -1)
    //     .collect();
    let num_vec: Vec<i32> = string_vec
        .into_iter()
        .filter_map(|elem| elem.parse::<i32>().ok())
        .collect();

    'outer: for (i, num1) in num_vec.iter().enumerate() {
        for (j, num2) in num_vec.iter().enumerate() {
            for (k, num3) in num_vec.iter().enumerate() {
                if i != j && i != k && j != k {
                    if num1 + num2 + num3 == 2020 {
                        println!(
                            "Number combo found! {}, {}, {}. Code is {}!",
                            num1,
                            num2,
                            num3,
                            num1 * num2 * num3
                        );
                        break 'outer;
                    }
                }
            }
        }
    }
}
