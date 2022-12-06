fn main() {
    let input = include_str!("../../inputs/day06.txt");
    /*println!("Part 1: {:?}", find_start_marker(input, 4));
    println!("Part 2: {:?}", find_start_marker(input, 14)); It stays as Option (Some)*/
    let part1 = find_start_marker(input, 4).unwrap().to_string();
    let part2 = find_start_marker(input, 14).unwrap().to_string();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

}
// Since the only thing that changes between part and part 2 is the length of the marker start, it can be done with only one function, passing length as an arg
fn find_start_marker(input: &str, length: usize) -> Option<usize> {
    let res = input
        .as_bytes().windows(length)
        .enumerate().find(|(_i, window)| {
            window
                .iter()
                .fold(0u32, |acc, c| (acc | (1 << (c - b'a'))))
                .count_ones()
                == length as u32
        }).map(|(i, _)| i + length);
        res
}

#[cfg(test)]
mod tests{
    use super::*;
    const EXAMPLE_1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE_2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE_3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE_4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    const EXAMPLE_5: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE_6: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE_7: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE_8: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE_9: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    // Part 1 tests
    #[test]
    fn test_example_1_1() {
        let marker = find_start_marker(EXAMPLE_1, 4).unwrap().to_string();
        assert_eq!(marker, "5");
    }
    #[test]
    fn test_example_1_2() {
        let marker = find_start_marker(EXAMPLE_2, 4).unwrap().to_string();
        assert_eq!(marker, "6");
    }
    #[test]
    fn test_example_1_3() {
        let marker = find_start_marker(EXAMPLE_3, 4).unwrap().to_string();
        assert_eq!(marker, "10");
    }
    #[test]
    fn test_example_1_4() {
        let marker = find_start_marker(EXAMPLE_4, 4).unwrap().to_string();
        assert_eq!(marker, "11");
    }

    // Part 2 tests
    #[test]
    fn test_example_2_1() {
        let marker = find_start_marker(EXAMPLE_5, 14).unwrap().to_string();
        assert_eq!(marker, "19");
    }
    #[test]
    fn test_example_2_2() {
        let marker = find_start_marker(EXAMPLE_6, 14).unwrap().to_string();
        assert_eq!(marker, "23");
    }
    #[test]
    fn test_example_2_3() {
        let marker = find_start_marker(EXAMPLE_7, 14).unwrap().to_string();
        assert_eq!(marker, "23");
    }
    #[test]
    fn test_example_2_4() {
        let marker = find_start_marker(EXAMPLE_8, 14).unwrap().to_string();
        assert_eq!(marker, "29");
    }
    #[test]
    fn test_example_2_5() {
        let marker = find_start_marker(EXAMPLE_9, 14).unwrap().to_string();
        assert_eq!(marker, "26");
    }
}