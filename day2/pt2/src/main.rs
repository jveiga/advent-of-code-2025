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
    for Range { start, end } in ids {
        for n in start..=end {
            let string_rep = n.to_string();

            for chunk_size in 1..=(string_rep.len() / 2) {
                let split_chunk = split_chunk(&string_rep, chunk_size);
                if !split_chunk.is_empty() && split_chunk.iter().all(|c| c == &split_chunk[0]) {
                    sum += n;
                    count += 1;
                    break;
                }
            }
        }
    }

    (count, sum)
}

fn split_chunk(input: &str, chunk_size: usize) -> Vec<&str> {
    if input.len() % chunk_size != 0 {
        return Vec::new();
    }
    let mut chunks = Vec::new();
    let mut starting = 0;

    while starting + chunk_size <= input.len() {
        chunks.push(&input[starting..starting + chunk_size]);
        starting += chunk_size;
    }

    chunks
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

    #[test]
    fn test_111() {
        let res = split_chunk("111", 1);
        assert_eq!(vec!["1", "1", "1"], res);
        let res = split_chunk("446446", 3);
        assert_eq!(vec!["446", "446"], res);
        let res = split_chunk("565656", 2);
        assert_eq!(vec!["56", "56", "56"], res);

        let res = split_chunk("2121212118", 4);
        assert!(res.is_empty());

        let res = split_chunk("1188511888", 4);
        assert_eq!(vec!["56", "56", "56"], res);
    }
}
