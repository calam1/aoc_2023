fn main() {
    let re = regex::Regex::new(r"[A-Za-z]").unwrap();
    println!(
        "{:?}",
        include_str!("../input.txt")
            .lines()
            .map(|s| re.split(s).filter(|s| !s.is_empty()))
            .map(|s| {
                s.into_iter()
                    .flat_map(|s| {
                        if s.len() > 1 {
                            s.chars().map(|c| c.to_string()).collect::<Vec<String>>()
                        } else {
                            vec![s.to_string()]
                        }
                    })
                    .collect::<Vec<String>>()
            })
            .map(|s| {
                let mut v = Vec::new();
                v.push(s[0].to_string());
                v.push(s[s.len() - 1].to_string());
                v.join("")
            })
            .map(|s| { s.parse::<u32>().unwrap() })
            .sum::<u32>()
    );
}
