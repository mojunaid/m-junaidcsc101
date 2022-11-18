

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();


    println!("Enter your age");
    io::stdin().read_line(&mut input1).expect("Not a valid String Value");
    let age:i32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter your years of experience: ");
    io::stdin().read_line(&mut input2).expect("Not a valid String Value");
    let experience:i32 = input2.trim().parse().expect("Not a valid number");

    if age >= 40 && experience >= 2
    {
        println!("you are entitled to N1,560,000 month");
    }
    else if age > 30 && age < 40 && experience >= 2
    {
        println!("You are entitled to N1,480,000 per month", );
    }
    else if age < 28 && experience >= 2
    {
        println!("You are entitled to N1,300,000 per month");
    }
    else if experience < 2
    {
        println!("You are entitled to N100,000 per month");
    }

}
