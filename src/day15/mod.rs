use std::array;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

fn hash(s: &str) -> usize {
    s.chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17).rem_euclid(256))
}

pub fn part1(input: &str) -> crate::Result<usize> {
    Ok(input.split(',').map(hash).sum())
}

pub fn part2(input: &str) -> crate::Result<usize> {
    let mut boxes: [Vec<(String, u32)>; 256] = array::from_fn(|_| Vec::new());
    for instr in input.split(',') {
        let mut label = String::new();
        let mut op = None;
        let mut focal_length = None;
        for c in instr.chars() {
            if c.is_ascii_alphabetic() {
                label.push(c);
            } else if c.is_ascii_punctuation() {
                op = Some(c);
            } else {
                focal_length = c.to_digit(10);
            }
        }
        let op = op.ok_or(crate::Error::boxed(Error::InvalidInput))?;

        let lens_box = &mut boxes[hash(&label)];
        match op {
            '=' => {
                let focal_length = focal_length.ok_or(crate::Error::boxed(Error::InvalidInput))?;
                if let Some(lens) = lens_box.iter_mut().find(|(l, _)| l == &label) {
                    lens.1 = focal_length;
                } else {
                    lens_box.push((label, focal_length));
                }
            }
            '-' => {
                lens_box.retain(|(l, _)| l != &label);
            }
            _ => return Err(crate::Error::boxed(Error::InvalidInput)),
        }
    }

    let mut sum = 0;
    for (box_num, lens_box) in boxes.into_iter().enumerate() {
        for (slot_num, (_, focal_length)) in lens_box.into_iter().enumerate() {
            sum += (1 + box_num) * (1 + slot_num) * focal_length as usize;
        }
    }
    Ok(sum)
}
