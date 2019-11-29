fn main() {
    println!("以下为 traits 部分！");
    struct Sheep {
        naked: bool,
        name: &'static str
    }
    trait Animal {
        fn new(name: &'static str) -> Self;
        
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }
    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }
        fn shear(&mut self) {
            if self.is_naked() {
                println!("{} is already naked...", self.name);
            } else {
                println!("{} gets a haircut!", self.name);

                self.naked = true;
            }
        }
    }
    impl Animal for Sheep {
        fn new(name: &'static str) -> Sheep {
            Sheep {
                name: name,
                naked: false
            }
        }
        fn name(&self) -> &'static str {
            self.name
        }
        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "baaaaah?"
            } else {
                "baaaaah!"
            }
        }
        fn talk(&self) {
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();

    println!("以下为 重载运算符 部分！");
    use std::ops;
    struct Fooo;
    struct Bar;
    #[derive(Debug)]
    struct FoooBar;
    #[derive(Debug)]
    struct BarFooo;
    impl ops::Add<Bar> for Fooo {
        type Output = FoooBar;

        fn add(self, _rhs: Bar) -> FoooBar {
            println!("> Fooo.add(Bar) was called");

            FoooBar
        }
    }
    impl ops::Add<Fooo> for Bar {
        type Output = BarFooo;

        fn add(self, _rhs: Fooo) -> BarFooo {
            println!("> Bar.add(Fooo) was calles");

            BarFooo
        }
    }

    println!("Fooo + Bar = {:?}", Fooo + Bar);
    println!("Bar + Fooo = {:?}", Bar + Fooo);

    println!("以下为 traints 之 iteratiors 部分！");
    struct Fibonacci {
        curr: u32,
        next: u32,
    }
    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            let new_next = self.curr + self.next;

            self.curr = self.next;
            self.next = new_next;

            Some(self.curr)
        }
    }
    fn fibonacci() -> Fibonacci {
        Fibonacci {
            curr: 0,
            next: 1,
        }
    }
    let mut sequence = 0..3;
    println!("Four consecutive 'next' calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterate through 0..3 using 'for'");
    for i in 0..3 {
        println!("> {}", i);
    }

    println!("The first four term od the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1_u32, 3, 3, 7];
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }

    println!("以下为 impl Trait 部分！");
    /*
    use std::iter;
    use std::vec::IntoIter;
    fn combine_vecs_explicit_return_type<'a>(
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
        v.into_iter().chain(u.into_iter()).cycle()
    }
    fn combine_vecs<'a>(
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> impl Iterator<Item=i32> {
        v.into_iter().chain(u.into_iter()).cycle()
    }
    let v: Vec<i32> = vec![0, 1, 2];
    let u: Vec<i32> = vec![3, 4, 5];
    for i in combine_vecs_explicit_return_type(v, u) {
        println!("{}", i);
    }
    let v: Vec<i32> = vec![0, 1, 2];
    let u: Vec<i32> = vec![3, 4, 5];
    for i in combine_vecs(v, u) {
        println!("{}", i);
    }
    */
    fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
        let closure = move |x: i32| { x + y };
        closure
    }
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);

    fn double_positive<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
        numbers
            .iter()
            .filter(|x| x > &&0)
            .map(|x| x * 2)
    }
    let v: Vec<i32> = vec![-1, -5, -23, 0, 3, 4, 2, -3];
    for i in double_positive(&v) {
        println!("{}", i);
    }

    println!("以下为 traits 之 clone 部分！");
    #[derive(Debug, Clone, Copy)]
    struct Nil1;

    // A tuple struct with resources that implements the `Clone` trait
    #[derive(Clone, Debug)]
    struct Pair1(Box<i32>, Box<i32>);
    // Instantiate `Nil`
    let nil = Nil1;
    // Copy `Nil`, there are no resources to move
    let copied_nil = nil;

    // Both `Nil`s can be used independently
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    // Instantiate `Pair`
    let pair = Pair1(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // Copy `pair` into `moved_pair`, moves resources
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);

    // Error! `pair` has lost its resources
    //println!("original: {:?}", pair);
    // TODO ^ Try uncommenting this line
    
    // Clone `moved_pair` into `cloned_pair` (resources are included)
    let cloned_pair = moved_pair.clone();
    // Drop the original pair using std::mem::drop
    drop(moved_pair);

    // Error! `moved_pair` has been dropped
    //println!("copy: {:?}", moved_pair);
    // TODO ^ Try uncommenting this line

    // The result from .clone() can still be used!
    println!("clone: {:?}", cloned_pair);

    println!("以下为 traits 之 supertraits 部分！");
    trait Human {
        fn name(&self) -> String;
    }
    trait Student: Human {
        fn university(&self) -> String;
    }
    trait Programmer {
        fn fav_language(&self) -> String;
    }
    trait CompSciStudent: Programmer + Student {
        fn git_usrname(&self) -> String;
    }
    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}. My favorite programming language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_usrname()
        )
    }

    struct Drogan {
        name: &'static str,
        university: &'static str,
        fav_language: &'static str,
        git_username: &'static str,
    }
    impl Human for Drogan {
        fn name(&self) -> String {
            self.name.to_string()
        }
    }
    impl Student for Drogan {
        fn university(&self) -> String {
            self.university.to_string()
        }
    }
    impl Programmer for Drogan {
        fn fav_language(&self) -> String {
            self.fav_language.to_string()
        }
    }
    impl CompSciStudent for Drogan {
        fn git_usrname(&self) -> String {
            self.git_username.to_string()
        }
    }
    let drogan = Drogan {
        name: "小明",
        university: "耶鲁大学",
        fav_language: "rust",
        git_username: "xiaoming@qq.com"
    };
    println!("{}", comp_sci_student_greeting(&drogan));

    println!("以下为 disambiguating overlapping traits 部分！");
    trait UsernameWidget {
        fn get(&self) -> String;
    }
    trait AgeWidget {
        fn get(&self) -> u8;
    }
    struct Form {
        username: String,
        age: u8,
    }
    impl UsernameWidget for Form {
        fn get(&self) -> String {
            self.username.clone()
        }
    }
    impl AgeWidget for Form {
        fn get(&self) -> u8 {
            self.age
        }
    }
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };
    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}