fn main1() {
    
    for countdown_number in (4..5).rev() {
        println!("Countdown: {}", countdown_number);
    }
    println!("Blast off!");
    
}

fn greet_user() { 
    println!("Hello there, brave Rustacean!");
    println!("Welcome to our amazing program!");
}

fn greet_user_1(name: &str)
{
    println!("name: {}", name);
}

fn add(a: i32, b: i32)
{
let sum = a+b;
    println!("added value = {}", sum);
}

fn calculate_rectangle_area(l: f64, b: f64)
{
    let area = l*b;
    println!("area of the rectangle is {}", area);
}

fn main() { 
    println!("Starting the program...");

    greet_user(); 
    greet_user(); 
    greet_user(); 
    main1();
    greet_user_1("hari");
    add(3,5);
    calculate_rectangle_area(9.2,7.6);
calculate_rectangle_area(9.42,7.6);
    println!("Program finished.");
}
