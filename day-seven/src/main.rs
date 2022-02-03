#![allow(unused)]

fn main() {
    let mut input = include_str!("../input/input.txt")
        .trim()
        .split(",")
        .map(|val| val.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let length = (input.len() as f32 / 2.0).floor();
    input.sort();
    let result = input
        .iter()
        .fold(0, |acc, val| acc + (length as i32 - val).abs());
    println!("{}", result);
}
