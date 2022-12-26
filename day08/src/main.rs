mod day08 {
    use std::io::BufRead;
    use take_until::TakeUntilExt;

    pub fn how_many_trees_are_visible_from_outside_the_grid(input: &mut dyn BufRead) -> usize {
        let forest = Forest{grid: parse_input(input)};
        return forest.how_many_trees_are_visible_from_outside_the_grid();
    }

    pub fn what_is_the_highest_scenic_score_possible_for_any_tree(input: &mut dyn BufRead) -> u32 {
        let forest = Forest{grid: parse_input(input)};
        return forest.what_is_the_highest_scenic_score_possible_for_any_tree();
    }

    fn parse_input(input: &mut dyn BufRead) -> Vec<Vec<u8>> {
        let lines = read_input(input);
        return lines.iter().map(|line| parse_trees(line)).collect();
    }

    fn read_input(input: &mut dyn BufRead) -> Vec<String> {
        return input.lines().map(|line| line.unwrap()).collect();
    }

    fn parse_trees(line: &String) -> Vec<u8> {
        return line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
    }

    struct Forest {
        grid: Vec<Vec<u8>>
    }

    impl Forest {
        fn how_many_trees_are_visible_from_outside_the_grid(&self) -> usize {
            return self.get_trees_coords().iter()
                .filter(|(x, y)| self.is_visible_from_outside(*x, *y))
                .count();
        }

        pub fn what_is_the_highest_scenic_score_possible_for_any_tree(&self) -> u32 {
            return self.get_trees_coords().iter()
                .map(|(x, y)| self.calculate_scenic_score(*x, *y))
                .max().unwrap_or(0);
        }

        fn get_trees_coords(&self) -> Vec<(usize, usize)> {
            return (0..self.grid.len())
                .flat_map(|y| (0..self.grid.get(y).unwrap().len()).map(move |x| (x, y)))
                .collect();
        }

        fn calculate_scenic_score(&self, x: usize, y: usize) -> u32 {
            let row = self.get_row(y);
            let col = self.get_column(x);
            return self.get_viewing_distance(x, &row) * self.get_viewing_distance(y, &col);
        }

        fn get_viewing_distance(&self, position: usize, series: &Vec<u8>) -> u32 {
            let height = series.get(position).unwrap();
            return (0..position).rev().take_until(|p| series.get(*p).unwrap() >= height).count() as u32
                * (position+1..series.len()).take_until(|p| series.get(*p).unwrap() >= height).count() as u32;
        }

        fn is_visible_from_outside(&self, x: usize, y: usize) -> bool {
            return self.is_on_edge(x, y) || self.is_visible_in_interior(x, y);
        }

        fn is_on_edge(&self, x: usize, y: usize) -> bool {
            return x == 0 || x == self.grid.get(y).unwrap().len()
                || y == 0 || y == self.grid.len();
        }

        fn is_visible_in_interior(&self, x: usize, y: usize) -> bool {
            let row: Vec<u8> = self.get_row(y);
            let column: Vec<u8> = self.get_column(x);
            return self.is_visible_in_series_interior(x, &row) 
                || self.is_visible_in_series_interior(y, &column);
        }

        fn get_row(&self, y: usize) -> Vec<u8> {
            return self.grid.get(y).unwrap().to_vec();
        }

        fn get_column(&self, x: usize) -> Vec<u8> {
            return (0..self.grid.len())
                .map(|yy| self.grid.get(yy).unwrap().get(x).unwrap())
                .map(|v| *v)
                .collect();
        }

        fn is_visible_in_series_interior(&self, position: usize, series: &Vec<u8>) -> bool {
            let height = series.get(position).unwrap();
            return (0..position).all(|p| series.get(p).unwrap() < height)
                || (position + 1..series.len()).all(|p| series.get(p).unwrap() < height);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day08;
    use std::{fs::File, io::BufReader};

    #[test]
    fn part1_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day08::how_many_trees_are_visible_from_outside_the_grid(&mut f), 21);
    }

    #[test]
    fn part1_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day08::how_many_trees_are_visible_from_outside_the_grid(&mut f), 1779);
    }

    #[test]
    fn part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day08::what_is_the_highest_scenic_score_possible_for_any_tree(&mut f), 8);
    }

    #[test]
    fn part2_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day08::what_is_the_highest_scenic_score_possible_for_any_tree(&mut f), 172224);
    }
}
