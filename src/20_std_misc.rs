use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::path::Path;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, prelude::*};
use std::process::{Command, Stdio};
use std::os::unix;
use std::env;
use std::fmt;

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
    println!("以下为 Child processes 部分！");
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
    });
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("rustc succeeded and stdout was : \n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("rustc failed and stderr was :\n{}", s);
    }
    println!("以下为 Pipes 部分！");
    static PANGRAM: &'static str =
        "the quick brown fox jumped over the lazy dog\n";
    let process = match Command::new("wc")
                                                        .stdin(Stdio::piped())
                                                        .stdout(Stdio::piped())
                                                        .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why.description()),
        Ok(process) =>process,
    };
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why.description()),
        Ok(_) => println!("sent pangram to wc"),
    }
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why.description()),
        Ok(_) => println!("wc responded with: \n{}", s),
    }
    println!("以下为 wait 部分！");
    let mut child = Command::new("sleep").arg("1").spawn().unwrap();
    let _result = child.wait().unwrap();
    println!("reached end of main");
    println!("以下为 Filesystem Operations 部分！");
    fn cat(path: &Path) -> io::Result<String> {
        let mut f = File::open(path)?;
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    fn echo(s: &str, path: &Path) -> io::Result<()> {
        let mut f = File::create(path)?;
        f.write_all(s.as_bytes())
    }
    fn touch(path: &Path) -> io::Result<()> {
        match OpenOptions::new().create(true).write(true).open(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    println!("'mkdir a'");
    match fs::create_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }
    println!("'echo hello > a/b.txt'");
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!("'mkdir -p a/c/d'");
    fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!("'touch a/c/e.txt'");
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!("'ln -s ../b.txt a/c/b.txt'");
    if cfg!(target_family = "unix") {
        unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }
    println!("'cat a/c/b.txt'");
    match cat(&Path::new("a/c/b.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }
    println!("'ls a'");
    match fs::read_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            println!("> {:?}", path.unwrap().path());
        },
    }
    println!("'rm a/c/e.txt'");
    fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!("'rmdir a/c/d'");
    fs::remove_dir("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!("以下为 Program arguments 部分！");
    let args: Vec<String> = env::args().collect();
    println!("My path is {}", args[0]);
    println!("I got {:?} arguments: {:?}.", args.len()-1, &args[1..]);
    println!("以下为 Argument parsing 部分！");
    fn increase(number: i32) {
        println!("{}", number + 1);
    }
    fn decrease(number: i32) {
        println!("{}", number - 1);
    }
    fn help() {
        println!("usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one.");
    }
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            println!("My name is 'match_args'. Try passing some arguments!");
        },
        // one argument passed
        2 => {
            match args[1].parse() {
                Ok(42) => println!("This is the answer!"),
                _ => println!("This is not the answer."),
            }
        },
        // one command and one argument passed
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // parse the number
            let number: i32 = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                },
            };
            // parse the command
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                },
            }
        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
    println!("以下为 Foreign Function Interface 部分！");
    #[link(name = "m")]
    extern {
        fn csqrtf(z: Complex) -> Complex;
        fn ccosf(z: Complex) -> Complex;
    }
    fn cos(z: Complex) -> Complex {
        unsafe { ccosf(z) }
    }
    let z = Complex { re: -1., im: 0. };
    let z_sqrt = unsafe { csqrtf(z) };
    println!("the square root of {:?} is {:?}", z, z_sqrt);
    println!("cos({:?}) = {:?}", z, cos(z));
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Complex {
        re: f32,
        im: f32,
    }
    impl fmt::Debug for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.im < 0. {
                write!(f, "{}-{}i", self.re, -self.im)
            } else {
                write!(f, "{}+{}i", self.re, self.im)
            }
        }
    }
}