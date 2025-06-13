fn main(){
    
    let mut player_gold = 100;
    println!("gold the player has = {}", player_gold);
    player_gold = player_gold+50;
     println!("gold the player has currently = {}", player_gold);
     player_gold = player_gold-25;
    println!("gold the player has currently = {}", player_gold);
    
    let mut age: i32 = 10;
let name: &str = "hari";
let city: &str ="blr";
let height: f64 = 175.5;
let is_real: bool = true;
    println!("hey am {}, and my age is {} and i live in {}. Im {} cm's tall and its {}", name, age, city, height, is_real);
    age = age+90;
    println!("age is {}",age);
    
    let height_age = age as f64 * height;
     println!("product of height and age is {}",height_age);
     
     let a = 10;
     let b = 3;
     let m = a%b;
     println!("modulo = {}",m);
  
}
