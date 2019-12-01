use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, prelude::*};

static NTHREADS: i32 = 10;
static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn main() {
    println!("以下为 多线程 部分！");
    let mut children = vec![];
    for i in 0..NTHREADS {
        children.push(thread::spawn(move | | {
            println!("this is thread number {}", i);
        }));
    }
    for child in children {
        let _ = child.join();
    }
    println!("以下为 多线程处理数据 部分！");
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";
    let mut children = vec![];
    let chunked_data = data.split_whitespace();
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);
        children.push(thread::spawn(move | | -> u32 {
            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();
            println!("processed segment {}, result = {}", i, result);
            result
        }));
    }
    let mut intermediate_sums = vec![];
    for child in children {
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }
    let final_result = intermediate_sums.iter().sum::<u32>();
    println!("Final sum result: {}", final_result);
    let data = "869678977374164718532973\
    2705036495911861322575564723963\
    2975426249628507085623470186085\
    1907960690014725639383979667071\
    0609417278323874766921952380795\
    2578882365254593033303028375849\
    5327135744041048897885734297812\
    6992021643898087354880841372095\
    6532162784246374525898603453748\
    28574668";
    let result: u32 = data.chars()
        .map(|c| c.to_digit(10).expect("should be a digit"))
        .sum();
    println!("Ha ha ha is {}", result);
    println!{"以下为 多进程通道 部分！"};
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();
    for id in 0..NTHREADS {
        let thread_tx = tx.clone();
        let child = thread::spawn(move || {
            thread_tx.send(id).unwrap();
            println!("thread {} finished", id);
        });
        children.push(child);
    }
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        ids.push(rx.recv());
    }
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }
    println!("{:?}", ids);
    println!{"以下为 Path 部分！"};
    let path = Path::new(".");
    let _display = path.display();
    let new_path = path.join("a").join("b");
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
    println!("以下为 File I/O 部分！");
    let path = Path::new("hello.txt");
    let display = path.display();
    let mut file = match File::open(&path)  {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                                            why.description()),
        Ok(_) => println!("{} contains: \n{}", display, s),
    }

    let path = Path::new("lorem_ipsum.txt");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
    if let Ok(lines) = read_lines("./hosts") {
        for line in lines {
            if let Ok(ip) = line {
                println!("ip: {}", ip);
            }
        }
    }
    fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(file_name)?;
        Ok(io::BufReader::new(file).lines())
    }
}