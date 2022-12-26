mod day07 {
    use std::io::BufRead;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub fn sum_of_directories_of_size_at_most_100000(input: &mut dyn BufRead) -> u32 {
        let root = Rc::new(RefCell::new(FileNode::new()));
        let current = Rc::clone(&root);
        current.borrow_mut().name = "/".to_string();
        let root_ref = Rc::clone(&root);

        let lines = read_input(input);
        let mut i = OutputInterpreter {root: root_ref, current: current};
        for line in lines.iter() {
            i.interpret(line);
        }
        let mut fs = FileSystem {root: Rc::clone(&root), total_disk_space: 0};
        fs.calculate_sizes();
        fs.tree();
        return fs.find_sum_of_size_of_directories_with_a_size_of_at_most_100000();
    }

    pub fn find_smallest_directory_size_that_would_free_up_enough_space_for_update(input: &mut dyn BufRead) -> u32 {
        let root = Rc::new(RefCell::new(FileNode::new()));
        let current = Rc::clone(&root);
        current.borrow_mut().name = "/".to_string();
        let root_ref = Rc::clone(&root);

        let lines = read_input(input);
        let mut i = OutputInterpreter {root: root_ref, current: current};
        for line in lines.iter() {
            i.interpret(line);
        }
        let mut fs = FileSystem {root: Rc::clone(&root), total_disk_space: 70000000};
        fs.calculate_sizes();
        return fs.find_smallest_directory_size_that_would_free_up_enough_space(30000000);
    }

    fn read_input(input: &mut dyn BufRead) -> Vec<String> {
        return input.lines().map(Result::unwrap).collect();
    }

    struct OutputInterpreter {
        root: Rc<RefCell<FileNode>>,
        current: Rc<RefCell<FileNode>>
    }

    impl OutputInterpreter {
        fn interpret(&mut self, line: &String) {
            let first_char = line.chars().nth(0).unwrap();
            match first_char {
                '$' => self.interpret_command(line),
                _ => self.interpret_output(line)
            }
        }

        fn interpret_command(&mut self, line: &String) {
            let mut parts = line.split(' ').skip(1);
            let cmd = parts.next().unwrap();

            if cmd.eq("cd") {
                self.interpret_cd(parts.next().unwrap());
            }
        }

        fn interpret_output(&mut self, line: &String) {
            if line.starts_with("dir ") {
                let dir_name = line.split_once(" ").unwrap().1;
                self.add_dir(dir_name);
            }
            else {
                let (size, file_name) = line.split_once(" ").unwrap();
                self.add_file(file_name, size.parse::<u32>().unwrap());
            }
        }

        fn add_dir(&mut self, name: &str) {
            let child = Rc::new(RefCell::new(FileNode::new()));
            let mut child_mut = child.borrow_mut();
            child_mut.name = name.to_string();
            child_mut.parent = Some(Rc::clone(&self.current));

            self.current.borrow_mut().add_child(Rc::clone(&child));
        }

        fn add_file(&mut self, name: &str, size: u32) {
            let child = Rc::new(RefCell::new(FileNode::new()));
            let mut child_mut = child.borrow_mut();
            child_mut.name = name.to_string();
            child_mut.parent = Some(Rc::clone(&self.current));
            child_mut.size = Some(size);

            self.current.borrow_mut().add_child(Rc::clone(&child));
        }

        fn interpret_cd(&mut self, arg: &str) {
            match arg {
                ".." => self.move_out_one_level(),
                "/" => self.switch_to_outermost_directory(),
                _ => self.move_in_one_level(arg)
            }
        }

        fn move_out_one_level(&mut self) {
            let u = Rc::clone(&self.current.borrow().parent.as_ref().unwrap());
            self.current = u;
        }

        fn switch_to_outermost_directory(&mut self) {
            self.current = Rc::clone(&self.root);
        }

        fn move_in_one_level(&mut self, dir_name: &str) {
            let u = Rc::clone(&self.current.borrow().children.iter().find(|c| c.borrow().name.eq(dir_name)).unwrap());
            self.current = u;
        }
    }

    struct FileSystem {
        total_disk_space: u32,
        root: Rc<RefCell<FileNode>>
    }

    impl FileSystem {
        fn calculate_sizes(&mut self) {
            self.calculate_size(Rc::clone(&self.root));
        }

        fn calculate_size(&mut self, node: Rc<RefCell<FileNode>>) -> u32 {
            if node.borrow().children.len() > 0 {
                let u: u32 = node.borrow().children.iter().map(|c| self.calculate_size(Rc::clone(c))).sum();
                node.borrow_mut().size = Some(u);
                return node.borrow().size.unwrap();
            }
            else {
                return node.borrow().size.unwrap();
            }
        }

        fn find_sum_of_size_of_directories_with_a_size_of_at_most_100000(&self) -> u32 {
            return self.find_sum_of_size_of_directories_with_a_size_of_at_most(Rc::clone(&self.root), 100000);
        }

        fn find_sum_of_size_of_directories_with_a_size_of_at_most(&self, node: Rc<RefCell<FileNode>>, threshold: u32) -> u32 {
            let directories = self.collect_directories(node);
            let total_size = directories.iter().map(|d| d.borrow().size.unwrap()).filter(|s| *s <= threshold).sum();
            return total_size;
        }

        fn collect_directories(&self, node: Rc<RefCell<FileNode>>) -> Vec<Rc<RefCell<FileNode>>> {
            let mut nested = node.borrow().children.iter()
                .filter(|c| c.borrow().children.len() > 0)
                .flat_map(|c| self.collect_directories(Rc::clone(c)))
                .collect::<Vec<Rc<RefCell<FileNode>>>>();
            nested.push(node);
            return nested;
        }

        fn tree(&self) {
            self.tree_dir(Rc::clone(&self.root), 0);
        }

        fn tree_dir(&self, node: Rc<RefCell<FileNode>>, indent: usize) {
            println!("{}{} {}", " ".repeat(indent), node.borrow().name, node.borrow().size.unwrap());
            for n in node.borrow().children.iter() {
                self.tree_dir(Rc::clone(n), indent + 1)
            }
        }

        fn find_smallest_directory_size_that_would_free_up_enough_space(&self, free_at_latest: u32) -> u32 {
            let currently_unused_space = self.total_disk_space - self.root.borrow().size.unwrap();
            let missing_free_space = free_at_latest - currently_unused_space;

            let mut sizes = self.collect_directories(Rc::clone(&self.root)).iter()
                .map(|d| d.borrow().size.unwrap())
                .collect::<Vec<u32>>();
            sizes.sort();
            return sizes.iter().find(|s| **s > missing_free_space).map(|s| *s).unwrap_or(0);

        }
    }

    struct FileNode {
        name: String,
        size: Option<u32>,
        children: Vec<Rc<RefCell<FileNode>>>,
        parent: Option<Rc<RefCell<FileNode>>>
    }

    impl FileNode {
        pub fn new() -> FileNode {
            return FileNode { name: "".to_string(), size: None, children: vec![], parent: None }
        }

        pub fn add_child(&mut self, new_node: Rc<RefCell<FileNode>>) {
            self.children.push(new_node);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day07;
    use std::{fs::File, io::BufReader};

    #[test]
    fn part1_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day07::sum_of_directories_of_size_at_most_100000(&mut f), 95437)
    }

    #[test]
    fn part1_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day07::sum_of_directories_of_size_at_most_100000(&mut f), 1086293)
    }

    #[test]
    fn part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day07::find_smallest_directory_size_that_would_free_up_enough_space_for_update(&mut f), 24933642)
    }

    #[test]
    fn part2_day_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day07::find_smallest_directory_size_that_would_free_up_enough_space_for_update(&mut f), 366028)
    }
}
