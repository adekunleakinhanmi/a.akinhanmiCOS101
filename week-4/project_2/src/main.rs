//A program to determine the annual incentive of an employee.
use std::io;
fn main() {
    println!("Enter your age ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let age:u32=input2.trim().parse().expect("Failed to input");


    println!("Are you experienced? (true or false)");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let xp:bool=input1.trim().parse().expect("Failed to input");

    if xp== true  && age>=40
    {
        println!("Congratulations! You are entitled to an incentive of 1,560,000 per month");
    }
    else if xp== true && age<40 && age>=30
    {
        println!("Congratulations! You are entitled to an incentive of 1,480,000 per month");
    }
    else if xp== true && age<28
    {
        println!("Congratulations! You are entitled to an incentive of 1,300,000 per month" );
    }
    else if xp== false 
    {
        println!("You are entitled to an incentive of 100,000 per month");
    }
    else
    {
        println!("Sorry, you are not eligible for an incentive");
    }

}
