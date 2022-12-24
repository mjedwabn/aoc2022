mod day04 {
    use std::io::BufRead;

    pub fn count_overlapping_pairs(input: &mut dyn BufRead) -> usize {
        let pairs = parse_input(input);
        return pairs.iter().filter(|pair| pair.is_overlapping()).count();
    }

    pub fn count_partially_overlapping_pairs(input: &mut dyn BufRead) -> usize {
        let pairs = parse_input(input);
        return pairs.iter().filter(|pair| pair.is_partially_overlapping()).count();
    }

    fn parse_input(input: &mut dyn BufRead) -> Vec<Pair> {
        return read_input(input).iter().map(|line| parse_pair(line)).collect();
    }

    fn read_input(input: &mut dyn BufRead) -> Vec<String> {
        return input.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    }

    fn parse_pair(line: &String) -> Pair {
        let (elf1, elf2) = line.split_once(',').unwrap();
        
        return Pair { elf1: parse_range(elf1), elf2: parse_range(elf2) };
    }

    fn parse_range(range: &str) -> (u32, u32) {
        return range.split_once('-')
            .map(|(start, end)| (start.parse::<u32>().unwrap(), end.parse::<u32>().unwrap()))
            .unwrap();
    }

    struct Pair {
        elf1: (u32, u32),
        elf2: (u32, u32)
    }

    impl Pair {
        fn is_overlapping(&self) -> bool {
            return (self.elf1.0 >= self.elf2.0 && self.elf1.1 <= self.elf2.1)
                || (self.elf2.0 >= self.elf1.0 && self.elf2.1 <= self.elf1.1);
        }

        fn is_partially_overlapping(&self) -> bool {
            return (self.elf1.0 >= self.elf2.0 && self.elf1.0 <= self.elf2.1)
                || (self.elf1.1 >= self.elf2.0 && self.elf1.1 <= self.elf2.1)
                || (self.elf2.0 >= self.elf1.0 && self.elf2.0 <= self.elf1.1)
                || (self.elf2.1 >= self.elf1.0 && self.elf2.1 <= self.elf1.1);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day04;
    use std::{fs::File, io::BufReader};

    #[test]
    fn part1_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day04::count_overlapping_pairs(&mut f), 2);
    }

    #[test]
    fn part1_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day04::count_overlapping_pairs(&mut f), 550);
    }

    #[test]
    fn part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day04::count_partially_overlapping_pairs(&mut f), 4);
    }

    #[test]
    fn part2_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day04::count_partially_overlapping_pairs(&mut f), 931);
    }
}
