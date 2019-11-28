use std::convert::{TryFrom, TryInto};
use std::fmt;

fn main() {
    println!("以下为 类型转换之 TryForm and TryInto 部分！");
    #[derive(Debug, PartialEq)]
    struct EventNumber(i32);

    impl TryFrom<i32> for EventNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EventNumber(value))
            } else {
                Err(())
            }
        }
    }

    assert_eq!(EventNumber::try_from(8), Ok(EventNumber(8)));
    assert_eq!(EventNumber::try_from(5), Err(()));

    let result: Result<EventNumber, ()> = 8_i32.try_into();
    assert_eq!(result, Ok(EventNumber(8)));
    let result: Result<EventNumber, ()> = 5_i32.try_into();
    assert_eq!(result, Err(()));

    #[derive(Debug)]
    struct Circle {
        radius: f32,
    }
    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }

    let circle = Circle { radius: 6. };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    println!("以下为 表达式 部分！");
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}