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
    unimplemented!();
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

    #[ignore]
    #[rstest]
    fn problem2_test(input1: String) {
        unimplemented!();
    }
}
