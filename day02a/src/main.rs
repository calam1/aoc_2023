fn main() {
    println!(
        "{:?}",
        include_str!("../input.txt")
            .lines()
            // split on they colon first part is the key second part is the 3 times the dice are pulled out of the bag
            .map(|s| s.split_once(':').unwrap())
            .map(|(key, value)| {
                (
                    // game id
                    key,
                    // split the 3 times the dice are pulled out of the bag
                    value.split(|c| c == ';').collect::<Vec<&str>>(),
                )
            })
            // this block of code swaps the key and value, i.e. 1 blue -> blue 1, etc
            // and convert the value to a vector of tuples i.e. [("blue", 3), ("red", 1), ("green", 2)]
            .map(|(key, value)| {
                (
                    key,
                    value
                        .iter()
                        .map(|s| s.split(',').map(|t| t.trim()))
                        .flat_map(|s| {
                            s.map(|t| {
                                let (a, b) = t.split_once(' ').unwrap();
                                (b, a.parse::<u32>().unwrap())
                            })
                            .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>(),
                )
            })
            // .collect::<Vec<_>>()
            // iterate through the vector of tuples and check if any of the values are above the limit of 12 red, 13 green , 14 blue
            // if any are above the threshold that means the game is not possible so don't use the game id to sum ut the total score
            .map(|(key, value)| {
                (
                    key,
                    value
                        .iter()
                        .cloned()
                        .filter(|&(key, v)| match key {
                            "red" => v > 12,
                            "green" => v > 13,
                            "blue" => v > 14,
                            _ => false,
                        })
                        .collect::<Vec<_>>(),
                )
            })
            // .collect::<Vec<_>>()
            // now check to see if the length of the Game 1 value is 0 and if it is use the game id to sum up the total points
            .map(|s| {
                if s.1.len() == 0 {
                    s.0
                        .chars()
                        .filter(|c| c.is_digit(10))
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap()
                } else {
                    0
                }
            })
            .sum::<u32>()
    );
}
