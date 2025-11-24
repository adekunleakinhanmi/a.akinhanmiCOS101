use::std::io::Write;

fn main() {
    let ln_1 = "Lager             Stout            Non-Alcoholic\n";
    let ln_2 = "33 Export         Legend           Maltina\n";
    let ln_3 = "Desperados        Turbo King       Amstel Malta\n";
    let ln_4 = "Goldberg          Williams         Malta Gold\n";
    let ln_5 = "Gulder                             Fayrouz\n";
    let ln_6 = "Heineken\n";
    let ln_7 = "Star\n";

    let mut file = std::fs::File::create("Nigerian Brewery Limited.txt").expect("create failed");
    file.write_all(ln_1.as_bytes()).expect("write failed");
    file.write_all(ln_2.as_bytes()).expect("write failed");
    file.write_all(ln_3.as_bytes()).expect("write failed");
    file.write_all(ln_4.as_bytes()).expect("write failed");
    file.write_all(ln_5.as_bytes()).expect("write failed");
    file.write_all(ln_6.as_bytes()).expect("write failed");
    file.write_all(ln_7.as_bytes()).expect("write failed");   
    println!("Nigerian Brewery file Created.");
}
