use std::cell::RefCell;
use std::rc::Rc;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Data;

const THRESHOLD: usize = 100_000;
const TOTAL_SPACE: usize = 70_000_000;
const REQUIRED_SPACE: usize = 30_000_000;
                            //170_172_574

fn main() {
    let example_file = Data::get("example.txt").unwrap();
    let example_data = std::str::from_utf8(example_file.data.as_ref()).unwrap();
    assert_eq!(95437, part1(example_data));
    assert_eq!(24933642, part2(example_data));

    let input_file = Data::get("input.txt").unwrap();
    let input_data = std::str::from_utf8(input_file.data.as_ref()).unwrap();
    assert_eq!(1427048, part1(input_data));
    assert_eq!(2940614, part2(input_data));
}

fn part1(data: &str) -> usize {
    let fs = Tree::from_cmds(data);
    let x = fs.borrow().filtered_sum(THRESHOLD); x
}

fn part2(data: &str) -> usize {
    let fs = Tree::from_cmds(data);
    let x = fs.borrow().find_dir_to_delete(); x
}

fn parse_file_size(s: &str) -> usize {
    let s = s.split_whitespace().next().unwrap_or_default();
    s.parse().unwrap_or_default()
}

struct Tree {
    value: Option<usize>,
    children: Vec<Rc<RefCell<Tree>>>,
    parent: Option<Rc<RefCell<Tree>>>
}

impl Tree {
    fn new() -> Rc<RefCell<Tree>> {
        Rc::new(RefCell::new(Tree{
            value: None,
            children: vec![],
            parent: None
        }))
    }

    fn with_value(v: usize) -> Rc<RefCell<Tree>> {
        Rc::new(RefCell::new(Tree{
            value: Some(v),
            children: vec![],
            parent: None
        }))
    }

    fn from_cmds(data: &str) -> Rc<RefCell<Tree>> {
        let fs = Tree::new();
        let mut cur = Rc::clone(&fs);
        let cmds= data.split("$");

        cmds.for_each(|cmd| {
            if cmd.starts_with(" ls") {
                cmd.lines().skip(1)
                    .filter(|line| !line.starts_with("dir"))
                    .for_each(|line| {
                        let v = parse_file_size(line);
                        cur.borrow_mut().add_child(Tree::with_value(v))
                    })
            } else if cmd.contains(" cd ..") {
                let cur_clone = Rc::clone(&cur);
                cur = Rc::clone(cur_clone.borrow().parent.as_ref().unwrap());
            } else { // enter directory
                let child = Tree::new();
                cur.borrow_mut().add_child(Rc::clone(&child));
                child.borrow_mut().parent = Some(Rc::clone(&cur));
                cur = Rc::clone(&child);
            }
        });

        fs
    }

    fn add_child(&mut self, t: Rc<RefCell<Tree>>) {
        self.children.push(t);
    }

    fn filtered_sum(&self, limit: usize) -> usize {
        let mut dir_totals: Vec<usize> = Vec::new();
        self.dir_sizes(&mut dir_totals);
        dir_totals.clone().into_iter().filter(|x| *x <= limit).sum()
    }

    fn dir_sizes(&self, totals: &mut Vec<usize>) -> usize {
        match self.value {
            Some(v) => v,
            None => {
                let mut subtotal = 0;
                for child in &self.children {
                    subtotal += child.borrow().dir_sizes(totals);
                }
                totals.push(subtotal);
                subtotal
            }
        }
    }

    fn find_dir_to_delete(&self) -> usize {
        let used: usize = self.sum();
        let remaining = TOTAL_SPACE - used;
        let required = REQUIRED_SPACE - remaining;

        let mut dir_totals: Vec<usize> = Vec::new();
        self.dir_sizes(&mut dir_totals);

        let mut min: usize = usize::MAX;
        dir_totals.into_iter().for_each(|total| {
            if total >= required && total < min {
                min = total;
            }
        });

        min
    }

    fn sum(&self) -> usize {
        match self.value {
            Some(v) => v,
            None => {
                let mut total = 0;
                for child in &self.children {
                    total += child.borrow().sum()
                }
                total
            }
        }
    }
}