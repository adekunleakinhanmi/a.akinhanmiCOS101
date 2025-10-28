use std::io;

fn main() {
    let mut name= String::new();
    let mut input2= String::new();
    let mut input3= String::new();
    let mut input4= String::new();

    println!("Enter your name:");
    io::stdin().read_line(&mut name).expect("Not a valid string");

    println!("Enter your MTH 101 test score: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let score_1:f32 = input2.trim().parse().expect("Not a valid number");
    if score_1<0.00{
    println!("Please enter a valid score from 0 to 100.");
}
    else if score_1>100.00{
    println!("Error, please enter a valid score from 0 to 100");
}
    println!("Enter your COS 101 score: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let score_2:f32 = input3.trim().parse().expect("Not a valid number");
    if score_2<0.00{
    println!("Please enter a valid score from 0 to 100.")
}
    else if score_2>100.00{
    println!("Error, please enter a valid score from 0 to 100");
}

    println!("Enter your STA 111 score: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let score_3:f32 = input4.trim().parse().expect("Not a valid number");
    if score_3<0.00{
    println!("Error, please enter a valid score from 0 to 100");
}
    else if score_3>100.00{
    println!("Error, please enter a valid score from 0 to 100");
}
    let average:f32 = (score_1+score_2+score_3)/3.00;

    if average>=70.00 && average<=100.00
    {
        println!("Hello {} your grade based on the average of your 3 test scores is A.", name);
    }else if average>=60.00 && average<=69.00
    {
        println!("Hello {} your grade based on the average of your 3 test scores is B.", name);
    }else if average>=50.00 && average<=59.00
    {
        println!("Hello {} your grade based on the average of your 3 test scores is C.",name );
    }else if average>=45.00 && average<=49.00
    {
        println!("Hello {} your grade based on the average of your 3 test scores is D.", name);
    }else if average>=0.00 && average<=44.00
    {
        println!("Hello {} your grade based on the average of your 3 test scores is F.", name);
    }else 
    {
        println!("Error found, please confirm you have put in the correct test scores.", );
    }

}
