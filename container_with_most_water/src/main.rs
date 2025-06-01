fn main() {
    println!("Hello, world!");
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = height.len() - 1;

    let mut max_vol: i32 = 0;

    while left < right {
        let pot_vol = i32::min(height[left], height[right]) * (right - left) as i32;
        max_vol = i32::max(max_vol, pot_vol);

        if height[left] <= height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_vol
}
