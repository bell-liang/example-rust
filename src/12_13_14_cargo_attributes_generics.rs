use std::fmt::{self, Display};

#[derive(Debug)]
struct Circle {
    radius: f32,
}
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
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
}