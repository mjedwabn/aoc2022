mod day12 {
    use std::{io::BufRead, collections::{HashMap, HashSet}};

    const EDGE_COST: usize = 1;

    pub fn fewest_steps_required_to_reach_best_signal_location_from_current_position(input: &mut dyn BufRead) -> usize {
        let raw_grid = parse_input(input);
        let digital_grid = raw_grid.digitize();
        let current_coords = raw_grid.get_current_position_coords();
        let best_signal_coords = raw_grid.get_best_signal_position_coords();

        let mut navigation = Navigation::new(digital_grid);
        navigation.analyze_routes(best_signal_coords);

        return navigation.find_shortest_path_length(&current_coords);
    }

    pub fn fewest_steps_required_to_reach_best_signal_location_from_any_square_at_elevation_a(input: &mut dyn BufRead) -> usize {
        let raw_grid = parse_input(input);
        let digital_grid = raw_grid.digitize();
        let coords_at_elevation_a = digital_grid.find_coords_at_elevation(raw_grid.digitize_position('a'));
        let best_signal_coords = raw_grid.get_best_signal_position_coords();

        let mut navigation = Navigation::new(digital_grid);
        navigation.analyze_routes(best_signal_coords);

        return coords_at_elevation_a.iter()
            .map(|c| navigation.find_shortest_path_length(c))
            .min().unwrap_or(usize::MAX);
    }

    fn parse_input(input: &mut dyn BufRead) -> Grid<char> {
        let raw_grid = read_input(input).iter()
            .map(|line| parse_line(line))
            .collect::<Vec<Vec<char>>>();
        return Grid::new(raw_grid);
    }

    fn parse_line(line: &String) -> Vec<char> {
        return line.chars().collect();
    }

    fn read_input(input: &mut dyn BufRead) -> Vec<String> {
        return input.lines().map(Result::unwrap).collect();
    }

    struct Grid<T> {
        grid: Vec<Vec<T>>,
        height: usize,
        width: usize
    }

    impl<T> Grid<T> {
        fn new(grid: Vec<Vec<T>>) -> Grid<T> {
            return Grid { height: grid.len(), width: grid.get(0).unwrap().len(), grid }
        }

        fn get_coords(&self) -> Vec<(usize, usize)> {
            return self.grid.iter()
                .zip(0..self.grid.len())
                .flat_map(|(row, y)| row.iter().zip(0..row.len()).map(move |(_, x)| (x, y)))
                .collect();
        }

        fn get_position(&self, x: usize, y: usize) -> &T {
            return self.grid.get(y).and_then(|row| row.get(x)).unwrap();
        }
    }

    impl Grid<char> {
        fn get_current_position_coords(&self) -> (usize, usize) {
            return self.find_position_coords('S');
        }
    
        fn get_best_signal_position_coords(&self) -> (usize, usize) {
            return self.find_position_coords('E');
        }
    
        fn find_position_coords(&self, position: char) -> (usize, usize) {
            return self.get_coords().iter()
                .find(|(x, y)| *self.get_position(*x, *y) == position)
                .map(|(x, y)| (*x, *y))
                .unwrap();
        }

        fn digitize(&self) -> Grid<usize> {
            return Grid::new(self.grid.iter()
                .map(|line| self.digitize_line(line))
                .collect());
        }

        fn digitize_line(&self, line: &Vec<char>) -> Vec<usize> {
            return line.iter().map(|p| self.digitize_position(*p)).collect();
        }

        fn digitize_position(&self, position: char) -> usize {
            let unified_position = match position {
                'S' => 'a',
                'E' => 'z',
                _ => position
            };
            return unified_position as usize - 'a' as usize;
        }
    }

    impl Grid<usize> {
        fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
            return vec![
                if x > 0 { Some((x - 1, y)) } else { None },
                if y < self.height - 1 { Some((x, y + 1)) } else { None },
                if x < self.width - 1 { Some((x + 1, y)) } else { None },
                if y > 0 { Some((x, y - 1)) } else { None }
            ].iter().filter(|c| c.is_some()).map(|c| c.unwrap()).collect();
        }

        fn find_coords_at_elevation(&self, elevation: usize) -> Vec<(usize, usize)> {
            return self.get_coords().iter()
                .map(|(x, y)| ((x, y), self.get_position(*x, *y)))
                .filter(|(_, e)| **e == elevation)
                .map(|((x, y), _)| (*x, *y))
                .collect();
        }
    }

    struct Navigation {
        grid: Grid<usize>,
        graph: HashMap<(usize, usize), HashSet<(usize, usize)>>,
        distances: HashMap<(usize, usize), usize>
    }

    impl Navigation {
        fn new(grid: Grid<usize>) -> Navigation {
            return Navigation { graph: GraphBuilder::new(&grid).build(), grid, distances: HashMap::new() };
        }

        fn analyze_routes(&mut self, to: (usize, usize)) {
            let mut predecessors: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
            let mut to_check = self.grid.get_coords().iter().map(|v| *v).collect::<HashSet<(usize, usize)>>();

            self.distances.insert(to, 0);

            for _ in 0..to_check.len() {
                let mut nodes: Vec<((usize, usize), &usize)> = to_check.iter()
                    .filter(|q| self.distances.contains_key(q))
                    .map(|q| (*q, self.distances.get(q).unwrap()))
                    .collect();
                nodes.sort_by(|a, b| a.1.cmp(b.1));
                
                if let Some(nearest_node) = nodes.get(0).map(|q| q.0) {
                    to_check.remove(&nearest_node);

                    for neighbour in self.graph.get(&nearest_node).unwrap() {
                        let existing_distance = self.distances.get(neighbour);
                        let candidate_distance = *self.distances.get(&nearest_node).unwrap();
                        if existing_distance.map(|d| *d > candidate_distance + EDGE_COST).unwrap_or(true) {
                            self.distances.insert(*neighbour, candidate_distance + EDGE_COST);
                            predecessors.insert(*neighbour, nearest_node);
                        }
                    }
                }
            }
        }
        
        fn find_shortest_path_length(&self, from: &(usize, usize)) -> usize {
            return self.distances.get(from).map(|d| *d).unwrap_or(usize::MAX);
        }
    }

    struct GraphBuilder<'a> {
        grid: &'a Grid<usize>
    }

    impl GraphBuilder<'_> {
        fn new(grid: &Grid<usize>) -> GraphBuilder {
            return GraphBuilder { grid }
        }

        fn build(&self) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
            let mut graph: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
            
            for src in self.grid.get_coords() {
                if !graph.contains_key(&src) {
                    graph.insert(src, HashSet::new());
                }
                for neighbour in self.get_reachable_neighbours(src) {
                    graph.get_mut(&src).unwrap().insert(neighbour);
                }
            }

            return graph;
        }

        fn get_reachable_neighbours(&self, dst: (usize, usize)) -> Vec<(usize, usize)> {
            return self.grid.get_neighbours(dst.0, dst.1).iter()
                .filter(|src| self.is_reachable(**src, dst))
                .map(|c| *c)
                .collect();
        }

        fn is_reachable(&self, source: (usize, usize), destination: (usize, usize)) -> bool {
            let src_position = *self.grid.get_position(source.0, source.1) as isize;
            let dst_position = *self.grid.get_position(destination.0, destination.1) as isize;
            return src_position == dst_position - 1 || src_position >= dst_position;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day12;
    use std::{fs::File, io::BufReader};

    #[test]
    fn part1_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day12::fewest_steps_required_to_reach_best_signal_location_from_current_position(&mut f), 31);
    }

    #[test]
    fn part1_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day12::fewest_steps_required_to_reach_best_signal_location_from_current_position(&mut f), 423);
    }

    #[test]
    fn part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day12::fewest_steps_required_to_reach_best_signal_location_from_any_square_at_elevation_a(&mut f), 29);
    }

    #[test]
    fn part2_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day12::fewest_steps_required_to_reach_best_signal_location_from_any_square_at_elevation_a(&mut f), 416);
    }
}