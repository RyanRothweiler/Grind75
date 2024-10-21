use std::collections::HashSet;

pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    if sr < 0 || sr >= image.len() as i32 || sc < 0 || sc >= image[0].len() as i32 {
        return image;
    }

    let mut img = image.clone();
    let color_target = img[sr as usize][sc as usize];

    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    fill(&mut img, &mut seen, sr, sc, color, color_target);
    return img;
}

pub fn fill(
    image: &mut Vec<Vec<i32>>,
    seen: &mut HashSet<(i32, i32)>,
    origin_x: i32,
    origin_y: i32,
    color_new: i32,
    color_target: i32,
) {
    if origin_x < 0 || origin_y < 0 {
        return;
    }

    // Don't process cells we've already processed.
    // Otherwise we might flood fill forever.
    if seen.contains(&(origin_x, origin_y)) {
        return;
    }

    match image.get_mut(origin_x as usize) {
        Some(col) => {
            match col.get_mut(origin_y as usize) {
                Some(image_color) => {
                    if *image_color == color_target {
                        *image_color = color_new;

                        seen.insert((origin_x, origin_y));

                        fill(image, seen, origin_x + 1, origin_y, color_new, color_target);
                        fill(image, seen, origin_x - 1, origin_y, color_new, color_target);
                        fill(image, seen, origin_x, origin_y + 1, color_new, color_target);
                        fill(image, seen, origin_x, origin_y - 1, color_new, color_target);
                    }
                }
                None => {
                    return;
                }
            };
        }

        None => {
            return;
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let orig_image: Vec<Vec<i32>> = vec![vec![0, 0, 0], vec![0, 0, 0]];
        let mod_image = flood_fill(orig_image.clone(), 0, 0, 0);
        assert_eq!(mod_image, orig_image);
    }

    #[test]
    fn full() {
        let orig_image: Vec<Vec<i32>> = vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]];
        let mod_image = flood_fill(orig_image, 1, 1, 3);

        let target_image: Vec<Vec<i32>> = vec![vec![3, 3, 3], vec![3, 3, 3], vec![3, 3, 3]];

        assert_eq!(mod_image, target_image);
    }

    #[test]
    fn square_single() {
        let orig_image: Vec<Vec<i32>> = vec![vec![2, 2, 1], vec![2, 2, 1], vec![2, 2, 1]];
        let mod_image = flood_fill(orig_image, 1, 1, 3);

        let target_image: Vec<Vec<i32>> = vec![vec![3, 3, 1], vec![3, 3, 1], vec![3, 3, 1]];

        assert_eq!(mod_image, target_image);
    }

    #[test]
    #[rustfmt::skip]
    fn square_none() {
        let orig_image: Vec<Vec<i32>> = 
            vec![
                vec![2, 0, 1], 
                vec![2, 0, 1], 
                vec![2, 0, 1]
            ];
        let mod_image = flood_fill(orig_image, 1, 1, 3);

        let target_image: Vec<Vec<i32>> = 
            vec![
                vec![2, 3, 1], 
                vec![2, 3, 1], 
                vec![2, 3, 1]
            ];

        assert_eq!(mod_image, target_image);
    }

    #[test]
    #[rustfmt::skip]
    fn rect_full() {
        let orig_image: Vec<Vec<i32>> = 
            vec![
                vec![2, 2, 2], 
                vec![2, 2, 2], 
                vec![2, 2, 2], 
                vec![2, 2, 2]
            ];
        let mod_image = flood_fill(orig_image, 1, 1, 3);

        let target_image: Vec<Vec<i32>> = 
            vec![
                vec![3, 3, 3], 
                vec![3, 3, 3], 
                vec![3, 3, 3], 
                vec![3, 3, 3]
            ];

        assert_eq!(mod_image, target_image);
    }

    #[test]
    #[rustfmt::skip]
    fn rect_wide() {
        let orig_image: Vec<Vec<i32>> = 
            vec![
                vec![2, 2, 2, 1], 
                vec![2, 2, 2, 1], 
                vec![2, 2, 2, 1]
            ];
        let mod_image = flood_fill(orig_image, 1, 1, 3);

        let target_image: Vec<Vec<i32>> = 
            vec![
                vec![3, 3, 3, 1], 
                vec![3, 3, 3, 1], 
                vec![3, 3, 3, 1]
            ];

        assert_eq!(mod_image, target_image);
    }

    #[test]
    #[rustfmt::skip]
    fn negative() {
        let orig_image: Vec<Vec<i32>> = 
            vec![
                vec![2, 2, 2, 1], 
                vec![2, 2, 2, 1], 
                vec![2, 2, 2, 1]
            ];
        let mod_image = flood_fill(orig_image.clone(), -1, -1, 3);

        assert_eq!(mod_image, orig_image);
    }
}
