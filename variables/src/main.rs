use std::io;

pub fn my_function() {
    println!("My function was called")
}

fn main() {
    // VARIABLES
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // # constants
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("the value of THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");
    // # shadowing, we shadow the value of x in different scopes
    let x = x+1;
    println!("The value of x is: {x}");
    {
        let x = x*2;
        println!("The inner scope value of x is: {x}");
    }
    println!("But outside scope of the value of x is still: {x}");
    // change type but it wont work if we declare space as mutable since it is not possible to mutate type
    let spaces = "   ";
    let spaces = spaces.len();

    // DATA TYPES: Rust is statically typed, so every variable's type should be clear in compile time

    // ## scalars types
    let boolean: bool = true;
    let integer: i64 = 5000*100;
    let float: f64 = 45.2/123.2;
    let divint = 256/88;
    println!("note that using integers 256/88 is equal to {divint} because it rounds to closest integer");
    let remainder: i64 = 46%5;
    // note that chars have single quotes whereas strings have double quotes, chars are also unicode
    let character: char = 'b';
    let stringa = "ciao";

    // ## compound types
    // # tuples group toghether different vars types, they cant grow or shrink size
    let tup: (i32,f32,char) = (666, 32.12, 'ę');
    let (x,y,z) = tup;
    println!("using tuple (i32,f32,char) = (666, 32.12, 'ę') and letting\n(x,y,z) = tup let us print x as {x}. This is calle shrinking\n you can also access individuals elements like tup.2");
    let len_tup = tup.2;
    // # arrays: arrays have a single type and cant grow or shrink size. Use arrays for fixed size, unstead for growing things use vectors
    let a = [1,2,3,4,5,6,7];
    // to specify type and size using semicolon
    let arr: [f32; 5] = [1.1,2.2,3.3,4.4,5.5];
    // or same value for given size using semicolon
    let same_size = ['3';5];
    // complex example
    let mut arr: [f32;5]=[0.0,0.0,0.0,0.0,0.0];
    arr[0] = 1.1;
    let first = arr[0];
    let second = arr[1];
    println!("set arr element 0 to be {first} and element 1 to be {second}");

    loop {
        let a = [1, 2, 3, 4, 5];
        println!("Please e ̑nter an array index.");
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = match index
            .trim()
            .parse(){
                Ok(number) => number,
                Err(_) => {
                    println!("mi hai inpanicato duro");
                    break;
                }
            };

        let element = a[index];
        println!("The value of the element at index {index} is: {element}");

        my_function()

    }
}
