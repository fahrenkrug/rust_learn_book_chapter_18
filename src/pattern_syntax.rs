use crate::pattern_syntax::Message::ChangeColor;

pub fn run() {
    matching_literals();
    matching_named_variables();
    multiple_patterns();
    matching_ranges();
    struct_destructuring();
    enum_destructuring();
    nested_destructuring();
    ignoring_values();
    match_guards();
    at_bindings();
}

fn matching_literals() {
    let x = 1;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("A very big number"),
    }
}

fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("got 50!"),
        Some(y) => println!("matched! y = {:?}", y),
        _ => println!("Default case. x = {:?}", x),
    }
    println!("At the end: x is {:?} and y is {:?}", x, y);
}

fn multiple_patterns() {
    let x = 1;
    match x {
        1 | 2 => println!("x is 1 or 2."),
        3 => println!("x is 3"),
        _ => println!("I don't know what it is...")
    }
}

fn matching_ranges() {
    let x = 5;
    match x {
        1..=5 => println!("x is a small number."),
        _ => println!("I don't know what it is really. But it is bigger than small.")
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("Something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn struct_destructuring() {
    let p = Point {
        x: 7,
        y: 0,
    };
    let Point {x: a, y: b} = p;
    assert_eq!(7,a);
    assert_eq!(0,b);
    let p = Point {
        x: 13,
        y: 12,
    };
    let Point { x, y} = p;
    assert_eq!(13, x);
    assert_eq!(12, y);

    match p {
        Point{ x, y: 0} => println!("On the x axis at {}", x),
        Point{ x: 0, y} => println!("On the y axis at {}", y),
        Point{x, y} => println!("On neither axis ({}, {})", x, y),
    }
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enum_destructuring() {
    let message = ChangeColor(0,160, 255);
    match message {
        Message::Quit => {
            println!("The Quit variant has no data to destructure");
        },
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        },
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) =>
            println!("Changing the color to red {}, green {} and blue {}", r, g, b)
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

fn nested_destructuring() {
    let message = Message2::ChangeColor(Color::Hsv(0,160,255));
    match message {
        Message2::ChangeColor(Color::Rgb(r,g,b)) => {
            println!("Changing the color to red {}, green {} and blue {}", r, g, b)
        },
        Message2::ChangeColor(Color::Hsv(h,s,v)) =>
            println!("Changing the color to hue {}, saturation {} and value {}", h, s, v),
        _ => (),
    }
    // A tuple that holds another tuple and a point
    let ((feet, inches), Point { x,y }) = ((3, 10), Point { x: 3, y: -10});
    println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y);
}

fn ignoring_values() {
    ignoring_foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't override an exinsting setting")
        },
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("Setting value is {:?}", setting_value);

    let numbers = (2,4,8,16,32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", fifth, third, fifth);
        }
    }

    let _x = 19; // I'm not in use but since my name is prefixed with an underscore the compiler
    // still likes me 🥲
    let y = 20;
    println!("I am in use: {}", y);

    // This won't work because _something is actually binding the value while only _ doesn't bind
    // at all.
    /*
    let s = Some(String::from("Hello"));
    if let Some(_s) = s {
        println!("The s is moved into this block here: {}", _s);
    }
    println!("S = {:?}", s);*/

    // And with just the _
    let s = Some(String::from("Hello"));
    if let Some(_) = s {
        println!("Found a string");
    }
    println!("s = {:?}", s);
    ignoring_all_the_rest();
    ignoring_something_in_between();
}

struct ThreeDimensionPoint {
    x: i32,
    y: i32,
    z: i32,
}
fn ignoring_all_the_rest() {
    let origin = ThreeDimensionPoint {
        x: 0,
        y: 0,
        z: 0,
    };
    match origin {
        ThreeDimensionPoint{ x, ..} => println!("x is {}", x)
    }
}

fn ignoring_something_in_between() {
    let numbers = (1,2,3,4,5,6,7,7);
    match numbers {
        (first, .., last) => println!("First is {} and last is {}", first, last)
    }
    // It has to be clear for the compiler which values to pick. This won't work:
    // (.., second, ..) => {
}

// Using the ignore pattern on a normal function is nonsense but imagine how this can help for a
// trait function
fn ignoring_foo(_: i32, y: i32) {
    println!("I'm so gonna to ignore that x thingy. Y = {}", y);
}

fn match_guards() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("Less than five!"),
        Some(x) => println!("{}", x),
        _ => ()
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Fifty it is!"),
        Some(n) if n == y => println!("Matched {}", n),
        _ => println!("Default case: {:?}", x)
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("YESS!"),
        _ => println!("NOOOO!")
    }
}

enum Message3 {
    Hello { id: i32}
}

fn at_bindings() {
    let msg = Message3::Hello { id: 5};
    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found id in range: {}", id_variable),
        Message3::Hello {
            id: 10..=12
        } => println!("Found id in another range"),
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
}