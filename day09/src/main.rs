mod day09 {
    use std::io::BufRead;
    use itertools::Itertools;

    pub fn how_many_positions_does_the_tail_of_the_rope_visit_at_least_once(input: &mut dyn BufRead) -> usize {
        let moves = parse_input(input);

        let mut sim = Simulator::new();

        for m in moves.iter() {
            sim.proceed(m.0, m.1);
        }

        return sim.get_number_of_unique_positions_visited_by_tail();
    }

    fn parse_input(input: &mut dyn BufRead) -> Vec<(Direction, usize)> {
        return read_input(input).iter().map(|line| parse_move(line)).collect();
    }

    fn read_input(input: &mut dyn BufRead) -> Vec<String> {
        return input.lines().map(Result::unwrap).collect();
    }

    fn parse_move(line: &String) -> (Direction, usize) {
        return line.split_once(' ')
            .map(|(dir, dist)| (parse_direction(dir), parse_distance(dist)))
            .unwrap();
    }

    fn parse_direction(direction: &str) -> Direction {
        return match direction {
            "L" => Ok(Direction::L),
            "R" => Ok(Direction::R),
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            _ => Err(())
        }.unwrap();
    }

    fn parse_distance(distance: &str) -> usize {
        return distance.parse::<usize>().unwrap();
    }

    #[derive(Copy, Clone)]
    enum Direction {
        R,
        L,
        U,
        D
    }

    struct Simulator {
        head: (i32, i32),
        tail: (i32, i32),

        visited_positions_by_tail: Vec<(i32, i32)>
    }

    impl Simulator {
        fn new() -> Simulator {
            return Simulator { head: (0, 0), tail: (0, 0), visited_positions_by_tail: vec![(0, 0)] };
        }

        fn proceed(&mut self, direction: Direction, distance: usize) {
            match direction {
                Direction::L => self.move_head_left(distance),
                Direction::R => self.move_head_right(distance),
                Direction::U => self.move_head_up(distance),
                Direction::D => self.move_head_down(distance)
            }
        }

        fn move_head_left(&mut self, distance: usize) {
            for _ in 0..distance {
                self.head.0 -= 1;
                self.move_tail();
            }
        }

        fn move_head_right(&mut self, distance: usize) {
            for _ in 0..distance {
                self.head.0 += 1;
                self.move_tail();
            }
        }

        fn move_head_up(&mut self, distance: usize) {
            for _ in 0..distance {
                self.head.1 += 1;
                self.move_tail();
            }
        }

        fn move_head_down(&mut self, distance: usize) {
            for _ in 0..distance {
                self.head.1 -= 1;
                self.move_tail();
            }
        }

        fn move_tail(&mut self) {
            if !self.is_tail_adjacent_to_head() {
                let rx = self.head.0 - self.tail.0;
                let ry = self.head.1 - self.tail.1;

                if rx.abs() + ry.abs() >= 2 {
                    if rx != 0 {
                        self.tail.0 += rx / rx.abs();
                    }
                    if ry != 0 {
                        self.tail.1 += ry / ry.abs();
                    }
                } 

                self.memoize_tail_position();
            }
        }

        fn memoize_tail_position(&mut self) {
            self.visited_positions_by_tail.push((self.tail.0, self.tail.1));
        }

        fn is_tail_adjacent_to_head(&self) -> bool {
            let dx = (self.tail.0 - self.head.0).abs();
            let dy = (self.tail.1 - self.head.1).abs();

            return dx <= 1 && dy <= 1;
        }

        fn get_number_of_unique_positions_visited_by_tail(&self) -> usize {
            return self.visited_positions_by_tail.iter().unique().count();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day09;
    use std::{fs::File, io::BufReader};

    #[test]
    fn part1_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day09::how_many_positions_does_the_tail_of_the_rope_visit_at_least_once(&mut f), 13);
    }

    #[test]
    fn part1_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day09::how_many_positions_does_the_tail_of_the_rope_visit_at_least_once(&mut f), 6470);
    }
}
