mod day11 {
    use std::io::BufRead;
    use regex::Regex;

    pub fn level_of_monkey_business_after_rounds(input: &mut dyn BufRead, rounds: usize, relief: Option<u128>) -> u128 {
        let monkeys = parse_input(input);
        let mut game = Game { monkeys, relief };

        for r in 0..rounds {
            println!("Round #{}", r);
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
        let (operator, operand) = parse_operation(lines.get(2).unwrap());
        let divisible_by = parse_divisible_by(lines.get(3).unwrap());
        let monkey_to_throw_if_true = parse_monkey_to_throw_if_true(lines.get(4).unwrap());
        let monkey_to_throw_if_false = parse_monkey_to_throw_if_false(lines.get(5).unwrap());
        return Monkey { items, operator, operand, divisible_by, monkey_to_throw_if_true, monkey_to_throw_if_false, inspected_items: 0 };
    }

    fn parse_starting_items(line: &String) -> Vec<Item> {
        return line.split_once(':').unwrap().1.trim()
            .split(", ").map(|i| i.parse::<u128>().unwrap())
            .map(|v| Item::new(v))
            .collect();
    }

    fn parse_operation(line: &String) -> (Operator, Option<u128>) {
        let tokens = line.split_once("=").unwrap().1.trim().split(' ').collect::<Vec<&str>>();
        
        let operator_char = tokens.get(1).unwrap().chars().nth(0).unwrap();
        let operator = match operator_char {
            '+' => Ok(Operator::ADD),
            '*' => Ok(Operator::MUL),
            _ => Err(())
        }.unwrap();
        
        let operand_token = tokens.get(2).unwrap().to_string();
        let operand = if operand_token.eq("old") {
            Option::None
        }
        else {
            Some(operand_token.parse::<u128>().unwrap())
        };

        return (operator, operand);
    }

    fn parse_divisible_by(line: &String) -> u128 {
        let re = Regex::new(r"  Test: divisible by (\d+)").unwrap();
        return re.captures(line).unwrap().get(1).unwrap().as_str().parse::<u128>().unwrap();
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
        items: Vec<Item>,
        operator: Operator,
        operand: Option<u128>,
        divisible_by: u128,
        monkey_to_throw_if_true: usize,
        monkey_to_throw_if_false: usize,
        inspected_items: u128
    }

    struct Item {
        initial_value: u128,
        operations: Vec<(Operator, Option<u128>)>
    }

    impl Item {
        fn new(initial_value: u128) -> Item {
            Item { initial_value, operations: Vec::new() }
        }

        fn add_operation(&mut self, operator: Operator, operand: Option<u128>) {
            self.operations.push((operator, operand));
        }

        fn add_relief(&mut self, relief: u128) {
            self.operations.push((Operator::DIV, Some(relief)));
        }

        fn perform_operations(&self, optimiser: Option<u128>) -> u128 {
            let mut value = self.initial_value;

            for (operator, operand) in self.operations.iter() {
                let operand = operand.unwrap_or(value);
                value = self.perform_operation(optimiser.map(|d| value % d).unwrap_or(value), operator, optimiser.map(|d| operand % d).unwrap_or(operand));
            };

            return value;
        }

        fn perform_operation(&self, value: u128, operator: &Operator, operand: u128) -> u128 {
            return match operator {
                Operator::ADD => value + operand,
                Operator::MUL => value * operand,
                Operator::DIV if operand == 0 => 0,
                Operator::DIV => value / operand
            };
        }
    }

    #[derive(Clone, Copy)]
    enum Operator {
        ADD,
        MUL,
        DIV
    }

    impl Monkey {
        fn inspect_next_item(&mut self, relief: Option<u128>) -> (Item, usize) {
            let mut item = self.items.remove(0);

            item.add_operation(self.operator, self.operand);
            if let Some(r) = relief {
                item.add_relief(r);
            }

            let monkey_to_throw_to = if self.passes_test(&item, relief) {
                 self.monkey_to_throw_if_true
            }
            else {
                self.monkey_to_throw_if_false
            };

            self.inspected_items += 1;

            return (item, monkey_to_throw_to);
        }

        fn passes_test(&self, item: &Item, relief: Option<u128>) -> bool {
            let optimiser: Option<u128> = if relief.is_none() { Some(self.divisible_by) } else { None };
            let worry_level = item.perform_operations(optimiser);
            return worry_level % self.divisible_by == 0;
        }
    }

    struct Game {
        monkeys: Vec<Monkey>,
        relief: Option<u128>
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
                self.throw_item_to_monkey(inspected_item, destination_monkey);
            }
        }

        fn throw_item_to_monkey(&mut self, item: Item, destination_monkey: usize) {
            self.monkeys.get_mut(destination_monkey).unwrap().items.push(item);
        }

        fn level_of_monkey_business(&self) -> u128 {
            let mut inspections = self.monkeys.iter().map(|m| m.inspected_items).collect::<Vec<u128>>();
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
        assert_eq!(day11::level_of_monkey_business_after_rounds(&mut f, 20, Some(3)), 10605);
    }

    #[test]
    fn part1_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day11::level_of_monkey_business_after_rounds(&mut f, 20, Some(3)), 72884);
    }

    #[test]
    fn part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day11::level_of_monkey_business_after_rounds(&mut f, 10000, None), 2713310158);
    }

    #[test]
    fn part2_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day11::level_of_monkey_business_after_rounds(&mut f, 10000, None), 15310845153);
    }
}