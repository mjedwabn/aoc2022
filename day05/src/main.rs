mod day05 {
    use std::io::BufRead;
    use std::str;
    use regex::Regex;

    pub fn get_crates_on_top_of_stacks_after_rearrangement(input: &mut dyn BufRead) -> String {
        let (stacks, procedure) = parse_input(input);
        // stacks.plot_crates();
        let mut crane: CrateMover9000 = CrateMover9000 { stacks: stacks };
        for step in procedure.steps {
            crane.rearrange(step);
            // stacks.plot_crates();
        }
        return crane.stacks.get_top_crates();
    }

    pub fn get_crates_on_top_of_stacks_after_rearrangement_using_crane_mover_9001(input: &mut dyn BufRead) -> String {
        let (stacks, procedure) = parse_input(input);
        // stacks.plot_crates();
        let mut crane: CrateMover9001 = CrateMover9001 { stacks: stacks };
        for step in procedure.steps {
            crane.rearrange(step);
            // stacks.plot_crates();
        }
        return crane.stacks.get_top_crates();
    }

    fn parse_input(input: &mut dyn BufRead) -> (Stacks, RearrangementProcedure) {
        let lines: Vec<String> = read_input(input);
        let segments = lines.split(|line| line == "").map(|ll| ll.iter().map(String::from).collect()).collect::<Vec<Vec<String>>>();
        let s1 = segments.first().unwrap();
        let stacks = parse_stacks(s1);
        let procedure = parse_rearrangament_procedure(segments.last().unwrap());
        return (stacks, procedure);
    }

    fn parse_stacks(lines: &[String]) -> Stacks {
        let stack_numbers = parse_stack_numbers(lines.last().unwrap());
        let stack_crates: Vec<Vec<Option<char>>> = lines[0..lines.len()-1].iter().map(|line| parse_crates(line)).collect();
        let stacks = stack_numbers.iter().map(|i| make_stack(*i, &stack_crates)).collect();
        return Stacks { stacks: stacks };
    }

    fn parse_stack_numbers(line: &String) -> Vec<usize> {
        return line.as_bytes()
            .chunks(4)
            .map(str::from_utf8)
            .map(Result::unwrap)
            .map(str::trim)
            .map(|x| x.parse::<usize>().map(|i| i - 1))
            .map(Result::unwrap)
            .collect();
    }

    fn parse_crates(line: &String) -> Vec<Option<char>> {
        return line.as_bytes()
            .chunks(4)
            .map(str::from_utf8)
            .map(Result::unwrap)
            .map(str::trim)
            .map(|x| x.chars().nth(1))
            .collect();
    }

    fn make_stack(i: usize, crates: &Vec<Vec<Option<char>>>) -> Vec<char> {
        return crates.iter().map(|c| c.get(i))
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .rev()
            .collect();
    }

    fn parse_rearrangament_procedure(lines: &[String]) -> RearrangementProcedure {
        let steps = lines.iter().map(|line| parse_procedure_step(line)).collect();
        return RearrangementProcedure { steps: steps };
    }

    fn parse_procedure_step(line: &String) -> Step {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let captures = re.captures(line).unwrap();
        let quantity = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to = captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
        return Step { quantity: quantity, from: from, to: to };
    }

    fn read_input(input: &mut dyn BufRead) -> Vec<String> {
        return input.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    }

    struct Stacks {
        stacks: Vec<Vec<char>>
    }

    struct CrateMover9000 {
        stacks: Stacks
    }
    struct CrateMover9001 {
        stacks: Stacks
    }

    trait CrateMover {
        fn rearrange(&mut self, step: Step);
    }

    impl CrateMover for CrateMover9000 {
        fn rearrange(&mut self, step: Step) {
            for _ in 0..step.quantity {
                let tmp = self.stacks.stacks.get_mut(step.from).unwrap()
                    .pop().unwrap();
                self.stacks.stacks.get_mut(step.to).unwrap().push(tmp);
            }
        }
    }

    impl CrateMover for CrateMover9001 {
        fn rearrange(&mut self, step: Step) {
            let mut tmp: Vec<char> = Vec::new();
            for _ in 0..step.quantity {
                tmp.push(self.stacks.stacks.get_mut(step.from).unwrap().pop().unwrap());
            }

            for _ in 0..step.quantity {
                self.stacks.stacks.get_mut(step.to).unwrap().push(tmp.pop().unwrap());
            }
        }
    }

    impl Stacks {
        fn get_top_crates(&self) -> String {
            return self.stacks.iter()
                .map(|stack| stack.last())
                .filter(|s| s.is_some())
                .map(|s| s.unwrap())
                .collect();
        }

        fn plot_crates(&self) {
            println!("---");
            for level in self.stacks.iter() {
                for c in level {
                    print!("[{}] ", c);
                }
                println!();
            }
        }
    }

    struct RearrangementProcedure {
        steps: Vec<Step>
    }

    struct Step {
        quantity: usize,
        from: usize,
        to: usize
    }
}

#[cfg(test)]
mod tests {
    use crate::day05;
    use std::{fs::File, io::BufReader};

    #[test]
    fn part1_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day05::get_crates_on_top_of_stacks_after_rearrangement(&mut f), "CMZ");
    }

    #[test]
    fn part1_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day05::get_crates_on_top_of_stacks_after_rearrangement(&mut f), "VJSFHWGFT");
    }

    #[test]
    fn part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day05::get_crates_on_top_of_stacks_after_rearrangement_using_crane_mover_9001(&mut f), "MCD");
    }

    #[test]
    fn part2_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day05::get_crates_on_top_of_stacks_after_rearrangement_using_crane_mover_9001(&mut f), "LCTQFBVZV");
    }
}
