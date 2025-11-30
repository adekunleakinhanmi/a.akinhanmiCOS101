use std::fs::File;
use::std::io::{Write,Result};

#[derive(Debug)]
struct Student{
    name:String, matric_no:String, department: String, level:u32,
}
fn display_students(students:&[Student]){
    println!("{:<20} {:<15} {:<15} {:<5}","Student name", "Matric Number","Department","Level" );

    for s in students{
        println!("{:<20} {:<15} {:<15} {:<5}", s.name, s.matric_no, s.department,s.level);
    }
}
fn save_to_file(students:&[Student])->Result<()>{
    let mut file= File::create("students.csv")?;
    writeln!(file,"PAU SMIS")?;
    writeln!(file,"Stuent Name, Matric Number, Department, Level")?;

    for s in students{
        writeln!(file,"{},{},{},{}", s.name, s.matric_no, s.department, s.level)?;
    }
    Ok(())
}
fn main()->Result<()> {
    let students= vec![
    Student{
        name:"Oluchi Mordi".into(),
        matric_no:"ECO10110101".into(),
        department:"Economics".into(),
        level:200,
    },
    Student{
        name:"Adekunle Gold".into(),
        matric_no:"EEE11020202".into(),
        department:"Electrical".into(),
        level:200,
    },
    Student{
        name:"Adams Aliyu".into(),
        matric_no:"ECO10110101".into(),
        department:"Economics".into(),
        level:100,
    },
    Student{
        name:"Shania Bolade".into(),
        matric_no:"CSC10328828".into(),
        department:"Computer".into(),
        level:200,
    },
    Student{
        name:"Bianca Edemoh".into(),
        matric_no:"MEE10202001".into(),
        department:"Mechanical".into(),
        level:100,
    },
    ];

    display_students(&students);
    save_to_file(&students)?;

    println!("\nStudent data saved to .csv successfully.");
    Ok(())
}
