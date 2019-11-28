#![allow(overflowing_literals)]
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

#[allow(dead_code)]
enum Status {
    Rich,
    Poor,
}
#[allow(dead_code)]
enum Work {
    Civilian,
    Soldier,
}

fn main() {
    println!("以下为 结构体 部分！");
    // A unit struct
    struct Nil;

    // A tuple struct
    struct Pair(i32, f32);

    // A struct with two fields
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: f32,
        y: f32,
    }

    // Structs can be reused as fields of another struct
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Rectangle {
        p1: Point,
        p2: Point,
    }
    /*
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = PersonNew { name, age };

    // Print debug struct
    println!("{:?}", peter);
    */

    // Instantiate a `Point`
    let point: Point = Point { x: 3., y: 4. };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point { x: 1., y: 5. };
    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: new_point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    fn rect_area(rectangle: Rectangle) -> f32 {
        let Rectangle {
            p1: Point { x: x1, y: y1 },
            p2: Point { x: x2, y: y2 },
        } = rectangle;
        println!("{:?}", rectangle);
        println!("x1: {}, x2: {}, y1: {}, y2: {}", x1, x2, y1, y2);
        (x1 - x2).abs() * (y1 - y2).abs()
    }
    println!("{}", rect_area(_rectangle));

    fn square(point: Point, length: f32) -> Rectangle {
        let upper_left_point = Point {
            x: point.x,
            y: point.y + length,
        };
        let lower_right_point = Point {
            x: point.x + length,
            y: point.y,
        };
        Rectangle {
            p1: upper_left_point,
            p2: lower_right_point,
        }
    }
    let new_rectangle = square(point, 2.);
    println!("{}\n", rect_area(new_rectangle));

    println!("以下为 枚举 部分！");
    enum WebEvent {
        // An `enum` may either be `unit-like`,
        PageLoad,
        PageUnload,
        // like tuple structs,
        KeyPress(char),
        Paste(String),
        // or c-like structures.
        Click { x: i64, y: i64 },
    }

    // A function which takes a `WebEvent` enum as an argument and
    // returns nothing.
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            // Destructure `c` from inside the `enum`.
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            // Destructure `Click` into `x` and `y`.
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            }
        }
    }
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
    impl Operations {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }
    println!("{}", Operations::Add.run(1, 2));

    println!("以下为 enum-use 部分！");
    use crate::Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    use crate::Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
    println!("以下为 enum-c-like 部分！");
    #[allow(dead_code)]
    enum Number {
        Zero,
        One,
        Two,
    }
    #[allow(dead_code)]
    enum ColorNew {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", ColorNew::Red as i32);
    println!("violets are #{:06x}", ColorNew::Blue as i32);

    println!("以下为 静态全局变量 部分！");
    let n = 16;
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}