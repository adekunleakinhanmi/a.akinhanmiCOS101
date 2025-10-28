use std::io;


fn main() {
    loop {
        println!("Enter 'n' to quit or any other alphabet to continue: ");
        let mut input0=String::new();
        io::stdin().read_line(&mut input0).expect("Not a valid string");
        let quit:char = input0.trim().parse().expect("Not a valid number");

        if quit=='n'{
            break;
        }else{
            println!("Enter the Principal: ");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let p:f32 = input1.trim().parse().expect("Not a valid number");

        println!("Enter the Rate to 1d.p: ");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let r:f32 = input2.trim().parse().expect("Not a valid number");

        println!("Enter the Time to 1d.p: ");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Not a valid string");
        let t:f32 = input1.trim().parse().expect("Not a valid number");


        let a:f32= p*(1.0+(r/100.0)).powf(t);
        let ci:f32= a-p;


        println!("The compound interest is {}",ci);
        println!("The Total Amount is {} \n",a);
        }


        

    }
   

    
    



}
