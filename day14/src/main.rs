mod day14 {
    use std::{io::BufRead, cmp, vec};

    pub fn how_many_units_of_sand_come_to_rest_before_sand_starts_flowing_into_the_abyss_below(input: &mut dyn BufRead) -> usize {
        let mut cave = CaveBuilder::new()
            .with_rocks(parse_input(input))
            .with_sand_source(500, 0)
            .build();

        cave.stabilize();

        return cave.count_sand_at_rest();
    }

    pub fn how_many_units_of_sand_come_to_rest_before_sand_of_source_becomes_blocked(input: &mut dyn BufRead) -> usize {
        let mut cave = CaveBuilder::new()
            .with_rocks(parse_input(input))
            .with_sand_source(500, 0)
            .with_infinite_horizonal_floor(2)
            .build();

        cave.stabilize();

        return cave.count_sand_at_rest();
    }

    fn parse_input(input: &mut dyn BufRead) -> Vec<Vec<(usize, usize)>> {
        return read_input(input).iter().map(parse_rock_path).collect();
    }

    fn read_input(input: &mut dyn BufRead) -> Vec<String> {
        return input.lines().map(Result::unwrap).collect();
    }

    fn parse_rock_path(line: &String) -> Vec<(usize, usize)> {
        return line.split(" -> ").map(parse_coords).collect();
    }

    fn parse_coords(coords: &str) -> (usize, usize) {
        return coords.split_once(',')
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            .unwrap();
    }

    struct Cave {
        grid: CartesianGrid<Object>,
        sand_source: (usize, usize)
    }

    impl Cave {
        fn print(&self) {
            println!("--Cave--");
            self.grid.print();
        }

        fn stabilize(&mut self) {
            loop {
                // self.print();
                let sand_tile = self.generate_sand_tile();
                if sand_tile.is_none() {
                    break;
                }
            }

            self.print();
        }

        fn generate_sand_tile(&mut self) -> Option<(usize, usize)> {
            let mut sand_tile = self.sand_source;

            loop {
                match self.analyze_drop_moves(&sand_tile) {
                    Ok(coords) => {
                        sand_tile = coords;
                    },
                    Err(TileError::Blocked) => {
                        self.grid.set(sand_tile.0, sand_tile.1, Object::Sand);
                        break;
                    },
                    Err(TileError::Void) => {
                        return None;
                    }
                }
            }

            if sand_tile == self.sand_source {
                return None;
            }

            return Some(sand_tile);
        }

        fn analyze_drop_moves(&self, coords: &(usize, usize)) -> Result<(usize, usize), TileError> {
            for drop_move in self.drop_moves(coords) {
                let possible_move = match drop_move {
                    Ok(coords) => Some(Ok(coords)),
                    Err(TileError::Blocked) => None,
                    Err(TileError::Void) => Some(Err(TileError::Void))
                };

                if let Some(m) = possible_move {
                    return m;
                }
            }

            return Err(TileError::Blocked);
        }

        fn drop_moves(&self, coords: &(usize, usize)) -> Vec<Result<(usize, usize), TileError>> {
            let drops = vec![
                self.grid.get_down_neighbour(coords),
                self.grid.get_down_left_neighbour(coords),
                self.grid.get_down_right_neighbour(coords)
            ];

            return drops.iter().map(|d|self.analyze_neighbour(*d)).collect();
        }

        fn analyze_neighbour(&self, coords: Option<(usize, usize)>) -> Result<(usize, usize), TileError> {
            return match coords {
                Some(neighbour) => 
                    if self.is_tile_available(&neighbour) {
                        Result::Ok(neighbour)
                    }
                    else {
                        Result::Err(TileError::Blocked)
                    },
                None => Result::Err(TileError::Void)
            };
        }

        fn is_tile_available(&self, coords: &(usize, usize)) -> bool {
            return *self.grid.get(coords) == Object::Air;
        }

        fn count_sand_at_rest(&self) -> usize {
            return self.grid.coords().iter()
                .filter(|coords| *self.grid.get(coords) == Object::Sand)
                .count();
        }
    }

    #[derive(Debug)]
    enum TileError {
        Blocked,
        Void
    }

    struct CaveBuilder {
        rock_paths: Vec<Vec<(usize, usize)>>,
        sand_source: (usize, usize),
        infinite_horizontal_floor_elevation_offset: Option<usize>
    }

    impl CaveBuilder {
        fn new() -> CaveBuilder {
            return CaveBuilder{ rock_paths: vec![], sand_source: (500, 0), infinite_horizontal_floor_elevation_offset: None };
        }

        fn with_rocks(&mut self, rock_paths: Vec<Vec<(usize, usize)>>) -> &mut CaveBuilder {
            self.rock_paths = rock_paths;
            return self;
        }

        fn with_sand_source(&mut self, x: usize, y: usize) -> &mut CaveBuilder {
            self.sand_source = (x, y);
            return self;
        }

        fn with_infinite_horizonal_floor(&mut self, y_offset: usize) -> &mut CaveBuilder {
            self.infinite_horizontal_floor_elevation_offset = Some(y_offset);
            return self;
        }

        fn build(&self) -> Cave {
            let mut coords = self.rock_paths.iter()
                .flat_map(|row| row)
                .collect::<Vec<&(usize, usize)>>();
            coords.push(&self.sand_source);

            let min_y = coords.iter().map(|(_, y)| y).min().unwrap();
            let max_y = coords.iter().map(|(_, y)| y).max().map(|max| max + self.infinite_horizontal_floor_elevation_offset.unwrap_or(0)).unwrap();
            let height = (max_y - min_y) as usize;

            let translate_y = |y: usize| (height as isize - (max_y as isize - y as isize)) as usize;

            let min_x_candidate = coords.iter().map(|(x, _)| x).min().unwrap();
            let min_x = self.infinite_horizontal_floor_elevation_offset.map(|_| cmp::min(*min_x_candidate, self.sand_source.0 - height - 3) as isize).unwrap_or(*min_x_candidate as isize);
            let max_x_candidate = coords.iter().map(|(x, _)| x).max().unwrap();
            let max_x = self.infinite_horizontal_floor_elevation_offset.map(|_| cmp::max(*max_x_candidate, self.sand_source.0 + height + 3)).unwrap_or(*max_x_candidate);
            let width = (max_x as isize - min_x) as usize;

            let translate_x = |x: isize| (width as isize - (max_x as isize - x)) as usize;

            let mut grid = CartesianGrid { grid: vec![vec![Object::Air; width + 1]; height + 1] };

            for rock_path in self.rock_paths.iter() {
                for rock_line in rock_path.windows(2) {
                    let from = rock_line.get(0).unwrap();
                    let to = rock_line.get(1).unwrap();

                    for (x, y) in grid.get_continuous_coords(from, to) {
                        grid.set(translate_x(x as isize), translate_y(y), Object::Rock);
                    }
                }
            }

            if self.infinite_horizontal_floor_elevation_offset.is_some() {
                for (x, y) in grid.get_continuous_coords(&(0, height), &(width, height)) {
                    grid.set(x, y, Object::Rock);
                }
            }

            let translated_sand_source = (translate_x(self.sand_source.0 as isize), translate_y(self.sand_source.1));

            grid.set(translated_sand_source.0, translated_sand_source.1, Object::SourceOfSand);

            return Cave { grid, sand_source: translated_sand_source };
        }
    }

    #[derive(Clone, Copy, PartialEq)]
    enum Object {
        Air,
        Rock,
        Sand,
        SourceOfSand
    }

    impl Object {
        fn to_char(&self) -> char {
            return match self {
                Object::Air => '.',
                Object::Rock => '#',
                Object::Sand => 'o',
                Object::SourceOfSand => '+'
            }
        }
    }

    struct CartesianGrid<T> {
        grid: Vec<Vec<T>>
    }

    impl <T> CartesianGrid<T> {
        fn get(&self, coords: &(usize, usize)) -> &T {
            return self.grid.get(coords.1).unwrap().get(coords.0).unwrap();
        }

        fn set(&mut self, x: usize, y: usize, object: T) {
            self.grid.get_mut(y).unwrap()[x] = object;
        }

        fn coords(&self) -> Vec<(usize, usize)> {
            return (0..self.grid.len())
                .flat_map(|y| (0..self.grid.get(y).unwrap().len()).map(move |x| (x, y)))
                .collect();
        }

        fn get_continuous_coords(&self, from: &(usize, usize), to: &(usize, usize)) -> Vec<(usize, usize)> {
            if from.0 == to.0 {
                return (cmp::min(from.1, to.1)..=cmp::max(from.1, to.1)).map(|y| (from.0, y)).collect();
            }
            else {
                return (cmp::min(from.0, to.0)..=cmp::max(from.0, to.0)).map(|x| (x, from.1)).collect();
            }
        }

        fn get_down_neighbour(&self, coords: &(usize, usize)) -> Option<(usize, usize)> {
            if coords.1 + 1 < self.grid.len() {
                return Some((coords.0, coords.1 + 1));
            }
            else {
                return None;
            }
        }

        fn get_down_left_neighbour(&self, coords: &(usize, usize)) -> Option<(usize, usize)> {
            if coords.1 + 1 < self.grid.len() && coords.0 > 0 {
                return Some((coords.0 - 1, coords.1 + 1));
            }
            else {
                return None;
            }
        }

        fn get_down_right_neighbour(&self, coords: &(usize, usize)) -> Option<(usize, usize)> {
            if coords.1 + 1 < self.grid.len() && coords.0 + 1 < self.grid.get(coords.1 + 1).unwrap().len() {
                return Some((coords.0 + 1, coords.1 + 1));
            }
            else {
                return None;
            }
        }
    }

    impl CartesianGrid<Object> {
        fn print(&self) {
            for row in self.grid.iter() {
                for object in row {
                    print!("{}", object.to_char());
                }
                println!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day14;
    use std::{fs::File, io::BufReader};

    #[test]
    fn part1_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day14::how_many_units_of_sand_come_to_rest_before_sand_starts_flowing_into_the_abyss_below(&mut f), 24);
    }

    #[test]
    fn part1_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day14::how_many_units_of_sand_come_to_rest_before_sand_starts_flowing_into_the_abyss_below(&mut f), 665);
    }

    #[test]
    fn part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day14::how_many_units_of_sand_come_to_rest_before_sand_of_source_becomes_blocked(&mut f), 93);
    }

    #[test]
    fn part2_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day14::how_many_units_of_sand_come_to_rest_before_sand_of_source_becomes_blocked(&mut f), 25434);
    }
}