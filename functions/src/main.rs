//// FUNCTIONS
fn main() {
    println!("EXECUTING another_function!");
    another_function(1, 2);
    let fiv = five();
    println!("EXECUTING five! with output {fiv}");
    let sum = plus_one(five());
    println!("EXECUTING plus_one! with input five() {sum}");
    println!("EXECUTING conditionals and returns :{}", conditionals(7));
    println!("EXECUTING loop and returns");
    looping(7);
    println!("EXECUTING whiling and returns");
    whiling(7);
    // println!("EXECUTING fibonacci and returns {}", fibonacci(3));
}

fn another_function(x: i32, y: i32) {
    // Expressions (returns smt) VS statements (does not return)
    // this is a statement (let) and it will not return a value! so running let x = (let y= 1;) will fail
    let output = x + y;
    // but it is possible to do that
    let example = {
        let y = -1;
        println!("inner y is {y}");
        // IMPORTANT: note that next expression does not contain a semicolon, this is done to make it a expression, with a semicolon it would be a stattement
        // leading to a fucking error and panic
        x + y
    };
    println!("My function! with x={x} and y={y} will sum {output}\nnote that example is {example}");
}

//// FUNCTIONS RETURNS
// FUNCTIONS THAT returns a value use ->
// note that semicolon misses too, since writing 5; would make it an expression
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}

//// CONDITIONALS
fn conditionals(x: i32) -> i32 {
    let booli = true;
    if x % 2 == 0 {
        println!("Number is divisible by 4");
    } else if x % 3 == 0 {
        println!("Number is divisible by 4");
    } else {
        println!("fottiti nel culo");
    }
    let out = if x == 7 { true } else { false };
    if out {
        println!("You entered 7 nice job");
        return 7;
    }
    return -1;
}

//// LOOPS
// three tipes: loop, for, while

fn looping(mut x: i32) {
    // loop runs forever until explicitly stopped
    loop {
        if x <= 0 {
            break;
        } else {
            x -= 1;
            println!("x in loop is: {}", x)
        }
    }
    println!("Exited loop");
    let mut counter = 0;
    let out = loop {
        counter += 1;
        if counter == 20 {
            //  note that break can also return a value
            break counter * 2 - 1;
        }
    };
    println!("The value of counter is {out}\nLets Stop a loop with labels");
    let mut count = 0;
    // Give label counting_up to loop note that break stops the outer loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn whiling(x: i32) {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for loop is amazing
    let mut arra = [1, 2, 3, 4, 5];
    let mut index = 0;
    for i in arra {
        println! {"{index}th element of arra is {i}"};
        index += 1;
    }
    println!("arra len {}", arra.len());
    for i in (0..arra.len()).rev() {
        println!("{i}th element of arra is {}", arra[i]);
    }
}

// fn fibonacci(x:i64) -> i64{
//     let mut arra = [1,1];
//     let mut counter: i64 = 3;
//     while counter <= x {
//         arra[counter-1] = arra[counter-3]+arra[counter-2];
//         counter += 1;
//     }
//     let out : i64 = if x <= 2 {arra[0] + arra[1]} else {arra[counter-1]};
//     return out;
// }
