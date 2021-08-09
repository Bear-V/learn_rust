#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

struct OtherPoint<T> {
    x: T,
    y: T,
}

impl<T> OtherPoint<T> {
    fn print_x(&self) -> &T {
        &self.x
    }
}

impl OtherPoint<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[test]
fn test_point() {
    let both_int_float = Point { x: 32, y: 3.0 };
    println!("{:#?}", both_int_float);

    let p = OtherPoint { x: 40, y: 30 };
    println!("p.print_x = {}", p.print_x());
    println!("p.y = {}", p.y);
    let p = OtherPoint { x: 32.2, y: 33.3 };
    println!("p.distance = {}", p.distance_from_origin())
}
