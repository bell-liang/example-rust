fn main() {
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

    println!("以下为 Rusult 部分！");
    fn multiply_new(first_number_str: &str, second_number_str: &str) -> i32 {
        let first_number = first_number_str.parse::<i32>().unwrap();
        let second_number = second_number_str.parse::<i32>().unwrap();
        first_number * second_number
    }

    let twenty = multiply_new("10", "2");
    println!("double is {}", twenty);
    //let tt = multiply_new("t", "2");
    //println!("double is {}", tt);

    use std::num::ParseIntError;
    fn multiply_second(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        match first_number_str.parse::<i32>() {
            Ok(first_number) => {
                match second_number_str.parse::<i32>() {
                    Ok(second_number) => {
                        Ok(first_number * second_number)
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }
    fn print_new(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }
    let twenty = multiply_second("10", "2");
    print_new(twenty);
    let tt = multiply_second("t", "2");
    print_new(tt);

    fn multiply_third(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
        })
    }
    let twenty = multiply_third("10", "2");
    print_new(twenty);
    let tt = multiply_third("t", "2");
    print_new(tt);

    type AliasedResult<T> = Result<T, ParseIntError>;
    fn multiply_fourth(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
        })
    }
    print_new(multiply_third("10", "2"));
    print_new(multiply_fourth("t", "2"));

    fn multiply_fifth(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        let first_number = match first_number_str.parse::<i32>() {
            Ok(first_number) => first_number,
            Err(e) => return Err(e),
        };
        let second_number = match second_number_str.parse::<i32>() {
            Ok(second_number) => second_number,
            Err(e) => return Err(e),
        };
        Ok(first_number * second_number)
    }
    print_new(multiply_fifth("10", "2"));
    print_new(multiply_fifth("t", "2"));

    fn multiply_sixth(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        let first_number = first_number_str.parse::<i32>()?;
        let second_number = second_number_str.parse::<i32>()?;
        Ok(first_number * second_number)
    }
    print_new(multiply_sixth("10", "2"));
    print_new(multiply_sixth("t", "2"));

    /*
    fn multiply_seventh(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        let first_number = try!(first_number_str.parse::<i32>());
        let second_number = try(!second_number_str.parse::<i32>());
        Ok(first_number * second_number)
    }
    */

    fn double_first(vec: Vec<&str>) -> i32 {
        let first = vec.first().unwrap();
        2 * first.parse::<i32>().unwrap()
    }
    let numbers = vec!["42", "93", "18"];
    //let empty = vec![];
    //let strings = vec!["tofu", "93", "18"];
    println!("The first doubled is {}", double_first(numbers));
    //println!("The first doubles is {}", double_first(empty));
    //println!("The first doubles is {}", double_first(strings));

    fn double_first_2(vec: Vec<&str>) -> Option<AliasedResult<i32>> {
        vec.first().map(|first| {
            first.parse::<i32>().map(|n| 2 * n)
        })
    }
    let numbers = vec!["42", "93", "18"];
    println!("The first doubles is {:?}", double_first_2(numbers));

    fn double_first_3(vec: Vec<&str>) -> AliasedResult<Option<i32>> {
        let opt = vec.first().map(|first| {
            first.parse::<i32>().map(|n| 2 * n)
        });
        
        let opt = opt.map_or(Ok(None), |r| r.map(Some))?;
        Ok(opt)
    }
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    println!("The first doubled is {:?}", double_first_3(numbers));
    println!("The first doubles is {:?}", double_first_3(empty));
    println!("The first doubles is {:?}", double_first_3(strings));
}