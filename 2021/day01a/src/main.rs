use itertools::Itertools;

fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|i| i.parse().unwrap())
            .tuple_windows::<(i32,i32)>()
            .fold(0, |count, (prev, next)| if prev < next { count + 1 } else { count })
    )
}
