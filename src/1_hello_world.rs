#![allow(overflowing_literals)]

use std::convert::{From, TryFrom, TryInto};
use std::fmt::{self, Display, Formatter};

fn main() {
    println!("以下为 println! 简单使用！\n");
    println!("Hello, world!");

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    println!("{number:>width$}", number = 1, width = 6);

    println!("{number:>0width$}", number = 1, width = 6);

    println!("My name is {0}, {1} {0}\n", "Bond", "James");

    println!("以下为 Debug 与 Display 的对比使用，其中 Display 用 fmt 格式化！\n");
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);
    println!("This struct `{:?}` won't print...", Structure(3));
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );
    println!("{}\n", Structure(2));

    #[derive(Debug)]
    struct PersonNew<'a> {
        name: &'a str,
        age: u8,
    }
    let name = "Peter";
    let age = 27;
    let peter = PersonNew { name, age };
    // Pretty print
    println!("{:#?}", peter);
    println!("{:?}\n", peter);

    #[derive(Debug)]
    struct MinMax(i64, i64);
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }
    let minmax = MinMax(3, 4);
    println!("{}", minmax);
    println!("{:?}\n", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}\n",
        small = small_range,
        big = big_range
    );

    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }
    let point = Point2D { x: 3., y: 4. };
    println!("{}", point);
    println!("{:?}\n", point);

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }
    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }
    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("{}", complex);
    println!("{:?\n}", complex);

    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ",")?;
                }
                write!(f, "{}", v)?;
            }

            write!(f, "]")
        }
    }
    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    struct ListIndex(Vec<i32>);
    impl fmt::Display for ListIndex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", count, v)?;
            }

            write!(f, "]")
        }
    }
    let v = ListIndex(vec![1, 2, 3]);
    println!("{}\n", v);
}