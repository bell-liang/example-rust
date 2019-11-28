#![allow(overflowing_literals)]

fn main() {
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
}