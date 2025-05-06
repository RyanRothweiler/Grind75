fn main() {
    println!("Hello, world!");
}

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
        return vec![];
    }

    let mut sorted = intervals;
    sorted.sort_by(|x, y| x[0].cmp(&y[0]));

    let mut ret: Vec<Vec<i32>> = vec![];
    ret.push(sorted[0].clone());

    for inserting in sorted.iter().skip(1) {
        if (inserting[0] >= ret.last().unwrap()[0] && inserting[0] <= ret.last().unwrap()[1])
            || (inserting[1] <= ret.last().unwrap()[1] && inserting[1] >= ret.last().unwrap()[0])
        {
            ret.last_mut().unwrap()[0] = i32::min(ret.last_mut().unwrap()[0], inserting[0]);
            ret.last_mut().unwrap()[1] = i32::max(ret.last_mut().unwrap()[1], inserting[1]);
        } else {
            ret.push(inserting.clone());
        }
    }

    ret
}
