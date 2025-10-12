fn largest<T: std::cmp::PartialOrd>(nums: &[T]) -> &T {
    let mut max = &nums[0];

    for num in nums {
        if num > max {
            max = num;
        }
    }
    max
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    println!("Hello, world!");

    let nums = vec![34, 50, 25, 100, 65];

    let mut max = largest(&nums);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let dist = float.distance_from_origin();
}
