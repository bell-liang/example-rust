fn main() {
    println!("以下为 use 部分！");
    use my::deeply::nested::function as other_function;

    fn use_function() {
        println!("called `use_function()`");
    }

    mod deeply {
        pub mod nested {
            pub fn function() {
                println!("called `deeply::nested::function()`");
            }
        }
    }
    // Easier access to `deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use deeply::nested::function;
        function();

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        println!("Leaving block");
    }

    use_function();
    #[allow(dead_code)]
    mod cool {
        pub fn function() {
            println!("called `cool::function()`");
        }
    }

    #[allow(dead_code)]
    mod my_super {
        fn function() {
            println!("called `my::function()`");
        }

        mod cool {
            pub fn function() {
                println!("called `my::cool::function()`");
            }
        }

        pub fn indirect_call() {
            // Let's access all the functions named `function` from this scope!
            print!("called `my::indirect_call()`, that\n> ");

            // The `self` keyword refers to the current module scope - in this case `my`.
            // Calling `self::function()` and calling `function()` directly both give
            // the same result, because they refer to the same function.
            self::function();
            function();

            // We can also use `self` to access another module inside `my`:
            self::cool::function();

            // The `super` keyword refers to the parent scope (outside the `my` module).
            //super::other_function;

            // This will bind to the `cool::function` in the *crate* scope.
            // In this case the crate scope is the outermost scope.
            {
                use cool::function as root_function;
                root_function();
            }
        }
    }
}