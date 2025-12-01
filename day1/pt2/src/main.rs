use anyhow::{Result, bail};

// const INPUT: &str = include_str!("../test_input.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    let instructions = parse_input(INPUT)?;
    let result = solve(instructions);
    println!("{}", result);

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Instruction {
    side: Side,
    rotations: i32,
}

#[derive(Debug, PartialEq)]
enum Side {
    Left,
    Right,
}

fn parse_line(line: &str) -> Result<Instruction> {
    let side = match line.chars().next() {
        Some('L') => Side::Left,
        Some('R') => Side::Right,
        other => bail!("unexpected char {:?}", other),
    };
    let Ok(rotations) = &line[1..].parse() else {
        bail!("unable to parse rotation {}", &line[1..]);
    };
    Ok(Instruction {
        side,
        rotations: *rotations,
    })
}

fn solve(instructions: Vec<Instruction>) -> i32 {
    const STARTING_POSITION: i32 = 50;
    let mut position = STARTING_POSITION;
    let mut zero_count = 0;
    for instruction in instructions {
        match instruction {
            Instruction {
                side: Side::Left,
                mut rotations,
            } => {
                while rotations > 0 {
                    position = (position - 1).rem_euclid(100);
                    rotations -= 1;
                    if position == 0 {
                        zero_count += 1;
                    }
                }
            }
            Instruction {
                side: Side::Right,
                mut rotations,
            } => {
                while rotations > 0 {
                    position = (position + 1).rem_euclid(100);
                    rotations -= 1;
                    if position == 0 {
                        zero_count += 1;
                    }
                }
            }
        }
    }

    zero_count
}

fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    let Ok(res) = input.lines().map(parse_line).collect() else {
        bail!("bad input");
    };

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use Side::*;
        dbg!((50i32 - 68).rem_euclid(100));
        assert!(false);
        assert_eq!(
            parse_line("L68").unwrap(),
            Instruction {
                side: Left,
                rotations: 68
            }
        );
        assert_eq!(
            parse_line("L30").unwrap(),
            Instruction {
                side: Left,
                rotations: 30
            }
        );
        assert_eq!(
            parse_line("R48").unwrap(),
            Instruction {
                side: Right,
                rotations: 48
            }
        );
        assert_eq!(
            parse_line("L5").unwrap(),
            Instruction {
                side: Left,
                rotations: 5
            }
        );
        assert_eq!(
            parse_line("R60").unwrap(),
            Instruction {
                side: Right,
                rotations: 60
            }
        );
        assert_eq!(
            parse_line("L55").unwrap(),
            Instruction {
                side: Left,
                rotations: 55
            }
        );
        assert_eq!(
            parse_line("L1").unwrap(),
            Instruction {
                side: Left,
                rotations: 1
            }
        );
        assert_eq!(
            parse_line("L99").unwrap(),
            Instruction {
                side: Left,
                rotations: 99
            }
        );
        assert_eq!(
            parse_line("R14").unwrap(),
            Instruction {
                side: Right,
                rotations: 14
            }
        );
        assert_eq!(
            parse_line("L82").unwrap(),
            Instruction {
                side: Left,
                rotations: 82
            }
        );
    }
}
