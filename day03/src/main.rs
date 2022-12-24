mod day03 {
    use std::io::BufRead;
    use itertools::Itertools;
    
    pub fn sum_priorities_of_items_appearing_in_both_compartments(input: &mut dyn BufRead) -> u32 {
        let rucksacks = parse_input(input);
        return rucksacks.iter().map(|r| sum_priorities_of_items_appearing_in_both_compartments_of_a_rucksack(r)).sum();
    }

    fn sum_priorities_of_items_appearing_in_both_compartments_of_a_rucksack(r: &Rucksack) -> u32 {
        return r.find_items_appearing_in_both_compartments().iter().sum();
    }

    pub fn sum_priorities_of_groups_badges(input: &mut dyn BufRead) -> u32 {
        let rucksacks = parse_input(input);
        return rucksacks.chunks(3).map(|group| find_badge(group)).sum();
    }

    fn find_badge(group: &[Rucksack]) -> u32 {
        let e1 = group.get(0).unwrap().get_items();
        let e2 = group.get(1).unwrap().get_items();
        let e3 = group.get(2).unwrap().get_items();
        return e1.iter()
            .filter(|it| e2.contains(*it))
            .filter(|it| e3.contains(*it))
            .unique()
            .map(|it| *it)
            .sum();
    }

    fn parse_input(input: &mut dyn BufRead) -> Vec<Rucksack> {
        return read_input(input).iter().map(|line| parse_rucksack(line)).collect();
    }

    fn read_input(input: &mut dyn BufRead) -> Vec<String> {
        return input.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    }

    fn parse_rucksack(line: &String) -> Rucksack {
        let (c1, c2) = line.split_at(line.len() / 2);
        let items1 = c1.chars().into_iter().map(|item| get_item_priority(item)).collect();
        let items2 = c2.chars().into_iter().map(|item| get_item_priority(item)).collect();
        return Rucksack { c1: Compartment { items: items1 }, c2: Compartment { items: items2 } };
    }

    fn get_item_priority(item: char) -> u32 {
        if item >= 'a' && item <= 'z' {
            return item as u32 - 'a' as u32 + 1;
        }
        else if item >= 'A' && item <= 'Z' {
            return item as u32 - 'A' as u32 + 27;
        }
        else {
            return 0;
        }
    }

    struct Rucksack {
        c1: Compartment,
        c2: Compartment
    }

    impl Rucksack {
        fn find_items_appearing_in_both_compartments(&self) -> Vec<u32> {
            return self.c1.items.iter()
                .filter(|it| self.c2.items.contains(*it))
                .map(|it| *it)
                .unique()
                .collect::<Vec<u32>>();
        }

        fn get_items(&self) -> Vec<&u32> {
            return self.c1.items.iter().chain(self.c2.items.iter()).collect();
        }
    }

    struct Compartment {
        items: Vec<u32>
    }
}

#[cfg(test)]
mod tests {
    use crate::day03;
    use std::{fs::File, io::BufReader};

    #[test]
    fn items_appearing_in_both_compartments() {
        assert_eq!(day03::sum_priorities_of_items_appearing_in_both_compartments(&mut "vJrwpWtwJgWrhcsFMMfFFhFp".as_bytes()), 16);
    }

    #[test]
    fn sum_uniqe_items() {
        assert_eq!(day03::sum_priorities_of_items_appearing_in_both_compartments(&mut "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".as_bytes()), 38);
    }

    #[test]
    fn part1_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day03::sum_priorities_of_items_appearing_in_both_compartments(&mut f), 157);
    }

    #[test]
    fn part1_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day03::sum_priorities_of_items_appearing_in_both_compartments(&mut f), 7997);
    }

    #[test]
    fn part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day03::sum_priorities_of_groups_badges(&mut f), 70);
    }

    #[test]
    fn part2_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day03::sum_priorities_of_groups_badges(&mut f), 2545);
    }
}
