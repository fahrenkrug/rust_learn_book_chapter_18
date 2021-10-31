pub fn run() {
    simple_match_example();
    if_let_example();
    while_let_example();
    for_loop_pattern();
    let_patterns();
    function_parameter_patterns();
}

fn simple_match_example() {
    find_findus("Petterson");
    find_findus("Findus");
}

fn find_findus(x: &str) {
    match x {
        "Findus" => println!("Yes, we found him"),
        _ => println!("No, that's not him.")
    }
}

fn if_let_example() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color {} as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color.");
        } else {
            println!("Using orange as the background color.");
        }
    } else {
        println!("Using blue as the background color.");
    }
}

fn while_let_example() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(x) = stack.pop() {
        println!("Popped from stack: {}", x);
    }
}

fn for_loop_pattern() {
    let v = vec!['a', 'b', 'c'];
    // Enumerate creates a tuple here. The tuples content (index and value) are destructured in
    // the line with the for loop.
    for (index, value) in v.iter().enumerate()  {
        println!("{} is at index {}.", value, index);
    }
}

// let PATTERN = EXPRESSION;
fn let_patterns() {
    // the expression 5 is matched to the pattern x.
    let x = 5;
    println!("x is {}", x);
    // tuple destructuring
    let (x,y,z) = (6,7,8);
    println!("x is {}, y is {}, z is {}", x, y, z);
    // Despite to other languages, this isn't possible:
    // let (x,y) = (6,7,8);
}

fn function_parameter_patterns() {
    foo(3);
    let point = (76,54);
    print_coordinates(&point);
}

fn foo(x: i32) {
    println!("So fuuu {}", x);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x,y);
}
