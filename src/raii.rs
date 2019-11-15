fn creat_box() {
    let _box1 = Box::new(3_i32);
}

struct ToDrop;
impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}

fn main() {
    let _box2 = Box::new(5_i32);

    {
        let _box3 = Box::new(4_i32);
    }

    for _ in 0_u32..1_000 {
        creat_box();
    }

    #[allow(unused_variables)]
    let x = ToDrop;
    println!("Made a ToDrop!");

    println!("以下为 所有权 部分！");
    // _Stack_ allocated integer
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;
    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it.
    
    // Error! `a` can no longer access the data, because it no longer owns the
    // heap memory
    //println!("a contains: {}", a);
    // TODO ^ Try uncommenting this line

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    // println!("b contains: {}", b);
    // TODO ^ Try uncommenting this line
    println!("以下为 不可变性 部分！");
    let immutable_box = Box::new(5_u32);
    println!("immutable_box contains {}", immutable_box);
    let mut mutable_box = immutable_box;
    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);
    
    println!("以下为 借用 部分！");
    fn eat_box_i32(boxed_i32: Box<i32>) {
        println!("Destroying box that contains {}", boxed_i32);
    }
    fn borrow_i32(borrowed_i32: &i32) {
        println!("This int is : {}", borrowed_i32);
    }
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);
    {
        let _ref_to_i32: &i32 = &boxed_i32;
        borrow_i32(_ref_to_i32);
    }
    eat_box_i32(boxed_i32);

    println!("以下为 可变借用 部分！");
    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    struct Book {
        // `&'static str` is a reference to a string allocated in read only memory
        author: &'static str,
        title: &'static str,
        year: u32,
    }

    // This function takes a reference to a book
    fn borrow_book(book: &Book) {
        println!("I immutably borrowed {} - {} edition", book.title, book.year);
    }

    // This function takes a reference to a mutable book and changes `year` to 2014
    fn new_edition(book: &mut Book) {
        book.year = 2014;
        println!("I mutably borrowed {} - {} edition", book.title, book.year);
    }
    // Create an immutable Book named `immutabook`
    let immutabook = Book {
        // string literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // Create a mutable copy of `immutabook` and call it `mutabook`
    let mut mutabook = immutabook;
    
    // Immutably borrow an immutable object
    borrow_book(&immutabook);

    // Immutably borrow a mutable object
    borrow_book(&mutabook);
    
    // Borrow a mutable object as mutable
    new_edition(&mut mutabook);
    
    // Error! Cannot borrow an immutable object as mutable
    //new_edition(&mut immutabook);
    // FIXME ^ Comment out this line

}