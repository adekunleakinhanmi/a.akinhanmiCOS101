use std::io;
//I would have done this in my test if i had more time.
fn main() {
    loop{


    println!("Code          Item            Price");
    println!(" L            Laptop          550,000");
    println!(" M            Monitor         120,000");
    println!(" K            Keyboard        15,000");
    println!(" H            Headset         25,000\n");

    let l_cost:f32=550_000.0;
    let m_cost:f32=120_000.0;
    let k_cost:f32=15_000.0;
    let h_cost:f32=25_000.0;
  


    println!("Enter an item code: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let item_code:char = input1.trim().parse().expect("Not a valid number");

    println!("Enter the Quantity: ");
    let mut input2= String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let q:f32 = input2.trim().parse().expect("Not a valid number");


    if item_code == 'l'
    {
        let total:f32= l_cost*q;
        println!("\nThe total cost is {}", total);
        if total>500_000.0{
            let discount= 0.07*total;
            let amount= total-discount;
            println!("The total amount payable after discount is {} \n", amount);
        }
    }else if item_code == 'm'
    {
        let total:f32= m_cost*q;
        println!("The total cost is {}",total);
        if total>500_000.0 {
            let discount= 0.07*total;
            let amount= total-discount;
            println!("The total amount payable after discount is {} \n", amount);
        }

    }else if item_code== 'k'
    {
        let total:f32= k_cost*q;
        println!("The total cost is {}",total );
        if total>500_000.0{
            let discount= 0.07*total;
            let amount= total-discount;
            println!("The total amount payable after discount is {} \n", amount);
        }
    }else if item_code== 'h' {
        let total:f32= h_cost*q;
        println!("The total cost is {}",total );
        if total>500_000.0{
            let discount= 0.07*total;
            let amount= total-discount;
            println!("The total amount payable after discount is {} \n", amount);
        }
        }else
        {
            println!("Invalid input, try again.");
        }

        println!("If you want to stop enter 'N'");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Not a valid string");
        let stop:char= input3.trim().to_uppercase().parse().expect("Not a valid number");

        if stop=='N'
        {
            break;
        } 
    }
}

