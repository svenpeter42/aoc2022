use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn find_unique_byte_window(input: &str, window_size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .filter_map(|(i, window)| {
            window
                .iter()
                .collect::<HashSet<_>>()
                .len()
                .eq(&window_size)
                .then(|| i + window_size)
        })
        .next()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("A: {}", find_unique_byte_window(&input, 4).unwrap());
    println!("B: {}", find_unique_byte_window(&input, 14).unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_a() {
        assert_eq!(
            find_unique_byte_window("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4).unwrap(),
            7
        );
        assert_eq!(
            find_unique_byte_window("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap(),
            5
        );
        assert_eq!(
            find_unique_byte_window("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap(),
            6
        );
        assert_eq!(
            find_unique_byte_window("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap(),
            10
        );
        assert_eq!(
            find_unique_byte_window("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap(),
            11
        );
    }

    #[test]
    fn examples_b() {
        assert_eq!(
            find_unique_byte_window("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap(),
            19
        );
        assert_eq!(
            find_unique_byte_window("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap(),
            23
        );
        assert_eq!(
            find_unique_byte_window("nppdvjthqldpwncqszvftbrmjlhg", 14).unwrap(),
            23
        );
        assert_eq!(
            find_unique_byte_window("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14).unwrap(),
            29
        );
        assert_eq!(
            find_unique_byte_window("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).unwrap(),
            26
        );
    }
}
