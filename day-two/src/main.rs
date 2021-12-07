use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Debug)]
enum Movement {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AddAssign<Movement> for Position {
    fn add_assign(&mut self, rhs: Movement) {
        match rhs {
            Movement::Forward(val) => {
                self.x = self.x + val;
            }
            Movement::Down(val) => {
                self.y = self.y + val;
            }
            Movement::Up(val) => {
                self.y = self.y - val;
            }
        }
    }
}
impl std::fmt::Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split_whitespace();
        if let (Some(direction), Some(magnitute)) = (parts.next(), parts.next()) {
            let mag = magnitute.parse::<i32>().unwrap();

            let mov = match direction {
                "forward" => Movement::Forward(mag),
                "down" => Movement::Down(mag),
                "up" => Movement::Up(mag),
                _ => return Err(()),
            };
            return Ok(mov);
        }
        return Err(());
    }
}

fn main() {
    let mut position = Position::default();
    let input = include_str!("../input/input.txt")
        .lines()
        .map(|line| line.parse::<Movement>().unwrap());
    for movement in input {
        position += movement;
    }
    println!("{}", position.x * position.y);
}
