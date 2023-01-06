fn main() {
    // Gets the first two arguments as integers and adds them safely by checking for overflow
    // (or prints an error if one of them is not an integer).
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Error: Expected 2 arguments, got {}", args.len() - 1);
        std::process::exit(1);
    }
    let x: i32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: First argument is not an integer");
            std::process::exit(1);
        }
    };
    let y: i32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Second argument is not an integer");
            std::process::exit(1);
        }
    };
    match x.checked_add(y) {
        Some(sum) => println!("{}", sum),
        None => {
            println!("Error: Overflow while adding {} and {}", x, y);
            std::process::exit(1);
        }
    }
}
