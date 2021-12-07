use std::io::BufRead;
fn main() {
    let file = std::io::BufReader::new(std::fs::File::open("./verify_input.txt").unwrap());
    let result = file
        .lines()
        .map(|val| val.unwrap().parse::<i32>().unwrap())
        .scan(0, |state, val| {
            let temp = *state;
            *state = val;
            Some(val > temp)
        })
        .filter(|val| *val)
        .count();
    println!("{}", result - 1);
}
