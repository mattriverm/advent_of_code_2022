fn check_signal(input: &str, packet_size: usize) -> usize {
    let split = input //slider
        .split("")
        .filter(|c| !c.is_empty())
        .collect::<Vec<&str>>();
    let res = split
        .windows(packet_size)
        .enumerate()
        .filter_map(|(pos, characters)| {
            let mut hash_set = std::collections::HashSet::new();
            for c in characters {
                if !hash_set.insert(c) {
                    return None;
                }
            }
            Some(pos + packet_size)
        })
        .collect::<Vec<usize>>();

    res[0]
}

fn main() {
    // Part 1
    println!(
        "First start of packet is {}",
        check_signal(include_str!("../input.txt"), 4)
    );

    // Part 2
    println!(
        "First start of message is {}",
        check_signal(include_str!("../input.txt"), 14)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(7, check_signal("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
    }
    #[test]
    fn case2() {
        assert_eq!(5, check_signal("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
    }
    #[test]
    fn case3() {
        assert_eq!(6, check_signal("nppdvjthqldpwncqszvftbrmjlhg", 4));
    }
    #[test]
    fn case4() {
        assert_eq!(10, check_signal("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
    }
    #[test]
    fn case5() {
        assert_eq!(11, check_signal("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
    }

    // Part 2
    #[test]
    fn case6() {
        assert_eq!(19, check_signal("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
    }

    #[test]
    fn case7() {
        assert_eq!(23, check_signal("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
    }
    #[test]
    fn case8() {
        assert_eq!(23, check_signal("nppdvjthqldpwncqszvftbrmjlhg", 14));
    }
    #[test]
    fn case9() {
        assert_eq!(29, check_signal("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
    }
    #[test]
    fn case10() {
        assert_eq!(26, check_signal("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
    }
}
