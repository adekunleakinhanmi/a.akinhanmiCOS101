
use std::io;
use std::io::Read;

fn show_sql_file(filename:&str){
    let mut file = std::fs::File::open(filename).expect("Failed to open SQL file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read SQL file");
    println!("\n======{}=====\n{}",filename,contents );

}

fn main() {
    loop{
    println!("Please enter your role:
     A = Administrator
     P = ProjectManager
     E = Employee
     C = Customer
     V = Vendor
     Q = Quit");

    let mut role = String::new();
    io::stdin().read_line(&mut role).expect("Failed to read input");
    let role = role.trim().to_lowercase();

    match role.as_str(){
        "a"=>{
            show_sql_file("globacom_dbase.sql");
        }
        "p"=>{
            show_sql_file("project_tb.sql");
        }
        "e"=>{
            show_sql_file("staff_tb.sql");
        }
        "c"=>{
            show_sql_file("customer_tb.sql");
        }
        "v"=>{
            show_sql_file("dataplan_tb.sql");
        }
        "q"=>{
            println!("Program terminated by user.");
            break;
        }
        _=>{
            println!("Invalid role entered.");
        }
        }
    }
}
