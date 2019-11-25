#![allow(overflowing_literals)]

use std::convert::{From, TryFrom, TryInto};
use std::fmt::{self, Display, Formatter};
use std::mem;

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

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

    struct City {
        name: &'static str,
        // Latitude
        lat: f32,
        // Longitude
        lon: f32,
    }
    impl Display for City {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            write!(
                f,
                "{}: {:.3}°{}, {:.3}°{}",
                self.name,
                self.lat.abs(),
                lat_c,
                self.lon.abs(),
                lon_c
            )
        }
    }
    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }
    impl Display for Color {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(
                f,
                "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}",
                self.red, self.green, self.blue, self.red, self.green, self.blue
            )
        }
    }
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}\n", *city);
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", *color);
        println!("{}\n", *color);
    }
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:02b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:4b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    println!("FF: 二进制：{:b}，十进制：{}", 0xFF, 0xFF);
    println!("11: {}, 0b11: {}, 0o11: {}, 0x11: {}", 11, 0b11, 0o11, 0x11);

    // Use underscores to improve readability!
    println!("One million is written as {}\n", 1_000_000u32);

    // Tuples can be used as function arguments and as return values
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        // `let` can be used to bind the members of a tuple to variables
        let (integer, boolean) = pair;

        (boolean, integer)
    }

    // The following struct is for the activity.
    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);
    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({} {})\n({} {})\n", self.0, self.1, self.2, self.3)
        }
    }
    // A tuple with a bunch of different types
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);

    fn transpose(matrix: Matrix) -> Matrix {
        Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
    }
    let matrix = transpose(matrix);
    println!("{}\n", matrix);

    println!("以上为 println! 及自定义输出格式的规则！\n");

    println!("以下为 数组和切片 部分！");
    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the size of the array
    println!("array size: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

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
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = PersonNew { name, age };

    // Print debug struct
    println!("{:?}", peter);

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
    use Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    use Work::*;

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

    println!("以下为 绑定作用域 部分！");
    let long_lived_binging = 1;
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
        let long_lived_binging = 5_f32;
        println!("inner long: {}", long_lived_binging);
    }
    println!("outer long: {}", long_lived_binging);
    let long_lived_binging = 'a';
    println!("outer long: {}", long_lived_binging);

    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }
    println!("a_binding: {}", a_binding);

    println!("以下为 类型转换 部分！");
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as u16 is: {}", 1000 as u16);
    println!(
        "1000 as u8 is: {}, 1000 as 0b: {:b}, {}, {}",
        1000 as u8,
        1000,
        0b11101000,
        (1000 >> 8) * 256 + 232
    );
    println!("-1 as a u8 is: {}", -1_i8 as u8);

    println!("1000 mod 256 is : {}", 1000 % 256);
    println!("128 as a i16 is : {}", 128 as i16);
    println!("128 as a i8 is : {}", 128 as i8);

    println!("1000 as a u8 is : {}", 1000 as u8);
    println!("1000 as a i8 is : {}", 1000 as i8);
    println!("232 as a i8 is : {}", 232 as i8);

    println!("以下为 变量类型定义之后缀 部分！");
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literal, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    let elem = 5_u8;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);

    println!("以下为 类型别名 部分！");
    type NanoSecond = u64;
    type Inch = u64;

    #[allow(non_camel_case_types)]
    type u64_t = u64;

    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );

    println!("以下为 类型转换之 From and Into 部分！");

    #[derive(Debug)]
    struct NewNumber {
        value: i32,
    }
    impl From<i32> for NewNumber {
        fn from(item: i32) -> NewNumber {
            NewNumber { value: item }
        }
    }

    let num = NewNumber::from(30);
    println!("My number is {:?}", num);
    let int = 5;
    let num: NewNumber = int.into();
    println!("My number is {:?}", num);

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

    println!("以下为 控制流 部分！");
    println!("if-else");
    let n = 25;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // This expression returns an `i32`.
        10 * n
    } else {
        println!(", and is a big number, halve the number");

        // This expression must return an `i32` as well.
        n / 2
        // TODO ^ Try suppressing this expression with a semicolon.
    };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);

    let mut count = 0u32;

    println!("loop");
    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }

    println!("loop-marker");
    let mut count = 0;
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            count += 1;

            if count == 3 {
                break;
            }

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            if count == 5 {
                break 'outer;
            }
        }

        println!("Exited the inner loop");
    }

    println!("Exited the outer loop");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    println!("while");
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }

    println!("for-in-range");
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    println!("for-in-range-iter");
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("以下为 match 部分！");
    for number in 1..=21 {
        match number {
            // Match a single value
            1 => println!("One!"),
            // Match several values
            2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
            // Match an inclusive range
            13..=19 => println!("A teen"),
            // Handle the rest of cases
            _ => println!("Ain't special"),
        }
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);

    for x in 0..3 {
        for y in 0..=2 {
            match (x, y) {
                (0, y) => println!("First is `0` and `y` is `{:?}`", y),
                (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
                _ => println!("It doesn't matter what they are"),
            }
        }
    }

    println!("以下为 destructuring 部分！");
    #[allow(dead_code)]
    enum ColorTwo {
        // These 3 are specified solely by their name.
        Red,
        Blue,
        Green,
        // These likewise tie `u32` tuples to different names: color models.
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }
    let color = ColorTwo::RGB(122, 17, 40);
    // TODO ^ Try different variants for `color`

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        ColorTwo::Red => println!("The color is Red!"),
        ColorTwo::Blue => println!("The color is Blue!"),
        ColorTwo::Green => println!("The color is Green!"),
        ColorTwo::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        ColorTwo::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        ColorTwo::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        ColorTwo::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        ColorTwo::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
        // Don't need another arm because all variants have been examined
    }
    println!("以下为 ref 部分！");
    // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    let reference = &4;

    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
    println!("{}", mut_value);

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo_t: Vec<Foo> = vec![
        Foo { x: (1, 2), y: 3 },
        Foo { x: (3, 2), y: 2 },
        Foo { x: (3, 2), y: 3 },
    ];

    for i in foo_t.iter() {
        match i {
            Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

            // you can destructure structs and rename the variables,
            // the order is not important
            Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

            // and you can also ignore some variables:
            Foo { y, .. } => println!("y = {}, we don't care about x", y),
            // this will give an error: pattern does not mention field `x`
            //Foo { y } => println!("y = {}", y);
        }
    }

    let pair = vec![(2, -2), (2, 2), (3, 2), (4, 5)];
    for i in pair.iter() {
        match i {
            (x, y) if x == y => println!("These are twins"),
            (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
            (x, _) if x % 2 == 1 => println!("The first one is odd"),
            _ => println!("No correlation..."),
        }
    }

    fn _age() -> u32 {
        15
    }
    println!("Tell me what type of person you are");
    match _age() {
        0 => println!("I'm nor born yet I guess"),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => print!("I'm an old person of age {:?}", n),
    }

    for age in 0..=20 {
        match age {
            0 => println!("I'm nor born yet I guess"),
            n @ 1..=12 => println!("I'm a child of age {:?}", n),
            n @ 13..=19 => println!("I'm a teen of age {:?}", n),
            n => println!("I'm an old person of age {:?}", n),
        }
    }

    for age in 0..=20 {
        match age {
            0 => println!("I'm nor born yet I guess"),
            1..=12 => println!("I'm a child of age {:?}", age),
            13..=19 => println!("I'm a teen of age {:?}", age),
            _ => println!("I'm an old person of age {:?}", age),
        }
    }
    println!("以下为 if-let 部分！");
    // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    enum _Foo {
        Bar,
        Baz,
        Qux(u32),
    }
    let a = _Foo::Bar;
    let b = _Foo::Baz;
    let c = _Foo::Qux(100);

    if let _Foo::Bar = a {
        println!("a is foobar");
    }
    if let _Foo::Baz = b {
        println!("b is foobaz");
    }
    if let _Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    println!("以下为 while-let 部分！");
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }

    println!("以下为 fn 部分！");
    fizzbuzz_to(15);
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        // Corner case, early return
        if rhs == 0 {
            return false;
        }

        // This is an expression, the `return` keyword is not necessary here
        lhs % rhs == 0
    }

    // Functions that "don't" return a value, actually return the unit type `()`
    fn fizzbuzz(n: u32) -> () {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            println!("fizz");
        } else if is_divisible_by(n, 5) {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // When a function returns `()`, the return type can be omitted from the
    // signature
    fn fizzbuzz_to(n: u32) {
        for n in 1..n + 1 {
            fizzbuzz(n);
        }
    }
    impl Point {
        fn origin() -> Point {
            Point { x: 0., y: 0. }
        }

        fn new(x: f32, y: f32) -> Point {
            Point { x: x, y: y }
        }
    }
    impl Rectangle {
        fn area(&self) -> f32 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f32 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            2. * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        fn translate(&mut self, x: f32, y: f32) {
            self.p1.x += x;
            self.p2.x += x;

            self.p1.y += y;
            self.p2.y += y;
        }
    }

    struct PairNew(Box<i32>, Box<i32>);
    impl PairNew {
        fn destroy(self) {
            let PairNew(first, second) = self;
            println!("Destroying Pair({}, {})", first, second);
        }
    }

    let rectangle = Rectangle {
        // Static methods are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Instance methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    square.translate(1., 1.);

    let pair = PairNew(Box::new(1), Box::new(2));
    pair.destroy();

    println!("以下为 闭包 部分！");
    // Increment via closures and functions.
    fn function(i: i32) -> i32 {
        i + 1
    }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());

    let color = "green";

    // A closure to print `color` which immediately borrows (`&`)
    // `color` and stores the borrow and closure in the `print`
    // variable. It will remain borrowed until `print` goes out of
    // scope. `println!` only requires `by reference` so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();
    print();

    let mut count = 0;

    // A closure to increment `count` could take either `&mut count`
    // or `count` but `&mut count` is less restrictive so it takes
    // that. Immediately borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside.
    // Thus, calling the closure mutates the closure which requires
    // a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure.
    inc();
    inc();

    //let _reborrow = &mut count;
    // ^ TODO: try uncommenting this line.

    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    //consume();
    // ^ TODO: Try uncommenting this line.

    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&2));
    let haystack = vec![1, 2, 3];
    let contains = |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&2));
    println!("The len of haystack: {}", haystack.len());

    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }
    fn apply_to_3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    let diary = || {
        println!("I said {}", greeting);
        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);
        println!("Now I can sleep. zzzzz");

        mem::drop(farewell);
    };
    apply(diary);

    let double = |x| 2 * x;
    println!("3 double: {}", apply_to_3(double));

    println!("以下为 闭包作为输入参数 部分！");
    fn applynew<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }
    let x = 7;
    let print = || println!("{}", x);
    applynew(print);

    fn call_me<F: Fn()>(f: F) {
        f();
    }
    fn function_new() {
        println!("I'm a function!");
    }
    let closure = || println!("I'm a closure!");
    call_me(function_new);
    call_me(closure);
    println!("以下为 闭包作为返回值 部分！");
    /*
    fn creat_fn() -> Box<Fn()> {
        let text = "Fn".to_owned();

        Box::new(move || println!("This is a: {}", text))
    }
    fn creat_fnmut() -> Box<FnMut()> {
        let text = "FnMut".to_owned();

        Box::new(move || println!("This is a: {}", text))
    }
    let fn_plain = creat_fn();
    let mut fn_mut = creat_fnmut();

    fn_plain();
    fn_mut();
    */

    println!("Iterator::any");
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));

    let vec2 = vec![4, 5, 6];

    println!("Find 2 in vec1: {:?}", vec1.iter().find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", vec2.into_iter().find(|&x| x == 2));

    let array2 = [4, 5, 6];

    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&&x| x == 2)
    );

    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }
    let upper = 1000;
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    let sum_of_squared_odd_number: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .fold(0, |acc, n_squared| acc + n_squared);
    println!("functional style: {}", sum_of_squared_odd_number);

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i % 2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!(
        "Sum of odd numbers up to 9 (excluding): {}",
        sum_odd_numbers(9)
    );

    println!("以下为 模块 部分！");
    // A module named `my_mod`
    mod my_mod {
        // Items in modules default to private visibility.
        fn private_function() {
            println!("called `my_mod::private_function()`");
        }

        // Use the `pub` modifier to override default visibility.
        pub fn function() {
            println!("called `my_mod::function()`");
        }

        // Items can access other items in the same module,
        // even when private.
        pub fn indirect_access() {
            print!("called `my_mod::indirect_access()`, that\n> ");
            private_function();
        }

        // Modules can also be nested
        pub mod nested {
            pub fn function() {
                println!("called `my_mod::nested::function()`");
            }

            #[allow(dead_code)]
            fn private_function() {
                println!("called `my_mod::nested::private_function()`");
            }

            // Functions declared using `pub(in path)` syntax are only visible
            // within the given path. `path` must be a parent or ancestor module
            pub fn public_function_in_my_mod() {
                print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
                public_function_in_nested();
            }

            // Functions declared using `pub(self)` syntax are only visible within
            // the current module, which is the same as leaving them private
            pub(self) fn public_function_in_nested() {
                println!("called `my_mod::nested::public_function_in_nested()`");
            }

            // Functions declared using `pub(super)` syntax are only visible within
            // the parent module
            pub(super) fn public_function_in_super_mod() {
                println!("called `my_mod::nested::public_function_in_super_mod()`");
            }
        }

        pub fn call_public_function_in_my_mod() {
            print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
            nested::public_function_in_my_mod();
            print!("> ");
            nested::public_function_in_super_mod();
        }

        // pub(crate) makes functions visible only within the current crate
        pub(crate) fn public_function_in_crate() {
            println!("called `my_mod::public_function_in_crate()`");
        }

        // Nested modules follow the same rules for visibility
        mod private_nested {
            #[allow(dead_code)]
            pub fn function() {
                println!("called `my_mod::private_nested::function()`");
            }

            // Private parent items will still restrict the visibility of a child item,
            // even if it is declared as visible within a bigger scope.
            #[allow(dead_code)]
            pub(crate) fn restricted_function() {
                println!("called `my_mod::private_nested::restricted_function()`");
            }
        }
    }
    fn mod_function() {
        println!("called 'function()'");
    }
    // Modules allow disambiguation between items that have the same name.
    mod_function();
    my_mod::function();

    // Public items, including those inside nested modules, can be
    // accessed from outside the parent module.
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();

    // pub(in path) items can only be called from within the mode specified
    // Error! function `public_function_in_my_mod` is private
    //my_mod::nested::public_function_in_my_mod();
    // TODO ^ Try uncommenting this line

    // Private items of a module cannot be directly accessed, even if
    // nested in a public module:

    // Error! `private_function` is private
    //my_mod::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_function` is private
    //my_mod::nested::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::restricted_function();
    // TODO ^ Try uncommenting this line

    mod my {
        // A public struct with a public field of generic type `T`
        pub struct OpenBox<T> {
            pub contents: T,
        }

        // A public struct with a private field of generic type `T`
        #[allow(dead_code)]
        pub struct ClosedBox<T: Copy> {
            contents: T,
        }

        impl<T: Copy> ClosedBox<T> {
            // A public constructor method
            pub fn new(contents: T) -> ClosedBox<T> {
                ClosedBox { contents: contents }
            }
            pub fn get(&self) -> T {
                self.contents
            }
        }
    }
    // Public structs with public fields can be constructed as usual
    let open_box = my::OpenBox {
        contents: "public information",
    };

    // and their fields can be normally accessed.
    println!("The open box contains: {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `ClosedBox` has private fields
    //let closed_box = my::ClosedBox { contents: "classified information" };
    // TODO ^ Try uncommenting this line

    // However, structs with private fields can be created using
    // public constructors
    let _closed_box = my::ClosedBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    //println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ Try uncommenting this line
    println!("The closed box contains: {}", _closed_box.get());

    println!("以下为 use 部分！");
    use deeply::nested::function as other_function;

    fn use_function() {
        println!("called `use_function()`");
    }

    mod deeply {
        pub mod nested {
            pub fn function() {
                println!("called `deeply::nested::function()`");
            }
        }
    }
    // Easier access to `deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use deeply::nested::function;
        function();

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        println!("Leaving block");
    }

    use_function();
    #[allow(dead_code)]
    mod cool {
        pub fn function() {
            println!("called `cool::function()`");
        }
    }

    #[allow(dead_code)]
    mod my_super {
        fn function() {
            println!("called `my::function()`");
        }

        mod cool {
            pub fn function() {
                println!("called `my::cool::function()`");
            }
        }

        pub fn indirect_call() {
            // Let's access all the functions named `function` from this scope!
            print!("called `my::indirect_call()`, that\n> ");

            // The `self` keyword refers to the current module scope - in this case `my`.
            // Calling `self::function()` and calling `function()` directly both give
            // the same result, because they refer to the same function.
            self::function();
            function();

            // We can also use `self` to access another module inside `my`:
            self::cool::function();

            // The `super` keyword refers to the parent scope (outside the `my` module).
            //super::other_function;

            // This will bind to the `cool::function` in the *crate* scope.
            // In this case the crate scope is the outermost scope.
            {
                use cool::function as root_function;
                root_function();
            }
        }
    }

    println!("以下为 cfg 部分！");
    #[cfg(target_os = "linux")]
    fn are_you_on_linux() {
        println!("You are running linux!");
    }
    #[cfg(not(target_os = "linux"))]
    fn are_you_on_linux() {
        println!("You are *not* running linux!");
    }

    are_you_on_linux();
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }

    println!("以下为 泛型 部分！");
    struct Val {
        val: f64,
    }
    struct GenVal<T> {
        gen_val: T,
    }
    impl Val {
        fn value(&self) -> &f64 {
            &self.val
        }
    }
    impl<T> GenVal<T> {
        fn value(&self) -> &T {
            &self.gen_val
        }
    }

    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3_i32 };
    println!("{}, {}", x.value(), y.value());

    println!("以下为 traits 部分！");
    struct Empty;
    struct Null;
    trait DoubleDrop<T> {
        fn double_drop(self, _: T);
    }
    impl<T, U> DoubleDrop<T> for U {
        fn double_drop(self, _: T) {}
    }
    let empty = Empty;
    let null = Null;
    empty.double_drop(null);
    x.double_drop(y);

    println!("以下为 trait限定 部分！");
    trait HasArea {
        fn area(&self) -> f32;
    }
    impl HasArea for Circle {
        fn area(&self) -> f32 {
            self.radius * self.radius * 3.1415926
        }
    }
    use std::fmt::Debug;
    fn print_debug<T: Debug>(t: &T) {
        println!("{:?}", t)
    }
    fn area<T: HasArea>(t: &T) -> f32 {
        t.area()
    }
    let circle = Circle { radius: 3. };
    print_debug(&circle);
    println!("Area: {}", area(&circle));

    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: `{:?}`", t);
        println!("Display: `{}`", t);
    }
    
    fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t: `{:?}`", t);
        println!("u: `{:?}`", u);
    }
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    // compare_prints(&array);
    // TODO ^ Try uncommenting this.

    compare_types(&array, &vec);

    trait PrintInOption {
        fn print_in_option(self);
    }
    
    // Because we would otherwise have to express this as `T: Debug` or 
    // use another method of indirect approach, this requires a `where` clause:
    impl<T> PrintInOption for T where
        Option<T>: Debug {
        // We want `Option<T>: Debug` as our bound because that is what's
        // being printed. Doing otherwise would be using the wrong bound.
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }
    vec.print_in_option();

    println!("以下为 类型限定 部分！");
    struct Years(i64);

    struct Days(i64);

    impl Years {
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }


    impl Days {
        /// truncates partial years
        pub fn to_years(&self) -> Years {
            Years(self.0 / 365)
        }
    }

    fn old_enough(age: &Years) -> bool {
        age.0 >= 18
    }
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));

    println!("以下为 关联类型 部分！");
    struct Container(i32, i32);

    // A trait which checks if 2 items are stored inside of container.
    // Also retrieves first or last value.
    /*
    trait Contains<A, B> {
        fn contains(&self, _: &A, _: &B) -> bool; // Explicitly requires `A` and `B`.
        fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`.
        fn last(&self) -> i32;  // Doesn't explicitly require `A` or `B`.
    }

    impl Contains<i32, i32> for Container {
        // True if the numbers stored are equal.
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        // Grab the first number.
        fn first(&self) -> i32 { self.0 }

        // Grab the last number.
        fn last(&self) -> i32 { self.1 }
    }

    // `C` contains `A` and `B`. In light of that, having to express `A` and
    // `B` again is a nuisance.
    fn difference<A, B, C>(container: &C) -> i32 where
        C: Contains<A, B> {
        container.last() - container.first()
    }
    */
    // A trait which checks if 2 items are stored inside of container.
    // Also retrieves first or last value.
    trait Contains {
        // Define generic types here which methods will be able to utilize.
        type A;
        type B;

        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains for Container {
        // Specify what types `A` and `B` are. If the `input` type
        // is `Container(i32, i32)`, the `output` types are determined
        // as `i32` and `i32`.
        type A = i32;
        type B = i32;

        // `&Self::A` and `&Self::B` are also valid here.
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }
        // Grab the first number.
        fn first(&self) -> i32 { self.0 }

        // Grab the last number.
        fn last(&self) -> i32 { self.1 }
    }

    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));

    println!("以下为 PhantoData标签 部分！");
    use std::marker::PhantomData;
    #[derive(PartialEq)]
    struct PhantomTuple<A, B>(A, PhantomData<B>);

    #[derive(PartialEq)]
    struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> };

    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    use std::ops::Add;
    #[derive(Debug, Clone, Copy)]
    struct Cm {};
    #[derive(Debug, Clone, Copy)]
    struct Mm {};

    #[derive(Debug, Clone, Copy)]
    struct Length<Unit>(f64, PhantomData<Unit>);

    impl<Unit> Add for Length<Unit> {
        type Output = Length<Unit>;

        fn add(self, rhs: Length<Unit>) -> Length<Unit> {
            Length(self.0 + rhs.0 , PhantomData)
        }
    }
    let one_cm: Length<Cm> = Length(12., PhantomData);
    let one_mm: Length<Mm> = Length(1000., PhantomData);

    let two_cm = one_cm + one_cm;
    let two_mm = one_mm + one_mm;

    println!("one cm + one cm = {:?}", two_cm.0);
    println!("one mm + one mm = {:?}", two_mm.0);

    //let one_feter = one_cm + one_mm;

    println!("以下为 可变变量在被借用时不可变，直到借用完毕 Freezing 部分！");
    let mut _mutable_integer = 7_i32;
    println!("_mutable_integer {}", _mutable_integer);
    {
        let large_integer = &_mutable_integer;

        // _mutable_integer = 50;

        println!("Immutably borrowed {}", large_integer);
    }

    _mutable_integer = 3;
    println!("_mutable_integer {}", _mutable_integer);
    println!("以下为 ref 引用 部分！");
    let c = 'Q';
    let ref ref_c1 = c;
    let ref_c2 = &c;
    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0., y: 0. };
    let _copy_of_x = {
        let Point { x: ref ref_to_x, y: _ } = point;

        *ref_to_x
    };

    let mut mutable_point = point;
    {
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        *mut_ref_to_y = 1.;
    }
    println!{"point is ({}, {})", point.x, point.y};
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    let mut mutable_tuple = (Box::new(5_u32), 3_u32);
    {
        let (_, ref mut last) = mutable_tuple;
        *last = 2_u32;
    }
    println!("tuple is {:?}", mutable_tuple);

    println!("以下为 生命周期 部分！");
    let i = 3;
    {
        let borrow1 = &i;
        println!("borrow1: {}", borrow1);
    }
    {
        let borrow2 = &i;
        println!("borrow2: {}", borrow2);
    }

    fn print_one<'a>(x: &'a i32) {
        println!("'print_one': x is {}", x);
    }
    fn add_one<'a>(x: &'a mut i32) {
        *x += 1;
    }
    fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("'print_multi': x is {}, y is {}", x, y);
    }
    fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);

    println!("以下为 方法 部分！");
    struct Owner(i32);

    impl Owner {
        // Annotate lifetimes as in a standalone function.
        fn add_one<'a>(&'a mut self) { self.0 += 1; }
        fn print<'a>(&'a self) {
            println!("`print`: {}", self.0);
        }
    }
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();

    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    // Similarly, both references here must outlive this structure.
    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    // An enum which is either an `i32` or a reference to one.
    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);

    // Annotate lifetimes to impl.
    impl<'a> Default for Borrowed<'a> {
        fn default() -> Self {
            Self(&10)
        }
    }
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);

    #[derive(Debug)]
    struct Ref<'a, T: 'a>(&'a T);
    // `Ref` contains a reference to a generic type `T` that has
    // an unknown lifetime `'a`. `T` is bounded such that any
    // *references* in `T` must outlive `'a`. Additionally, the lifetime
    // of `Ref` may not exceed `'a`.

    // A generic function which prints using the `Debug` trait.
    fn print_lifetime<T>(t: T) where
        T: Debug {
        println!("`print`: t is {:?}", t);
    }

    // Here a reference to `T` is taken where `T` implements
    // `Debug` and all *references* in `T` outlive `'a`. In
    // addition, `'a` must outlive the function.
    fn print_ref<'a, T>(t: &'a T) where
        T: Debug + 'a {
        println!("`print_ref`: t is {:?}", t);
    }
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print_lifetime(ref_x);

    fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
        first * second
    }

    // `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
    // Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
    fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
        first
    }
    let first = 2; // Longer lifetime
    
    {
        let second = 3; // Shorter lifetime
        
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };

    static NUM: i32 = 18;
    fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
        &NUM
    }

    let static_string = "I'm in read-only memory";
    println!("static_string: {}", static_string);
    {
        let lifetime_num = 9;

        let  coerced_static = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerced_static);
    }
    println!("NUM: {} stays accessible!", NUM);
    fn elided_input(x: &i32) {
        println!("`elided_input`: {}", x);
    }

    fn annotated_input<'a>(x: &'a i32) {
        println!("`annotated_input`: {}", x);
    }

    // Similarly, `elided_pass` and `annotated_pass` have identical signatures
    // because the lifetime is added implicitly to `elided_pass`:
    fn elided_pass(x: &i32) -> &i32 { x }

    fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

        let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));

    println!("以下为 traits 部分！");
    struct Sheep {
        naked: bool,
        name: &'static str
    }
    trait Animal {
        fn new(name: &'static str) -> Self;
        
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }
    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }
        fn shear(&mut self) {
            if self.is_naked() {
                println!("{} is already naked...", self.name);
            } else {
                println!("{} gets a haircut!", self.name);

                self.naked = true;
            }
        }
    }
    impl Animal for Sheep {
        fn new(name: &'static str) -> Sheep {
            Sheep {
                name: name,
                naked: false
            }
        }
        fn name(&self) -> &'static str {
            self.name
        }
        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "baaaaah?"
            } else {
                "baaaaah!"
            }
        }
        fn talk(&self) {
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();

    println!("以下为 重载运算符 部分！");
    use std::ops;
    struct Fooo;
    struct Bar;
    #[derive(Debug)]
    struct FoooBar;
    #[derive(Debug)]
    struct BarFooo;
    impl ops::Add<Bar> for Fooo {
        type Output = FoooBar;

        fn add(self, _rhs: Bar) -> FoooBar {
            println!("> Fooo.add(Bar) was called");

            FoooBar
        }
    }
    impl ops::Add<Fooo> for Bar {
        type Output = BarFooo;

        fn add(self, _rhs: Fooo) -> BarFooo {
            println!("> Bar.add(Fooo) was calles");

            BarFooo
        }
    }

    println!("Fooo + Bar = {:?}", Fooo + Bar);
    println!("Bar + Fooo = {:?}", Bar + Fooo);

    println!("以下为 traints 之 iteratiors 部分！");
    struct Fibonacci {
        curr: u32,
        next: u32,
    }
    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            let new_next = self.curr + self.next;

            self.curr = self.next;
            self.next = new_next;

            Some(self.curr)
        }
    }
    fn fibonacci() -> Fibonacci {
        Fibonacci {
            curr: 0,
            next: 1,
        }
    }
    let mut sequence = 0..3;
    println!("Four consecutive 'next' calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterate through 0..3 using 'for'");
    for i in 0..3 {
        println!("> {}", i);
    }

    println!("The first four term od the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1_u32, 3, 3, 7];
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }

    println!("以下为 impl Trait 部分！");
    /*
    use std::iter;
    use std::vec::IntoIter;
    fn combine_vecs_explicit_return_type<'a>(
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
        v.into_iter().chain(u.into_iter()).cycle()
    }
    fn combine_vecs<'a>(
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> impl Iterator<Item=i32> {
        v.into_iter().chain(u.into_iter()).cycle()
    }
    let v: Vec<i32> = vec![0, 1, 2];
    let u: Vec<i32> = vec![3, 4, 5];
    for i in combine_vecs_explicit_return_type(v, u) {
        println!("{}", i);
    }
    let v: Vec<i32> = vec![0, 1, 2];
    let u: Vec<i32> = vec![3, 4, 5];
    for i in combine_vecs(v, u) {
        println!("{}", i);
    }
    */
    fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
        let closure = move |x: i32| { x + y };
        closure
    }
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);

    fn double_positive<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
        numbers
            .iter()
            .filter(|x| x > &&0)
            .map(|x| x * 2)
    }
    let v: Vec<i32> = vec![-1, -5, -23, 0, 3, 4, 2, -3];
    for i in double_positive(&v) {
        println!("{}", i);
    }

    println!("以下为 traits 之 clone 部分！");
    #[derive(Debug, Clone, Copy)]
    struct Nil1;

    // A tuple struct with resources that implements the `Clone` trait
    #[derive(Clone, Debug)]
    struct Pair1(Box<i32>, Box<i32>);
    // Instantiate `Nil`
    let nil = Nil1;
    // Copy `Nil`, there are no resources to move
    let copied_nil = nil;

    // Both `Nil`s can be used independently
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    // Instantiate `Pair`
    let pair = Pair1(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // Copy `pair` into `moved_pair`, moves resources
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);

    // Error! `pair` has lost its resources
    //println!("original: {:?}", pair);
    // TODO ^ Try uncommenting this line
    
    // Clone `moved_pair` into `cloned_pair` (resources are included)
    let cloned_pair = moved_pair.clone();
    // Drop the original pair using std::mem::drop
    drop(moved_pair);

    // Error! `moved_pair` has been dropped
    //println!("copy: {:?}", moved_pair);
    // TODO ^ Try uncommenting this line

    // The result from .clone() can still be used!
    println!("clone: {:?}", cloned_pair);

    println!("以下为 traits 之 supertraits 部分！");
    trait Human {
        fn name(&self) -> String;
    }
    trait Student: Human {
        fn university(&self) -> String;
    }
    trait Programmer {
        fn fav_language(&self) -> String;
    }
    trait CompSciStudent: Programmer + Student {
        fn git_usrname(&self) -> String;
    }
    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}. My favorite programming language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_usrname()
        )
    }

    struct Drogan {
        name: &'static str,
        university: &'static str,
        fav_language: &'static str,
        git_username: &'static str,
    }
    impl Human for Drogan {
        fn name(&self) -> String {
            self.name.to_string()
        }
    }
    impl Student for Drogan {
        fn university(&self) -> String {
            self.university.to_string()
        }
    }
    impl Programmer for Drogan {
        fn fav_language(&self) -> String {
            self.fav_language.to_string()
        }
    }
    impl CompSciStudent for Drogan {
        fn git_usrname(&self) -> String {
            self.git_username.to_string()
        }
    }
    let drogan = Drogan {
        name: "小明",
        university: "耶鲁大学",
        fav_language: "rust",
        git_username: "xiaoming@qq.com"
    };
    println!("{}", comp_sci_student_greeting(&drogan));

    println!("以下为 disambiguating overlapping traits 部分！");
    trait UsernameWidget {
        fn get(&self) -> String;
    }
    trait AgeWidget {
        fn get(&self) -> u8;
    }
    struct Form {
        username: String,
        age: u8,
    }
    impl UsernameWidget for Form {
        fn get(&self) -> String {
            self.username.clone()
        }
    }
    impl AgeWidget for Form {
        fn get(&self) -> u8 {
            self.age
        }
    }
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };
    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);

    println!("以下为 宏 macro_rules! 部分！");
    // This is a simple macro named `say_hello`.
    macro_rules! say_hello {
        // `()` indicates that the macro takes no argument.
        () => {
            // The macro will expand into the contents of this block.
            println!("Hello!");
        };
    }
    say_hello!();
    
    macro_rules! create_function {
        ($func_name: ident) => {
            fn $func_name() {
                println!("You called {:?}()",
                            stringify!($func_name));
            }
        };
    }
    create_function!(foo);
    create_function!(bar);

    macro_rules! print_result {
        ($expression: expr) => {
            println!("{:?} = {:?}",
                        stringify!($expression),
                        $expression);
        };
    }

    foo();
    bar();
    print_result!(1_u32 + 1);
    print_result!({
        let x = 1_u32;
        x * x + 2 * x - 1
    });

    macro_rules! test {
        ($left: expr; and $right: expr) => {
            println!("{:?} and {:?} is {:?}",
                        stringify!($left),
                        stringify!($right),
                        $left && $right)
        };
        ($left: expr; or $right: expr) => {
            println!("{:?} or {:?} is {:?}",
                        stringify!($left),
                        stringify!($right),
                        $left || $right)
        };
    }
    test!(1_i32 + 1 == 2_i32; and 2_i32 * 2 == 4_i32);
    test!(true; or false);

    macro_rules! find_min {
        ($x: expr) => ($x);
        ($x: expr, $($y: expr),+) => {
            std::cmp::min($x, find_min!($($y),+))
        }
    }
    println!("{}", find_min!(1_u32));
    println!("{}", find_min!(1_u32 + 2, 2_u32));
    println!("{}", find_min!(5_u32, 2_u32 * 3, 4_u32));

    macro_rules! calculate {
        (eval $e: expr) => {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        };
        (eval $e: expr, $(eval $es: expr),+) => {
            calculate! { eval $e }
            calculate! { $(eval $es),+ }
        }
    }
    calculate! {
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
    println!("以下为 错误处理 部分！");
    fn give_princess(gift: &str) {
        if gift == "snake" { panic!("AAAaaaaa!!!!"); }
        println!("I love {}s!!!!!", gift);
    }
    give_princess("teddy bear");
    //give_princess("snake");

    fn give_commoner(gift: Option<&str>) {
        match gift {
            Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
            Some(inner) => println!("{}? How nice.", inner),
            None => println!("No gife? Oh well."),
        }
    }
    fn give_princess_new(gift: Option<&str>) {
        let inside = gift.unwrap();
        if inside == "snake" { panic!("AAAaaaaa!!!!"); }
        
        println!("I love {}s!!!!", inside);
    }
    let food = Some("cabbage");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    //let nothing = None;

    give_princess_new(bird);
    //give_princess_new(nothing);

    fn next_birthday(current_age: Option<u8>) -> Option<String> {
        let next_age: u8 = current_age?;
        Some(format!("Next year I will be {}", next_age))
    }
    let age0 = Some(19);
    let age1 = None;
    println!("{:?}", next_birthday(age0));
    println!("{:?}", next_birthday(age1));

    struct Person {
        job: Option<Job>,
    }
    #[derive(Clone, Copy)]
    struct Job {
        phone_number: Option<PhoneNumber>,
    }
    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    struct PhoneNumber {
        area_code: Option<u8>,
        number: u32,
    }
    impl Person {
        fn work_phone_area_code(&self) -> Option<u8> {
            self.job?.phone_number?.area_code
        }
    }
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };
    assert_eq!(p.work_phone_area_code(), Some(61));

    #[derive(Debug)] enum Food { Apple, Carrot, Potato };
    #[derive(Debug)] struct Peeled(Food);
    #[derive(Debug)] struct Chopped(Food);
    #[derive(Debug)] struct Cooked(Food);

    fn peel(food: Option<Food>) -> Option<Peeled> {
        match food {
            Some(food) => Some(Peeled(food)),
            None => None,
        }
    }
    fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
        match peeled {
            Some(Peeled(food)) => Some(Chopped(food)),
            None => None,
        }
    }
    fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
        chopped.map(|Chopped(food)| Cooked(food))
    }
    fn process(food: Option<Food>) -> Option<Cooked> {
        food.map(|f| Peeled(f))
            .map(|Peeled(f)| Chopped(f))
            .map(|Chopped(f)| Cooked(f))
    }
    fn eat(food: Option<Cooked>) {
        match food {
            Some(food) => println!("Mmm, I love {:?}", food),
            None => println!("Oh no! It wasn't edible."),
        }
    }

    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = Some(Food::Potato);

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);

    #[derive(Debug)] enum FoodNew { CordonBleu, Steak, Sushi };
    #[derive(Debug)] enum Day { Monday, Tuesday, Wednesday };
    fn have_ingredients(food: FoodNew) -> Option<FoodNew> {
        match food {
            FoodNew::Sushi => None,
            _ => Some(food),
        }
    }
    fn have_recipe(food: FoodNew) -> Option<FoodNew> {
        match food {
            FoodNew::CordonBleu => None,
            _ => Some(food),
        }
    }
    #[allow(dead_code)]
    fn cookable_v1(food: FoodNew) -> Option<FoodNew> {
        match have_recipe(food) {
            None => None,
            Some(food) => match have_ingredients(food) {
                None => None,
                Some(food) => Some(food),
            }
        }
    }
    fn cookable_v2(food: FoodNew) -> Option<FoodNew> {
        have_recipe(food).and_then(have_ingredients)
    }
    fn eat_new(food: FoodNew, day: Day) {
        match cookable_v2(food) {
            Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
            None => println!("Oh no. We dont't get to eat on {:?}?", day),
        }
    }

    let (cordon_bleu, steak, sushi) = (FoodNew::CordonBleu, FoodNew::Steak, FoodNew::Sushi);
    eat_new(cordon_bleu, Day::Monday);
    eat_new(steak, Day::Tuesday);
    eat_new(sushi, Day::Wednesday);

}
