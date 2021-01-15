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

    let test_vec = num_vec.clone();

    'outer: for (i, num) in num_vec.iter().enumerate() {
        for (j, test) in test_vec.iter().enumerate() {
            if i != j && test + num == 2020 {
                println!(
                    "Number combo found! {}, {}. Code is {}!",
                    test,
                    num,
                    test * num
                );
                break 'outer;
            }
        }
    }
}
