use itertools::Itertools;

fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|i| i.parse().unwrap())
            .tuple_windows::<(i32,i32,i32)>()
            .map(|(a,b,c)| a+b+c)
            .tuple_windows::<(_,_)>()
            .fold(0, |count, (prev,next)| count + if next > prev { 1 } else { 0 } )
    );
}
