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
        if let Some((label, focal_length)) = instr.split_once('=') {
            let lens_box = &mut boxes[hash(label)];
            if let Some(lens) = lens_box.iter_mut().find(|(l, _)| l == label) {
                lens.1 = focal_length.parse()?;
            } else {
                lens_box.push((label.to_string(), focal_length.parse()?));
            }
        } else if let Some((label, _)) = instr.split_once('-') {
            let lens_box = &mut boxes[hash(label)];
            lens_box.retain(|(l, _)| l != label);
        } else {
            return Err(crate::Error::boxed(Error::InvalidInput));
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
