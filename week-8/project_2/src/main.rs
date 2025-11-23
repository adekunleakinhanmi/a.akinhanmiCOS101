use std::io; 
fn main(){
    let mut candidates:Vec<(String,u32)>=Vec::new();
    let mut input = String::new();

    println!("How many candidates are you interviewing?");
    io::stdin().read_line(&mut input).unwrap();
    let total:usize=input.trim().parse().unwrap();

    for i in 1..=total{
        input.clear();
        println!("Enter candidate {} name:",i);
        io::stdin().read_line(&mut input).unwrap();
        let name= input.trim().to_string();

        input.clear();
        println!("Enter {}'s years of experience:",name);
        io::stdin().read_line(&mut input).unwrap();
        let years:u32=input.trim().parse().unwrap();

        //Store as a tuple
        candidates.push((name,years));
    }

    //Find the candidate with the highest experience
    let mut highest = &candidates[0];

    for candidate in &candidates{
        if candidate.1>highest.1{
            highest=candidate;
        }
    }

    println!("\nThe person with the highest years of programming experience is {} with {} years",highest.0,highest.1);
}
