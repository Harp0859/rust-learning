fn main()
{
    let mut numbers: Vec<i32> = Vec::new();
    println!("vector {:?}", numbers);
    
    numbers.push(1); 
    println!("vector {:?}", numbers);
    
    let mut name: Vec<&str> = vec!["hari", "harp"];
    println!("vector {:?}", name);
    
    name.push("abishek");
    println!("vector {:?}", name);
    
    name.pop();
    println!("vector {:?}", name);
    
    name.push("abishek");
    println!("vector {:?}", name);
    
    let colors = vec!["pink","black","blue","orange"];
    println!("vector {}", colors[0]);
    
    let sports = vec!["badminton", "cricket", "hockey", "tennis"];
    println!("sports list");
    for sports in sports {
        println!(" - {}", sports);
    }
    
    let mut my_wishlist: Vec<&str> = vec!["phone","car","bike"];
    println!("my wishlist {:?} ", my_wishlist);
    my_wishlist.push("crocs");
    my_wishlist.push("bottle");
    println!("my wishlist {:?} ", my_wishlist);
    println!("my wishlist");
    for my_wishlist in my_wishlist {
        println!(" - {}", my_wishlist);
    }
    
    
}
