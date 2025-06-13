fn main(){
let age = 17;
if age>=18 
{ println!("he is adult"); }
else 
{ println!("he is not an adult"); }

let temp = 60;

if temp==21
{ println!("wow what a great day"); }
else if temp <= 21
{ println!("wow") }
else
{ println!("na") }

let is_raining = true;
let time_of_day = "morning";
let a = 10;
let b = 89;
let c = b/a;

if c < a
{ println!("result is {}", c); }
else 
{ println!("result is {}", b); }

if time_of_day != "night"
{ println!("its mrng") }
else if time_of_day == "morning" 
{ println!(" okay shut") }

if is_raining == false
{ println!("why no rain?") }
else
{ println!("ok wow") }




let game_score = 85;

if game_score == 100
{
    println!("Perfect score! You are a master!");
}
else if game_score >= 90
 {
    println!("Excellent! You crushed it!");
} 
else if game_score >= 70
 {
    println!("Good job! You passed!");
} 
else 
 {
    println!("Keep practicing! You'll get there!");
} 
}
