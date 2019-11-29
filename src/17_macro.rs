fn main() {
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
}