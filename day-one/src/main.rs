use std::fs;

fn main() {
    println!("{}",fs::read_to_string("input.txt").expect("error reading input.txt").lines().map(|line| line.chars().filter(|c| c.is_numeric()).fold((None, None), |(first, last), c| match (first, last) { (None, None) => (Some(c), Some(c)), (_, _) => (first, Some(c)) })).filter_map(|(first, last)| match (first, last) { (Some(f), Some(l)) => Some([f, l]), _ => None }).map(|array| array.iter().collect::<String>().parse::<u32>().unwrap_or(0)).sum::<u32>())
}
