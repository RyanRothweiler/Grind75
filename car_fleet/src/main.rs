fn main() {
    car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]);
}

pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    #[derive(Debug, Clone, Copy)]
    struct Car {
        speed: i32,
        position: i32,
    }

    impl Car {
        pub fn time_to_target(&self, target: i32) -> f64 {
            (target as f64 - self.position as f64) / self.speed as f64
        }
    }

    let mut cars: Vec<Car> = vec![];

    for (i, p) in position.iter().enumerate() {
        cars.push(Car {
            speed: speed[i],
            position: *p,
        });
    }

    cars.sort_by(|a, b| b.position.cmp(&a.position));

    let mut coallated: Vec<Car> = vec![];

    for c in cars {
        if let Some(top_car) = coallated.last() {
            // if top_car and c do not meet then push c on to stack
            let top_time = top_car.time_to_target(target);
            let c_time = c.time_to_target(target);

            println!("top {}, c {}", top_time, c_time);

            if top_time < c_time {
                coallated.push(c);
            }
        } else {
            coallated.push(c);
        }
    }

    println!("{:#?}", coallated);

    coallated.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn working() {
        assert_eq!(car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]), 3);
    }
}
