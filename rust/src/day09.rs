use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Reverse;
use std::fs;
use std::iter;

pub fn problem1() {
    println!("{}", problem1_str(read_data()));
}

pub fn problem2() {
    println!("{}", problem2_str(read_data()));
}

fn read_data() -> String {
    fs::read_to_string("data/day09.txt")
        .expect("Error reading file")
}

fn problem1_str(data: String) -> usize {
    let mut data = data.trim();
    if data.len() % 2 == 0 {
        data = &data[0..data.len() - 2];
    }
    let size = block_size(data.chars());
    let n_files = (data.len() + 1) / 2;

    let mut fwd_iter = block_iter(data.chars());
    let mut rev_iter = block_iter(data.chars().rev());
    let mut fwd_pos: usize = 0;
    let mut rev_pos: usize = 0;
    let mut moving_fwd = true;
    let mut checksum: usize = 0;

    loop {
        if fwd_pos + rev_pos >= size {
            break;
        }
        if moving_fwd {
            match fwd_iter.next().unwrap() {
                Some(file_number) => {
                    checksum += file_number * fwd_pos;
                    fwd_pos += 1;
                },
                None => {
                    moving_fwd = false;
                }
            }
        } else {
            if let Some(file_number) = rev_iter.next().unwrap() {
                checksum += (n_files - file_number - 1) * fwd_pos;
                fwd_pos += 1;
                moving_fwd = true;
            }
            rev_pos += 1;
        }
    }
    checksum
}

fn problem2_str(data: String) -> usize {
    let mut data = data.trim();
    if data.len() % 2 == 0 {
        data = &data[0..data.len() - 2];
    }

    let mut files: Vec<File> = Vec::new();
    let mut gaps_by_size: HashMap<usize, BinaryHeap<Reverse<Gap>>> = HashMap::new();
    let mut pos: usize = 0;

    for (i, ch) in data.chars().enumerate() {
        let size = ch.to_digit(10).unwrap() as usize;
        if size > 0 {
            if i % 2 == 0 {
                files.push(File { id: i / 2, pos, size });
            } else {
                gaps_by_size.entry(size).or_default().push(Reverse(Gap { pos, size }));
            }
        } else if i % 2 == 0 {
            panic!("zero sized file"); // doesn't happen in practice, but enforcing
        }
        pos += size;
    }
    files.reverse();

    let mut checksum: usize = 0;
    for file in files {
        let valid_gap = gaps_by_size.keys()
            .filter(|&&gap_size| gap_size >= file.size)
            .flat_map(|gap_size| gaps_by_size.get(&gap_size).unwrap().peek())
            .filter(|&Reverse(gap)| gap.pos < file.pos)
            .min_by_key(|&Reverse(gap)| gap.pos);
        if let Some(&Reverse(ref gap)) = valid_gap {
            let new_file = File { id: file.id, pos: gap.pos, size: file.size };
            let new_gap = Gap { pos: gap.pos + file.size, size: gap.size - file.size };
            gaps_by_size.entry(gap.size).and_modify(|e| { e.pop(); });
            if new_gap.size > 0 {
                gaps_by_size.entry(new_gap.size).and_modify(|e| e.push(Reverse(new_gap)));
            }
            checksum += new_file.checksum();
        } else {
            checksum += file.checksum();
        }
    }
    checksum
}

#[derive(Debug)]
struct File {
    id: usize,
    pos: usize,
    size: usize,
}

impl File {
    fn checksum(&self) -> usize {
        if self.size == 0 {
            0
        } else {
            self.id * (self.pos * self.size + self.size * (self.size - 1) / 2)
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Gap {
    pos: usize,
    size: usize,
}

fn block_size<'a>(data: impl Iterator<Item = char> + 'a) -> usize {
    data.map(|ch| ch.to_digit(10).unwrap() as usize).sum()
}

fn block_iter<'a>(data: impl Iterator<Item = char> + 'a) -> impl Iterator<Item = Option<usize>> + 'a {
    data.enumerate()
        .flat_map(|(i, ch)| {
            let size = ch.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                iter::repeat(Some(i / 2)).take(size)
            } else {
                iter::repeat(None).take(size)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input1() -> String {
        String::from("2333133121414131402")
    }

    #[rstest]
    fn problem1_test(input1: String) {
        assert_eq!(problem1_str(input1), 1928);
    }

    #[rstest]
    fn problem2_test(input1: String) {
        assert_eq!(problem2_str(input1), 2858);
    }
}
