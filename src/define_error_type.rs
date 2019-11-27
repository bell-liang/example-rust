use std::error;
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;
#[derive(Debug, Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

type ResultBox<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;
impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first itrm to double")
    }
}
impl error::Error for EmptyVec {
    fn description(&self) ->&str {
        "invalid first item to double"
    }
    fn cause(&self) -> Option<&(dyn error::Error)> {
        None
    }
}

fn double_first_2(vec: Vec<&str>) -> ResultBox<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into())
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into())
                .map(|i| 2 * i)
        })
}

fn print_2(result: ResultBox<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn double_first_3(vec: Vec<&str>) -> ResultBox<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

type ResultErrors<T> = std::result::Result<T, DoubleErrorEnum>;
#[derive(Debug, Clone)]
enum DoubleErrorEnum {
    EmptyVec,
    Parse(ParseIntError),
}
impl fmt::Display for DoubleErrorEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleErrorEnum::EmptyVec => 
                write!(f, "please use a vector with at least one element"),
            DoubleErrorEnum::Parse(ref e) => e.fmt(f),
        }
    }
}
impl error::Error for DoubleErrorEnum {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleErrorEnum::EmptyVec => None,
            DoubleErrorEnum::Parse(ref e) => Some(e),
        }
    }
}

impl From<ParseIntError> for DoubleErrorEnum {
    fn from(err: ParseIntError) -> DoubleErrorEnum {
        DoubleErrorEnum::Parse(err)
    }
}

fn double_first_4(vec: Vec<&str>) -> ResultErrors<i32> {
    let first = vec.first().ok_or(DoubleErrorEnum::EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print_3(result: ResultErrors<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print_2(double_first_2(numbers));
    print_2(double_first_2(empty));
    print_2(double_first_2(strings));

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print_2(double_first_3(numbers));
    print_2(double_first_3(empty));
    print_2(double_first_3(strings));

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print_3(double_first_4(numbers));
    print_3(double_first_4(empty));
    print_3(double_first_4(strings));
}