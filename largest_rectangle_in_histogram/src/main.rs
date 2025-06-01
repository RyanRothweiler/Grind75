fn main() {
    println!("Hello, world!");
}

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    struct Rect {
        start: usize,
        height: i32,
    }

    let mut possible_rect: Vec<Rect> = vec![];
    let mut possible_max: i32 = 0;

    for (i, new_height) in heights.iter().enumerate() {
        let mut new_rect = Rect {
            start: i,
            height: *new_height,
        };

        while let Some(t) = possible_rect.last() {
            if t.height > *new_height {
                new_rect.start = t.start;

                let removed = possible_rect.pop().unwrap();
                let possible_size = (i - removed.start) as i32 * removed.height;
                possible_max = i32::max(possible_max, possible_size);
            } else {
                break;
            }
        }

        possible_rect.push(new_rect);
    }

    while let Some(r) = possible_rect.pop() {
        let possible_size = (heights.len() - r.start) as i32 * r.height;
        possible_max = i32::max(possible_max, possible_size);
    }

    possible_max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn working() {
        assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(largest_rectangle_area(vec![2, 4]), 4);
    }
}
