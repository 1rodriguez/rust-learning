use std::fmt;

fn main() {
    #[derive(Debug)]
    struct Vec2 {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Vec2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let v = Vec2 { x: 5.0, y: 6.0 };
    let Vec2 { x, .. } = v;
    println!("{}", v);
    println!("{}", x);
}
