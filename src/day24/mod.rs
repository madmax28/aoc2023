use std::collections::HashSet;

type Vec3<T> = (T, T, T);

fn parse<T>(input: &str) -> Vec<(Vec3<T>, Vec3<T>)>
where
    T: std::str::FromStr + Copy,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" @ ").unwrap();
            let p: Vec<T> = p.split(", ").map(|n| n.parse().unwrap()).collect();
            let v: Vec<T> = v.split(", ").map(|n| n.parse().unwrap()).collect();
            ((p[0], p[1], p[2]), (v[0], v[1], v[2]))
        })
        .collect()
}

pub fn part1(input: &str) -> crate::Result<usize> {
    const MIN: f64 = 200000000000000.0;
    const MAX: f64 = 400000000000000.0;
    let mut cnt = 0;
    let hails = parse::<f64>(input);
    for (idx, ((x1, y1, _), (vx1, vy1, _))) in hails.iter().enumerate() {
        for ((x2, y2, _), (vx2, vy2, _)) in &hails[1 + idx..] {
            let y = (vx1 / vy1 * y1 - vx2 / vy2 * y2 + x2 - x1) / (vx1 / vy1 - vx2 / vy2);
            let x = vx1 / vy1 * (y - y1) + x1;
            if !(MIN..=MAX).contains(&x) || !(MIN..=MAX).contains(&y) {
                continue;
            }

            // in the past
            let t1 = (x - x1) / vx1;
            let t2 = (x - x2) / vx2;
            if t1 <= 0.0 || t2 <= 0.0 {
                continue;
            }

            cnt += 1;
        }
    }
    Ok(cnt)
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let hails = parse::<i64>(input);

    let mut xs = HashSet::new();
    let (mut stone_x, mut stone_vx) = (0, 0);
    for ((x, _, _), (vx, _, _)) in &hails {
        if !xs.insert((x, vx)) {
            stone_x = *x;
            stone_vx = *vx;
            break;
        }
    }

    let ((x1, y1, z1), (vx1, vy1, vz1)) = hails[0];
    let t1 = (x1 - stone_x) / (stone_vx - vx1);
    let ((x2, y2, z2), (vx2, vy2, vz2)) = hails[1];
    let t2 = (x2 - stone_x) / (stone_vx - vx2);
    let stone_vy = (y2 - y1 + t2 * vy2 - t1 * vy1) / (t2 - t1);
    let stone_y = y1 + t1 * (vy1 - stone_vy);
    let stone_vz = (z2 - z1 + t2 * vz2 - t1 * vz1) / (t2 - t1);
    let stone_z = z1 + t1 * (vz1 - stone_vz);

    Ok(stone_x + stone_y + stone_z)
}
