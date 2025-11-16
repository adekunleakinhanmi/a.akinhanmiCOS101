use std::io;

fn trapezium(height:f64,base1:f64,base2:f64)->f64
{
    height/2.0*(base1+base2)
}

fn rhombus(diagonal1:f64,diagonal2:f64)->f64{
    0.5*diagonal1*diagonal2
}

fn parallelogram(base:f64,altitude:f64)->f64{
    base*altitude
}
fn cube(length:f64)->f64{ //calculates the area of cube
    6.0*length*length
}
fn cylinder(radius:f64,height:f64)->f64{ //calculates the volume of cylinder
    3.147*radius*radius*height
}

//function to read values
fn read_value(prompt:&str)->f64{
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse::<f64>().unwrap()
}

fn main() {
    loop {
    println!("Select what you want to calculate:");
    println!("1. Area of Trapezium.");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let mut input1= String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let selection:u32=input1.trim().parse().unwrap();

    if selection==1
    {
        let height = read_value("Enter height:");
        let base1 = read_value("Enter base 1:");
        let base2 = read_value("Enter base 2:");

        let area = trapezium(height,base1,base2);
        println!("Area of Trapezium = {}",area);
    }else if selection == 2
    {
        let d1 = read_value("Enter diagonal 1:");
        let d2 = read_value("Enter diagonal 2:");

        let area = rhombus(d1,d2);
        println!("Area of Rhombus = {}",area );
    }else if selection == 3
    {
        let base = read_value("Enter base:");
        let alt = read_value("Enter altitude:");

        let area = parallelogram(base,alt);
        println!("Area of parallelogram = {}",area );
    }else if selection == 4
    {
        let length = read_value("Enter length of side:");

        let area = cube(length);
        println!("Area of cube = {}", area);
    }else if selection== 5
    {
        let radius = read_value("Enter the radius:");
        let height = read_value("Enter the height:");

        let volume = cylinder(radius,height);
        println!("The volume of the cylinder = {}",volume );
    }else {
        println!("Invalid choice!");
    }

    println!("Do you want to stop: Y/N\n");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let stop:char= input2.trim().to_uppercase().parse().unwrap();

    if stop== 'Y'
    {
        break;
    }

    }
    
}
