use anyhow::{Context, Result};
// const INPUT: &str = include_str!("../test_input.txt");
const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq)]
struct Range {
    start: usize,
    end: usize,
}

fn parse_range(input: &str) -> Result<Range> {
    let (start, end) = input.trim().split_once('-').context("split_once")?;
    let start = start.parse().context("parse_start")?;
    let end = end.parse().context("parse_end")?;

    Ok(Range { start, end })
}

fn parse(inp: &str) -> Result<Vec<Range>> {
    inp.split(',').map(parse_range).collect()
}

fn solve(ids: Vec<Range>) -> (u32, usize) {
    let mut count = 0;
    let mut sum = 0usize;
    for id in ids {
        for n in id.start..=id.end {
            let string_rep = n.to_string();
            if string_rep.len() % 2 != 0 {
                continue;
            }
            let half = string_rep.len() / 2;
            if &string_rep[..half] == &string_rep[half..] {
                count += 1;
                sum = sum + n as usize;
            }
        }
    }

    (count, sum)
}

fn main() -> Result<()> {
    let ids = parse(INPUT)?;
    let invalid_ids = solve(ids);
    println!("{}", invalid_ids.1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert!(dbg!(parse(INPUT)).is_ok());
    }

    #[test]
    fn test_parse_range() {
        let input = "11-22";
        assert_eq!(parse_range(input).unwrap(), Range { start: 11, end: 22 });
    }
}
