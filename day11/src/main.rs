mod day11 {
    use std::io::BufRead;
    use regex::Regex;

    pub fn level_of_monkey_business_after_rounds(input: &mut dyn BufRead, rounds: usize, relief: u32) -> u32 {
        let monkeys = parse_input(input);
        let mut game = Game { monkeys, relief };

        for _ in 0..rounds {
            game.play_round();
        }

        return game.level_of_monkey_business();
    }

    fn parse_input(input: &mut dyn BufRead) -> Vec<Monkey> {
        let lines = read_input(input);
        return lines.split(|line| line == "")
            .map(|monkey| parse_monkey(monkey))
            .collect();
    }

    fn read_input(input: &mut dyn BufRead) -> Vec<String> {
        return input.lines().map(Result::unwrap).collect();
    }

    fn parse_monkey(lines: &[String]) -> Monkey {
        let items = parse_starting_items(lines.get(1).unwrap());
        let (operation, operation_argument) = parse_operation(lines.get(2).unwrap());
        let divisible_by = parse_divisible_by(lines.get(3).unwrap());
        let monkey_to_throw_if_true = parse_monkey_to_throw_if_true(lines.get(4).unwrap());
        let monkey_to_throw_if_false = parse_monkey_to_throw_if_false(lines.get(5).unwrap());
        return Monkey { items, operation, operation_argument, divisible_by, monkey_to_throw_if_true, monkey_to_throw_if_false, inspected_items: 0 };
    }

    fn parse_starting_items(line: &String) -> Vec<u32> {
        return line.split_once(':').unwrap().1.trim()
            .split(", ").map(|i| i.parse::<u32>().unwrap())
            .collect();
    }

    fn parse_operation(line: &String) -> (char, String) {
        let tokens = line.split_once("=").unwrap().1.trim().split(' ').collect::<Vec<&str>>();
        return (tokens.get(1).unwrap().chars().nth(0).unwrap(), tokens.get(2).unwrap().to_string());
    }

    fn parse_divisible_by(line: &String) -> u32 {
        let re = Regex::new(r"  Test: divisible by (\d+)").unwrap();
        return re.captures(line).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();
    }

    fn parse_monkey_to_throw_if_true(line: &String) -> usize {
        let re = Regex::new(r"    If true: throw to monkey (\d+)").unwrap();
        return re.captures(line).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
    }

    fn parse_monkey_to_throw_if_false(line: &String) -> usize {
        let re = Regex::new(r"    If false: throw to monkey (\d+)").unwrap();
        return re.captures(line).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
    }

    struct Monkey {
        items: Vec<u32>,
        operation: char,
        operation_argument: String,
        divisible_by: u32,
        monkey_to_throw_if_true: usize,
        monkey_to_throw_if_false: usize,
        inspected_items: u32
    }

    impl Monkey {
        fn inspect_next_item(&mut self, relief: u32) -> (u32, usize) {
            let item = self.items.remove(0);
            let new_worry_level = self.perform_operation(item, self.operation, &self.operation_argument);
            let reduced_worry_level = self.reduce_worry_level(new_worry_level, relief);
            let monkey_to_throw_to = if self.passes_test(reduced_worry_level, self.divisible_by) {
                 self.monkey_to_throw_if_true
            }
            else {
                self.monkey_to_throw_if_false
            };
            self.inspected_items += 1;
            return (reduced_worry_level, monkey_to_throw_to);
        }

        fn perform_operation(&self, item: u32, operation: char, argument: &String) -> u32 {
            let resolved_argument = if argument.eq("old") {
                item
            }
            else {
                argument.parse::<u32>().unwrap()
            };
            
            return match operation {
                '+' => Ok(item + resolved_argument),
                '*' => Ok(item * resolved_argument),
                _ => Err(())
            }.unwrap();
        }

        fn reduce_worry_level(&self, worry_level: u32, relief: u32) -> u32 {
            return worry_level / relief;
        }

        fn passes_test(&self, worry_level: u32, divisible_by: u32) -> bool {
            return worry_level % divisible_by == 0;
        }
    }

    struct Game {
        monkeys: Vec<Monkey>,
        relief: u32
    }

    impl Game {
        fn play_round(&mut self) {
            for monkey_id in 0..self.monkeys.len() {
                self.make_turn(monkey_id);
            }
        }

        fn make_turn(&mut self, monkey_id: usize) {
            for _ in 0..self.monkeys.get(monkey_id).unwrap().items.len() {
                let (inspected_item, destination_monkey) = self.monkeys.get_mut(monkey_id).map(|m| m.inspect_next_item(self.relief)).unwrap();
                self.pass_item_to_monkey(inspected_item, destination_monkey);
            }
        }

        fn pass_item_to_monkey(&mut self, item: u32, destination_monkey: usize) {
            self.monkeys.get_mut(destination_monkey).unwrap().items.push(item);
        }

        fn level_of_monkey_business(&self) -> u32 {
            let mut inspections = self.monkeys.iter().map(|m| m.inspected_items).collect::<Vec<u32>>();
            inspections.sort();
            inspections.reverse();
            return inspections[0..2].iter().fold(1, |a, b| a * b);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day11;
    use std::{fs::File, io::BufReader};

    #[test]
    fn part1_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day11::level_of_monkey_business_after_rounds(&mut f, 20, 3), 10605);
    }

    #[test]
    fn part1_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day11::level_of_monkey_business_after_rounds(&mut f, 20, 3), 72884);
    }

    #[test]
    fn part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day11::level_of_monkey_business_after_rounds(&mut f, 10000, 1), 2713310158);
    }
}