use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() {
    let example_file = Data::get("example.txt").unwrap();
    let example_data = std::str::from_utf8(example_file.data.as_ref()).unwrap();
    assert_eq!(21, part1(example_data));
    assert_eq!(8, part2(example_data));

    let input_file = Data::get("input.txt").unwrap();
    let input_data = std::str::from_utf8(input_file.data.as_ref()).unwrap();
    assert_eq!(1816, part1(input_data));
    assert_eq!(383520, part2(input_data));
}

fn part1(data: &str) -> usize {
    let grid = parse(data);
    let mut count: usize = 0;

    for ndx in 0..grid.data.len() {
        if grid.visible(ndx) {
            count += 1
        }
    }

    count
}

fn part2(data: &str) -> i32 {
    let grid = parse(data);
    let mut max: i32 = 0;

    for ndx in 0..grid.data.len() {
        let score = grid.score(ndx);
        if score > max {
            max = score
        }
    }

    max
}

fn parse(data: &str) -> Grid {
    let size = data.lines().count();
    let data = data.lines().flat_map(|line| {
        line.chars().map(|char| char.to_digit(10).unwrap_or_default() as usize)
    }).collect();
    Grid{size, data}
}

#[derive(Debug)]
struct Grid {
    size: usize,
    data: Vec<usize>
}

impl Grid {

    fn visible(&self, ndx: usize) -> bool {
        let ndx: i32 = ndx as i32;
        self.visible_north(ndx) || self.visible_south(ndx) ||
            self.visible_east(ndx) || self.visible_west(ndx)
    }

    fn visible_north(&self, ndx: i32) -> bool {
        let mut cur = ndx - self.size as i32;
        let mut visible = true;
        while cur >= 0 {
            if self.data[cur as usize] >= self.data[ndx as usize] {
                visible = false;
                break
            }
            cur -= self.size as i32
        }
        visible
    }

    fn visible_south(&self, ndx: i32) -> bool {
        let mut cur = ndx + self.size as i32;
        let mut visible = true;
        while cur < self.data.len() as i32 {
            if self.data[cur as usize] >= self.data[ndx as usize] {
                visible = false;
                break
            }
            cur += self.size as i32
        }
        visible
    }

    fn visible_east(&self, ndx: i32) -> bool {
        let mut cur = ndx + 1;
        let mut visible = true;
        let max = (ndx / self.size as i32 + 1) * self.size as i32;
        while cur < max as i32 {
            if self.data[cur as usize] >= self.data[ndx as usize] {
                visible = false;
                break
            }
            cur += 1
        }
        visible
    }

    fn visible_west(&self, ndx: i32) -> bool {
        let mut cur = ndx - 1;
        let mut visible = true;
        let min = ndx - (ndx % self.size as i32);
        while cur >= min {
            if self.data[cur as usize] >= self.data[ndx as usize] {
                visible = false;
                break
            }
            cur -= 1
        }
        visible
    }

    ////

    fn score(&self, ndx: usize) -> i32 {
        let ndx: i32 = ndx as i32;
        self.score_north(ndx) * self.score_south(ndx) *
            self.score_east(ndx) * self.score_west(ndx)
    }

    fn score_north(&self, ndx: i32) -> i32 {
        let mut distance = 0;
        let mut cur = ndx - self.size as i32;
        while cur >= 0 {
            distance += 1;
            if self.data[cur as usize] >= self.data[ndx as usize] {
                break
            }
            cur -= self.size as i32;

        }
        distance
    }

    fn score_south(&self, ndx: i32) -> i32 {
        let mut distance = 0;
        let mut cur = ndx + self.size as i32;
        while cur < self.data.len() as i32 {
            distance += 1;
            if self.data[cur as usize] >= self.data[ndx as usize] {
                break
            }
            cur += self.size as i32;

        }
        distance
    }

    fn score_east(&self, ndx: i32) -> i32 {
        let mut distance = 0;
        let mut cur = ndx + 1;
        let max = (ndx / self.size as i32 + 1) * self.size as i32;
        while cur < max as i32 {
            distance += 1;
            if self.data[cur as usize] >= self.data[ndx as usize] {
                break
            }
            cur += 1;

        }
        distance
    }

    fn score_west(&self, ndx: i32) -> i32 {
        let mut distance = 0;
        let mut cur = ndx - 1;
        let min = ndx - (ndx % self.size as i32);
        while cur >= min {
            distance += 1;
            if self.data[cur as usize] >= self.data[ndx as usize] {
                break
            }
            cur -= 1;

        }
        distance
    }
}