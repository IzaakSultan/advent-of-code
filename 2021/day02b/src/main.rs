use regex::Regex;

fn main() {
    let re = Regex::new(r#"^([forward|down|up]+) (\d+)$"#).unwrap();

    let (x, y, _) = include_str!("../input.txt")
        .lines()
        .map(|i| re.captures(i).unwrap())
        .fold((0, 0, 0), |coords, instruction| {
            let magnitude = instruction[2].parse::<i32>().unwrap();

            match &instruction[1] {
                "forward" => (coords.0 + magnitude, coords.1 + magnitude * coords.2, coords.2),
                "up" => (coords.0, coords.1, coords.2 - magnitude),
                "down" => (coords.0, coords.1, coords.2 + magnitude),
                _ => coords
            }
        });

    println!(
        "{}",
        x*y
    );
}
