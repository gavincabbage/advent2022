use std::cell::RefCell;
use std::rc::Rc;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Data;

const THRESHOLD: usize = 100_000;

fn main() {
    let example_file = Data::get("example.txt").unwrap();
    let example_data = std::str::from_utf8(example_file.data.as_ref()).unwrap();
    assert_eq!(95437, part1(example_data));

    let input_file = Data::get("input.txt").unwrap();
    let input_data = std::str::from_utf8(input_file.data.as_ref()).unwrap();
    assert_eq!(0, part1(input_data));
}

fn part1(data: &str) -> usize {
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
        } else if cmd == " cd .." {
            let cur_clone = Rc::clone(&cur);
            cur = Rc::clone(cur_clone.borrow().parent.as_ref().unwrap());
        } else { // enter directory
            let child = Tree::new();
            cur.borrow_mut().add_child(Rc::clone(&child));
            child.borrow_mut().parent = Some(Rc::clone(&cur));
            cur = Rc::clone(&child);
        }
    });

    // fs.borrow().traverse();

    let x = fs.borrow().filtered_sum(THRESHOLD); x
}

fn parse_file_size(s: &str) -> usize {
    let s = s.split_whitespace().next().unwrap_or_default();
    s.parse().unwrap_or_default()
}

#[derive(Debug)]
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

    fn add_child(&mut self, t: Rc<RefCell<Tree>>) {
        self.children.push(t);
    }

    // almost works
    // fn filtered_sum(&self, limit: usize) -> usize {
    //     let mut total = 0;
    //     match self.value {
    //         Some(v) => if v <= limit { total = v },
    //         None => {
    //             let mut subtotal = 0;
    //             for t in &self.children {
    //                 subtotal += t.borrow().filtered_sum(limit)
    //             }
    //             if subtotal <= limit { total += subtotal }
    //         }
    //     }
    //     total
    // }

    fn filtered_sum(&self, limit: usize) -> usize {
        let mut dir_totals: Vec<usize> = Vec::new();
        self.filtered_sum_inner(&mut dir_totals, 0);
        println!("{:?}", dir_totals);
        dir_totals.clone().into_iter().filter(|x| *x <= limit).sum()
    }

    fn filtered_sum_inner(&self, totals: &mut Vec<usize>, depth: usize) -> usize {
        println!("depth {depth}");
        match self.value {
            Some(v) => v,
            None => {
                let mut subtotal = 0;
                for child in &self.children {
                    subtotal += child.borrow().filtered_sum_inner(totals, depth+1);
                }
                totals.push(subtotal);
                subtotal
            }
        }
    }
}