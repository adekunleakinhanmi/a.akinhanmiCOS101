use std::io;
fn main() {
    loop{
        println!("\n        Menu                        Price", );
        println!("P = Poundo Yam/Edikang ikong Soup    -N3,200");
        println!("F = Fried Rice and Chicken           -N3,000");
        println!("A = Amala & Ewedu Soup               -N2,500");
        println!("E = Eba & Egusi Soup                 -N2,000");
        println!("W = White Rice & Stew                -N2,500\n");

        println!("Place your order or enter n to quit: ");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let menu:char = input1.trim().to_uppercase().parse().expect("Not a valid number");

        if menu == 'N'{
        break;
        }else {
        println!("Enter the portions of food you want: ");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let q:f32= input2.trim().parse().expect("Not a valid number");

            if menu == 'P'{
            let total:f32 = q*3_200.0;
            println!("\nThe total due for your order is {}",total);
                if total>10_000.0{
                    let discount:f32= 0.05*total;
            let new_total:f32 = total-discount;
            println!("The discounted total is {}",new_total);
        }
        }else if menu == 'F'{
            let total:f32 = q*3_000.0;
            println!("\nThe total due for your order is {}",total);
            if total>10_000.0{
                let discount:f32= 0.05*total;
            let new_total:f32 =total-discount;
            println!("The discounted total is {}",new_total);
        }
        }
        else if menu == 'A'{
            let total:f32 = q*2_500.0;
            println!("\nThe total due for your order is {}",total);
            if total>10_000.0{
            let discount:f32 = 0.05*total;
            let new_total:f32= total-discount;
            println!("The discounted total is {}",new_total);
        }
        }
        else if menu == 'E'{
            let total:f32 = q*2_000.0;
            println!("\nThe total due for your order is {}",total);
            if total>10_000.0{
            let discount:f32 = 0.05*total;
            let new_total:f32= total-discount;
            println!("The discounted total is {}",new_total);
        }
        }
        else if menu == 'W'{
            let total:f32= q*2_500.0;
            println!("\nThe total due for your order is {}",total);
        if total>10_000.0{
            let discount:f32 = 0.05*total;
            let new_total:f32= total-discount;
            println!("The discounted total is {}\n",new_total);
        }

        }else {
            println!("Invalid order. Please go through the menu carefully and try again.\n");
        }
        

        
        }
        }
 
    }

