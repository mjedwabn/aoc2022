mod day06 {
    use std::io::BufRead;
    use itertools::Itertools;

    pub fn locate_first_start_of_packet(input: &mut dyn BufRead) -> usize {
        return locate_start_of_unique_sequence_of_characters(input, 4);
    }
    
    pub fn locate_first_start_of_message(input: &mut dyn BufRead) -> usize {
        return locate_start_of_unique_sequence_of_characters(input, 14);
    }

    fn locate_start_of_unique_sequence_of_characters(input: &mut dyn BufRead, n: usize) -> usize {
        let buffer = parse_input(read_input(input));
        return buffer.windows(n).zip(0..buffer.len())
            .filter(|(candidate, _)| is_marker(candidate, n))
            .map(|(_, i)| i)
            .collect::<Vec<usize>>().first().map(|i| *i + n).unwrap();
    }

    fn read_input(input: &mut dyn BufRead) -> String {
        return String::from(input.lines().map(|line| line.unwrap()).collect::<Vec<String>>().get(0).unwrap());
    }

    fn parse_input(line: String) -> Vec<char> {
        return line.chars().collect();
    }

    fn is_marker(candidate: &&[char], number_of_distinct_characters: usize) -> bool {
        return candidate.iter().unique().count() == number_of_distinct_characters;
    }
}

#[cfg(test)]
mod tests {
    use crate::day06;
    use std::{fs::File, io::BufReader};

    #[test]
    fn part1_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day06::locate_first_start_of_packet(&mut f), 7);
    }

    #[test]
    fn part1_other_sample_inputs() {
        assert_eq!(day06::locate_first_start_of_packet(&mut "bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()), 5);
        assert_eq!(day06::locate_first_start_of_packet(&mut "nppdvjthqldpwncqszvftbrmjlhg".as_bytes()), 6);
        assert_eq!(day06::locate_first_start_of_packet(&mut "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes()), 10);
        assert_eq!(day06::locate_first_start_of_packet(&mut "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()), 11);
    }

    #[test]
    fn part1_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day06::locate_first_start_of_packet(&mut f), 1356);
    }

    #[test]
    fn part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day06::locate_first_start_of_message(&mut f), 19);
    }

    #[test]
    fn part2_other_sample_inputs() {
        assert_eq!(day06::locate_first_start_of_message(&mut "bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()), 23);
        assert_eq!(day06::locate_first_start_of_message(&mut "nppdvjthqldpwncqszvftbrmjlhg".as_bytes()), 23);
        assert_eq!(day06::locate_first_start_of_message(&mut "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes()), 29);
        assert_eq!(day06::locate_first_start_of_message(&mut "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()), 26);
    }

    #[test]
    fn part2_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day06::locate_first_start_of_message(&mut f), 2564);
    }
}
