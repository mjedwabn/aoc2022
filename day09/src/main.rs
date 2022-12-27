mod day09 {
    use std::io::BufRead;
    use itertools::Itertools;

    pub fn how_many_positions_does_the_tail_of_the_rope_visit_at_least_once(input: &mut dyn BufRead, number_of_knots: usize) -> usize {
        let moves = parse_input(input);

        let mut sim = Simulator::new(number_of_knots);

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
        knots: Vec<(i32, i32)>,
        visited_positions_by_tail: Vec<(i32, i32)>
    }

    impl Simulator {
        fn new(number_of_knots: usize) -> Simulator {
            return Simulator { knots: vec![(0, 0); number_of_knots], visited_positions_by_tail: vec![(0, 0)] };
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
                self.head().0 -= 1;
                self.move_knots();
            }
        }

        fn move_head_right(&mut self, distance: usize) {
            for _ in 0..distance {
                self.head().0 += 1;
                self.move_knots();
            }
        }

        fn move_head_up(&mut self, distance: usize) {
            for _ in 0..distance {
                self.head().1 += 1;
                self.move_knots();
            }
        }

        fn move_head_down(&mut self, distance: usize) {
            for _ in 0..distance {
                self.head().1 -= 1;
                self.move_knots();
            }
        }

        fn move_knots(&mut self) {
            for index in 1..self.knots.len() {
                if !self.is_knot_adjacent_to_predecessor(index) {
                    let knot = self.get_knot(index);
                    let predecessor = self.get_knot(index - 1);
                    let rx = predecessor.0 - knot.0;
                    let ry = predecessor.1 - knot.1;

                    if rx.abs() + ry.abs() >= 2 {
                        let dx = if rx != 0 { rx / rx.abs() } else { 0 };
                        let dy = if ry != 0 { ry / ry.abs() } else { 0 };
                        self.move_knot(index, (dx, dy));
                    } 

                    if self.is_tail(index) {
                        self.memoize_tail_position();
                    }
                }
            }
        }

        fn move_knot(&mut self, knot_index: usize, delta: (i32, i32)) {
            let mut knot = self.knots.get_mut(knot_index).unwrap();
            knot.0 += delta.0;
            knot.1 += delta.1;
        }

        fn is_tail(&self, knot_index: usize) -> bool {
            return self.knots.len() - 1 == knot_index;
        }

        fn memoize_tail_position(&mut self) {
            let x = self.tail().0;
            let y = self.tail().1;
            self.visited_positions_by_tail.push((x, y));
        }

        fn is_knot_adjacent_to_predecessor(&self, knot_index: usize) -> bool {
            let knot = self.get_knot(knot_index);
            let precedessor = self.get_knot(knot_index - 1);

            let dx = (knot.0 - precedessor.0).abs();
            let dy = (knot.1 - precedessor.1).abs();

            return dx <= 1 && dy <= 1;
        }

        fn get_knot(&self, index: usize) -> &(i32, i32) {
            return self.knots.get(index).unwrap();
        }

        fn head(&mut self) -> &mut(i32, i32) {
            return self.knots.get_mut(0).unwrap();
        }

        fn tail(&mut self) -> &mut(i32, i32) {
            let last = self.knots.len() - 1;
            return self.knots.get_mut(last).unwrap();
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
        assert_eq!(day09::how_many_positions_does_the_tail_of_the_rope_visit_at_least_once(&mut f, 2), 13);
    }

    #[test]
    fn part1_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day09::how_many_positions_does_the_tail_of_the_rope_visit_at_least_once(&mut f, 2), 6470);
    }

    #[test]
    fn part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day09::how_many_positions_does_the_tail_of_the_rope_visit_at_least_once(&mut f, 10), 1);
    }

    #[test]
    fn part2_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day09::how_many_positions_does_the_tail_of_the_rope_visit_at_least_once(&mut f, 10), 2658);
    }
}
