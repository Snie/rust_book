fn main() {
    {
        let mut s: String = String::from("hello");
        s.push_str(", world!"); // push_str() appends a literal to a String
        println!("Value of s is: \n{}", s)
    } // s ceases to exist (drop is called), because the rust runtime automatically calls the drop method on s, drop is a method usually implemented in crates

    // here we bind the data on the heap of s to c, whereas the string structure (pointer, length, capacity) is copied, very similar to C.
    // in this case s is a 'object' composed by (pointer, length, capacity), this is on the stack, whereas the actual string is in the heap
    let s: String = String::from("hello");
    let _c = s;
    // HOWEVER, s goes out of scope, because rust prevents double frees errors
    // this is called 'move'
    // rust never does automatically deep copies, instead only shallow ones

    // if we want to deep copy then use clone() method

    let s: String = String::from("hello");
    let _c = s.clone();

    // what about integers? they are copied anyway since their size is fixed and not variable or unknown as strings
    // so if a type has a drop method, then rust will not accept to copy it, all scalars implement a copy

    // s's value moves into the function and so is no longer valid here
    takes_ownership(s.clone());

    //  x comes into scope x would move into the function, but i32 is Copy, so it's okay to use x afterward
    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();

    // we lost s above in takes_ownership, so lets recreate it
    let mut s: String = String::from("hello");

    // the function returns s, but also takes is ownership by borrowing s, so we use a clone to not lose s
    let s2 = takes_and_gives_back(s.clone());
    println!("s is {} so as s2 {}", s.clone(), s2.clone());

    s = String::from("Hello world");

    // so how can we get s back s? lets return two values, but that's inconvinent...
    let (s1, len) = calculate_length(s.clone());
    println!("The length of '{}' is {}.", s, len);

    // luckily we have References: & which is kind of a pointer in C
    // you use it both in function params and argument
    ref_calculate_length(&s1);
    println!("See variable s1 is still here and is: {}", s1)
    // trying to modify s1 wont work because is not mutable, nor is the func is made for mutable references
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
}

// some_integer will remain here in this context as f doesnt return, but i32 are cloned types
fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
}

// returns only, note that ; is missing
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes
    // scope
    a_string // a_string is returned and moves out to the calling function
}

// tuple returns input, suboptimal
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

// references
fn ref_calculate_length(s: &String) -> usize {
    s.len() // len() returns the length of a String
}

// mutable references
fn change(some_string: &mut String) {
    // concatenate, note that f doesnt return, it just modifies some_string as it has mutable access to the pointer
    some_string.push_str(", world");
}
