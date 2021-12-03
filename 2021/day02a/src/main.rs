use regex::Regex;

fn main() {
    let re = Regex::new(r#"^([forward|down|up]+) (\d+)$"#).unwrap();

    let (x, y) = include_str!("../input.txt")
        .lines()
        .map(|i| re.captures(i).unwrap())
        .fold((0, 0), |coords, instruction| {
            let magnitude = instruction[2].parse::<i32>().unwrap();

            match &instruction[1] {
                "forward" => (coords.0 + magnitude, coords.1),
                "up" => (coords.0, coords.1 - magnitude),
                "down" => (coords.0, coords.1 + magnitude),
                _ => coords
            }
        });

    println!(
        "{}",
        x*y
    );
}
