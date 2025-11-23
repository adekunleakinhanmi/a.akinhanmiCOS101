use std::io;
use std::collections::HashMap;

//Structure representing a role and its APS numeric range
#[derive(Debug)]
struct RoleLevel{
    role: &'static str,
    aps_label:&'static str,
    min_years:u32,
    max_years:u32,
}

//Build the database
fn build_db()-> HashMap<&'static str, Vec<RoleLevel>>{
    let mut db: HashMap<&str, Vec<RoleLevel>>= HashMap::new();

    db.insert("Office Adminstrator",vec![
        RoleLevel{role:"Intern", aps_label:"APS 1-2", min_years:0, max_years:2},
        RoleLevel{role:"Adminstrator", aps_label:"APS 3-5", min_years:3, max_years:5},
        RoleLevel{role:"Senior Adminstrator", aps_label:"APS 5-8", min_years:5, max_years:8},
        RoleLevel{role:"Office Manager", aps_label:"EL1 8-10", min_years:8, max_years:10},
        RoleLevel{role:"Director", aps_label:"EL2 10-13", min_years:10, max_years:13},
        RoleLevel{role:"CEO", aps_label:"SES", min_years:13, max_years:50},
        ]);

    db.insert("Academic",vec![
        RoleLevel{role:"Research Assistant", aps_label:"APS 3-5", min_years:3, max_years:5},
        RoleLevel{role:"PhD Candidate", aps_label:"APS 5-8", min_years:5, max_years:8},
        RoleLevel{role:"Post-Doc Researcher", aps_label:"EL1 8-10", min_years:8, max_years:10},
        RoleLevel{role:"Senior Lecturer", aps_label:"EL2 10-13", min_years:10, max_years:13},
        RoleLevel{role:"Dean", aps_label:"SES", min_years:13, max_years:50},
        ]);

    db.insert("Lawyer",vec![
        RoleLevel{role:"Paralegal", aps_label:"APS 1-2", min_years:0, max_years:2},
        RoleLevel{role:"Junior Associate", aps_label:"APS 3-5", min_years:3, max_years:5},
        RoleLevel{role:"Associate", aps_label:"APS 5-8", min_years:5, max_years:8},
        RoleLevel{role:"Senior Associate 1-2", aps_label:"EL1 8-10", min_years:8, max_years:10},
        RoleLevel{role:"Senior Associate 3-4", aps_label:"EL2 10-13", min_years:10, max_years:13},
        RoleLevel{role:"Partner", aps_label:"SES", min_years:13, max_years:50},
        ]);

    db.insert("Teacher",vec![
        RoleLevel{role:"Placement", aps_label:"APS 1-2", min_years:0, max_years:2},
        RoleLevel{role:"Classroom Teacher", aps_label:"APS 3-5", min_years:3, max_years:5},
        RoleLevel{role:"Snr Teacher", aps_label:"APS 5-8", min_years:5, max_years:8},
        RoleLevel{role:"Leading Teacher", aps_label:"EL1 8-10", min_years:8, max_years:10},
        RoleLevel{role:"Deputy Principal", aps_label:"EL2 10-13", min_years:10, max_years:13},
        RoleLevel{role:"Principal", aps_label:"SES", min_years:13, max_years:50},
        ]);
    db
}

//match by role+experience range
fn find_aps_level(profession:&str, role:&str,years:u32)->Option<&'static str>{
    let db = build_db();
    if let Some(roles)=db.get(profession){
        for r in roles{
            if r.role.eq_ignore_ascii_case(role){
                if years>= r.min_years && years <=r.max_years{
                    return Some(r.aps_label);
                }else {
                    return Some("Experience does not match APS range for this role");
                }
            }
        }
    }
    None
}

fn main() {
    println!("=== APS Level Checker ===");

    //Input profession
    println!("Enter profession(Office Adminstrator, Academic, Lawyer, Teacher): ");
    let mut profession= String::new();
    io::stdin().read_line(&mut profession).expect("Failed to read input");
    let profession = profession.trim();

    //Input Role
    println!("Enter Role:");
    let mut role= String::new();
    io::stdin().read_line(&mut role).expect("Failed to read input");
    let role= role.trim();

    //Input years of experience
    println!("Enter Years of Experience:");
    let mut years_input= String::new();
    io::stdin().read_line(&mut years_input).expect("Failed to read input");
    let years:u32= years_input.trim().parse().unwrap();

    println!("\n--- Result ---");
    match find_aps_level(profession,role, years){
        Some(level)=>{
            println!("Profession:{}",profession);
            println!("Role:{}", role);
            println!("Years of Experience:{}",years);
            println!("APS level:{}",level);
        }
        None=>{
            println!("No matching APS level found for the provided details.");
        }
    }



}


