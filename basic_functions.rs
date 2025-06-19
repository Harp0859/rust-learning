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

fn get_lucky_num() -> i32 
{
    let num = 8;
    num
}

fn add_2_num(a: i32, b: i32) -> i32
{
    let sum = a+b;
    sum

}

fn celsius_to_farh(c: f64) -> f64
{

  let f = (c*1.8)+ 32 as f64;
  f
}

fn f_to_c(f: f64) -> f64
{
   
   let c = (f-32 as f64)/1.8;
   return c
   
}
fn main()
{
let c1 = 25.3;
    println!("The celsius to f for {}`C is {}`F", c1, celsius_to_farh(25.8) );
    
    let c2 = 0.0;
    let f2 = celsius_to_farh(c2);
    println!("The celsius to f for {}`C is {}`F", c2 ,f2);
    
    let f = 98.6;
    let c3 = f_to_c(f);
    println!("The f to c for {}'f is {}'c", f, c3);
    
    
    let lucky = get_lucky_num();
    println!("lucky num is {}", lucky);
    
    println!("lucky num is {}",get_lucky_num());
    
    let sum_of_2 = add_2_num(1,4);
    println!("sum of 2 num is {}", sum_of_2);
    
    println!("sum of 2 num is {}", add_2_num(1,9));
    

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
