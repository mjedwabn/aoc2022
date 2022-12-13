pub mod day01 {
    use std::io::BufRead;

    pub fn how_many_total_calories_is_elf_carrying_at_most(input: &mut dyn BufRead) -> i64 {
        return parse_input(input)
            .iter()
            .map(|elf| elf.get_total_carried_calories())
            .max().unwrap_or(0);
    }

    pub fn how_many_calories_are_carrying_top_three_elves_in_total(input: &mut dyn BufRead) -> i64 {
        let mut calories: Vec<i64> = parse_input(input)
            .iter()
            .map(|elf| elf.get_total_carried_calories())
            .collect();
        calories.sort_by(|a, b| b.cmp(a));
        return calories.iter().take(3).sum();
    }

    fn parse_input(input: &mut dyn BufRead) -> Vec<Elf> {
        return read_input(input)
            .split(|line| line == "")
            .map(|inventory| parse_inventory(inventory))
            .map(|food_calories| Elf::new(food_calories))
            .collect();
    }

    fn read_input(input: &mut dyn BufRead) -> Vec<String> {
        return input.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    }

    fn parse_inventory(lines: &[String]) -> Vec<i64> {
        return lines
            .iter()
            .map(|l| l.parse::<i64>().unwrap())
            .collect();
    }

    struct Elf {
        carried_food_calories: Vec<i64>
    }

    impl Elf {
        fn new(food_calories: Vec<i64>) -> Elf {
            Elf {
                carried_food_calories: food_calories
            }
        }
    }

    pub trait Carrier {
        fn get_total_carried_calories(&self) -> i64;
    }

    impl Carrier for Elf {
        fn get_total_carried_calories(&self) -> i64 {
            return self.carried_food_calories.iter().sum();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day01;
    use std::{fs::File, io::BufReader};

    #[test]
    fn day01_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day01::how_many_total_calories_is_elf_carrying_at_most(&mut f), 24000);
    }

    #[test]
    fn day01_part1_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day01::how_many_total_calories_is_elf_carrying_at_most(&mut f), 69836);
    }

    #[test]
    fn day01_part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day01::how_many_calories_are_carrying_top_three_elves_in_total(&mut f), 45000);
    }

    #[test]
    fn day01_part2_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day01::how_many_calories_are_carrying_top_three_elves_in_total(&mut f), 207968);
    }
}
