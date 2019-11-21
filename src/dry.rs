use std::fmt::Debug;

pub mod play {
    use std::ops::{Add, Mul, Sub};

    macro_rules! assert_equal_len {
        ($a: ident, $b: ident, $func: ident, $op: tt) => {
            assert!($a.len() == $b.len(),
                    "{:?}: dimension mismatch: {:?} {:?} {:?}",
                    stringify!($func),
                    ($a.len(),),
                    stringify!($op),
                    ($b.len(),));
        };
    }
    macro_rules! op {
        ($func: ident, $bound: ident, $op: tt, $method: ident) => {
            pub fn $func<T: $bound<T, Output=T> + Copy + Debug>(xs: &mut Vec<T>, ys: &Vec<T>) {
                assert_equal_len!(xs, ys, $func, $op);

                for (x, y) in xs.iter_mut().zip(ys.iter()) {
                    *x = $bound::$method(*x, *y);
                }
            }
        };
    }
    op!(add_assign, Add, +=, add);
    op!(mul_assign, Mul, *=, mul);
    op!(sub_assign, Sub, -=, sub);
}

/*
mod test {
    use std::iter;
    macro_rules! test {
        ($func: ident, $x: expr, $y: expr, $z: expr) => {
            #[test]
            fn $func() {
                for size in 0_usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);

                    assert_eq!(x, z)
                }
            }
        };
    }

    test!(add_assign, 1_u32, 2_u32, 3_u32);
    test!(mul_assign, 2_u32, 3_u32, 6_u32);
    test!(sub_assign, 3_u32, 2_u32, 1_u32);
    
}
*/

use std::iter;
macro_rules! run {
    ($func: ident, $x: expr, $y: expr, $z: expr) => {
        fn $func() {
            for size in 0_usize..10 {
                let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                let y: Vec<_> = iter::repeat($y).take(size).collect();
                let z: Vec<_> = iter::repeat($z).take(size).collect();

                play::$func(&mut x, &y);
                println!("x:\n{:?}\n", x);
                println!("y:\n{:?}\n", y);
                println!("z:\n{:?}\n", z);

                assert_eq!(x, z)
            }
        }
    };
}

fn main() {

    run!(add_assign, 1_u32, 2_u32, 3_u32);
    run!(mul_assign, 2_u32, 3_u32, 6_u32);
    run!(sub_assign, 3_u32, 2_u32, 1_u32);

    add_assign();
    mul_assign();
    sub_assign();
}