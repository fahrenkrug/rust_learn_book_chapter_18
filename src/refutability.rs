pub fn run() {
    let o: Option<&str> = None;
    // This line breaks the compile with the following warning:
    // refutable pattern in local binding: `None` not covered
    // let Some(text) = o;
    // using an if let will work
    if let Some(text) = o {
        println!("Oh yes - 1! {}", text)
    }
    let o = Some("Great!");
    if let Some(text) = o {
        println!("Oh yes - 2! {}", text)
    }
    // This next block will compile but gives a warning:
    // warning: irrefutable `if let` pattern
    if let x = 5 {
        println!("Oh yeah, we have the x: {}", x);
    }
}