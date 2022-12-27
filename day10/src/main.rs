mod day10 {
    use std::{io::BufRead, collections::HashMap};

    pub fn sum_of_signal_strengths(input: &mut dyn BufRead) -> i32 {
        let instructions = parse_input(input);
        let mut device = HandlheldDevice::new();
        device.set_watches(vec![20, 60, 100, 140, 180, 220]);

        for instruction in instructions.iter() {
            if instruction.starts_with("addx") {
                device.addx(instruction.split_once(' ').map(|s| s.1.parse::<i32>().unwrap()).unwrap());
            }
            else if instruction.eq("noop") {
                device.noop();
            }
        }

        return device.get_watched_values().iter().map(|(&cycle, &value)| cycle as i32 * value).sum();
    }

    pub fn render_image(input: &mut dyn BufRead) -> Vec<String> {
        let instructions = parse_input(input);
        let mut device = HandlheldDevice::new();
        device.set_watches(vec![20, 60, 100, 140, 180, 220]);

        for instruction in instructions.iter() {
            if instruction.starts_with("addx") {
                device.addx(instruction.split_once(' ').map(|s| s.1.parse::<i32>().unwrap()).unwrap());
            }
            else if instruction.eq("noop") {
                device.noop();
            }
        }

        return device.render_image();
    }
    
    fn parse_input(input: &mut dyn BufRead) -> Vec<String> {
        return input.lines().map(Result::unwrap).collect();
    }

    struct HandlheldDevice {
        clock: usize,
        x_register: i32,
        watches: Vec<usize>,
        watched_values: HashMap<usize, i32>,
        crt: Vec<Vec<char>>
    }

    impl HandlheldDevice {
        fn new() -> HandlheldDevice {
            return HandlheldDevice { clock: 0, x_register: 1, watches: Vec::new(), watched_values: HashMap::new(), crt: vec![vec![' '; 40]; 6]};
        }

        fn set_watches(&mut self, watches: Vec<usize>) {
            self.watches = watches;
        }

        fn addx(&mut self, arg: i32) {
            self.proceed_clock(2);
            self.x_register += arg;
        }

        fn noop(&mut self) {
            self.proceed_clock(1);
        }

        fn proceed_clock(&mut self, cycles: usize) {
            for _ in 0..cycles {
                self.clock += 1;
                self.watch_values();
                self.draw_pixel();
            }
        }

        fn watch_values(&mut self) {
            if self.watches.iter().find(|w| **w == self.clock).is_some() {
                self.watched_values.insert(self.clock, self.x_register);
            }
        }

        fn draw_pixel(&mut self) {
            if self.sprite_covers_current_pixel() {
                self.draw_lit_pixel();
            }
            else {
                self.draw_dark_pixel();
            }
        }

        fn sprite_covers_current_pixel(&self) -> bool {
            let sprite_position = self.x_register % 40;
            let sprite = [sprite_position - 1, sprite_position, sprite_position + 1];
            let current_pixel = self.get_current_pixel_coords().0 as i32;
            return sprite.contains(&current_pixel);
        }

        fn draw_lit_pixel(&mut self) {
            self.draw('#', self.get_current_pixel_coords());
        }

        fn draw_dark_pixel(&mut self) {
            self.draw('.', self.get_current_pixel_coords());
        }

        fn get_current_pixel_coords(&self) -> (usize, usize) {
            let y = (self.clock - 1) / 40;
            let x = (self.clock - 1) % 40;
            return (x, y);
        }

        fn draw(&mut self, pixel: char, coords: (usize, usize)) {
            self.crt.get_mut(coords.1).unwrap()[coords.0] = pixel;
        }

        fn get_watched_values(&self) -> &HashMap<usize, i32> {
            return &self.watched_values;
        }

        fn render_image(&self) -> Vec<String> {
            return self.crt.iter().map(|r| r.into_iter().collect()).collect();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day10;
    use std::{fs::File, io::BufReader, io::BufRead};

    #[test]
    fn part1_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day10::sum_of_signal_strengths(&mut f), 13140);
    }

    #[test]
    fn part1_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day10::sum_of_signal_strengths(&mut f), 14060);
    }

    #[test]
    fn part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        let rendered_image = day10::render_image(&mut f);
        let sample_image = BufReader::new(File::open("./sample.crt.out").unwrap()).lines().map(Result::unwrap).collect::<Vec<String>>();
        
        assert_eq!(rendered_image, sample_image);
    }

    #[test]
    fn part2_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        let rendered_image = day10::render_image(&mut f);
        
        for row in rendered_image {
            println!("{}", row);
        }

        // eight capital letters appearing on CRT: PAPKFKEJ
    }
}