mod day15 {
    use std::{io::BufRead, cmp};
    use itertools::Itertools;
    use regex::Regex;

    pub fn how_many_positions_cannot_contain_a_beacon_in_the_row(input: &mut dyn BufRead, y: isize) -> usize {       
        let sensors = parse_input(input);
        let known_beacons = sensors.iter()
            .map(|s| s.closest_beacon_coords)
            .unique()
            .collect::<Vec<(isize, isize)>>();

        let mut min_x: isize = isize::MAX;
        let mut max_x: isize = isize::MIN;
        let mut min_y: isize = isize::MAX;
        let mut max_y: isize = isize::MIN;

        for s in sensors.iter() {
            let distance = s.get_manhattan_distance_from_closest_beacon();
            min_x = cmp::min(min_x, s.coords.0 - distance);
            max_x = cmp::max(max_x, s.coords.0 + distance);
            min_y = cmp::min(min_y, s.coords.1 - distance);
            max_y = cmp::max(max_y, s.coords.1 + distance);
        }

        let reserved = (min_x..=max_x).map(|x| (x, y))
            .filter(|coords| !known_beacons.contains(coords))
            .filter(|coords| is_sensors_area(&sensors, coords))
            .collect::<Vec<(isize, isize)>>();
        
        return reserved.len();
    }

    pub fn distress_beacon_tuning_frequency(input: &mut dyn BufRead, min_coord: isize, max_coord: isize) -> Option<isize> {
        let sensors = parse_input(input);

        let mut min_x: isize = isize::MAX;
        let mut max_x: isize = isize::MIN;
        let mut min_y: isize = isize::MAX;
        let mut max_y: isize = isize::MIN;

        for s in sensors.iter() {
            let distance = s.get_manhattan_distance_from_closest_beacon();
            min_x = cmp::min(min_x, s.coords.0 - distance);
            max_x = cmp::max(max_x, s.coords.0 + distance);
            min_y = cmp::min(min_y, s.coords.1 - distance);
            max_y = cmp::max(max_y, s.coords.1 + distance);
        }

        for y in min_coord..max_coord {
            if let Some(beacon_x) = find_potential_beacons(&sensors, y, min_coord, max_coord) {
                return Some(beacon_x.0 * 4000000 + y);
            }
        }

        return None;
    }

    fn is_sensors_area(sensors: &Vec<Sensor>, coords: &(isize, isize)) -> bool {
        return sensors.iter().any(|s| s.is_in_covered_area(coords));
    }

    fn find_potential_beacons(sensors: &Vec<Sensor>, y: isize, min_x: isize, max_x: isize) -> Option<(isize, isize)> {
        let mut covered_regions = sensors.iter()
            .filter(|s| (s.coords.1 - y).abs() <= s.get_manhattan_distance_from_closest_beacon())
            .map(|s| (s, (s.coords.0 - (s.get_manhattan_distance_from_closest_beacon() - (s.coords.1 - y).abs()).abs(), s.coords.0 + (s.get_manhattan_distance_from_closest_beacon() - (s.coords.1 - y).abs()).abs())) )
            .map(|(s, (from, to))| (s, (bound_coord(from, min_x, max_x), bound_coord(to, min_x, max_x))))
            .collect::<Vec<(&Sensor, (isize, isize))>>();

        covered_regions.sort_by(|a, b| a.1.0.cmp(&b.1.0));

        let mut from = min_x;

        for reg in covered_regions.iter() {
            if reg.1.0 > from {
                return Some((from + 1, reg.1.0));
            }
            else {
                if reg.1.1 > from {
                    from = reg.1.1;
                }
            }
        }

        return None;
    }

    fn bound_coord(c: isize, min_coord: isize, max_coord: isize) -> isize {
        let left_ok = if c < min_coord { min_coord } else { c };
        return if left_ok > max_coord { max_coord } else { left_ok };
    }

    fn parse_input(input: &mut dyn BufRead) -> Vec<Sensor> {
        return read_input(input).iter().map(parse_sensor).collect();
    }

    fn read_input(input: &mut dyn BufRead) -> Vec<String> {
        return input.lines().map(Result::unwrap).collect();
    }

    fn parse_sensor(line: &String) -> Sensor {
        let re = Regex::new(r"Sensor at x=([-]?\d+), y=([-]?\d+): closest beacon is at x=([-]?\d+), y=([-]?\d+)").unwrap();
        let captures = re.captures(line).unwrap();
        return Sensor { 
            coords: (captures.get(1).unwrap().as_str().parse::<isize>().unwrap(), captures.get(2).unwrap().as_str().parse::<isize>().unwrap()), 
            closest_beacon_coords: (captures.get(3).unwrap().as_str().parse::<isize>().unwrap(), captures.get(4).unwrap().as_str().parse::<isize>().unwrap())
        }
    }

    struct Sensor {
        coords: (isize, isize),
        closest_beacon_coords: (isize, isize)
    }

    impl Sensor {
        fn get_manhattan_distance_from_closest_beacon(&self) -> isize {
            return self.get_manhattan_distance_to_coords(&self.closest_beacon_coords);
        }

        fn is_in_covered_area(&self, coords: &(isize, isize)) -> bool {
            return self.get_manhattan_distance_to_coords(coords) <= self.get_manhattan_distance_from_closest_beacon();
        }

        fn get_manhattan_distance_to_coords(&self, coords: &(isize, isize)) -> isize {
            return (self.coords.0 - coords.0).abs() + (self.coords.1 - coords.1).abs(); 
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day15;
    use std::{fs::File, io::BufReader};

    #[test]
    fn part1_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day15::how_many_positions_cannot_contain_a_beacon_in_the_row(&mut f, 10), 26);
    }

    #[test]
    fn part1_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day15::how_many_positions_cannot_contain_a_beacon_in_the_row(&mut f, 2000000), 6078701);
    }

    #[test]
    fn part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day15::distress_beacon_tuning_frequency(&mut f, 0, 20).unwrap(), 56000011);
    }

    #[test]
    fn part2_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day15::distress_beacon_tuning_frequency(&mut f, 0, 4000000).unwrap(), 12567351400528);
    }
}
