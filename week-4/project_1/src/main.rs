//A program to find the roots of a quadratic equation.
use std::io;

fn main() {
    println!("Enter the value of a");
    let mut input1=String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32= input1.trim().parse().expect("Failed to input");

    println!("Enter the value of b");
    let mut input2=String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32= input2.trim().parse().expect("Failed to input");

    println!("Enter the value of c");
    let mut input3=String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f32= input3.trim().parse().expect("Failed to input");

    let n:f32=2.0;
    let discriminant:f32=((b.powf(n))-(4.0*a*c)).sqrt();
    println!("The discriminant is {}",discriminant);
    if discriminant>0.0 
    {
        let x1=((b*-1.0)+discriminant)/(2.0*a);
        let x2=((b*-1.0)-discriminant)/(2.0*a);
        println!("There are two real and distinct roots which are {} and {}",x1, x2);
    }else if discriminant==0.0
    {
        let x1=(b*-1.0)/(2.0*a);
        println!("There is only one real root which is {}", x1);
    }else {
        println!("There are no real roots to this equation.");
    }
}
