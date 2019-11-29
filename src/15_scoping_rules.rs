use std::fmt::{Debug};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

fn main() {
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
}