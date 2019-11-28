use std::mem;

fn main() {
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
}