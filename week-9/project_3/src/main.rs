fn main() {
    //Dataset1: Names of Commisioners
    let commissioners= vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbonna", 
    "Adewale Jimoh Akanbi", "Osazuwa Faith Etiyeye"];
    //Dataset2: Ministries
    let ministries = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];

    //Dataset 3 : Geopolitical zones
    let zones = vec!["NorthWest", "NorthEast", "South East", "South West", "South South"];

    //ensure all datasets have the same length
    if commissioners.len()!=ministries.len()||ministries.len()!=zones.len(){
        println!("Error: Datasets are not the same length!");
        return;
    }

    //Merge and display results
    println!("Merged EFCC Dataset:\n");

    for i in 0..commissioners.len(){
        println!("Record:{}",i+1 );
        println!("Name:{}",commissioners[i] );
        println!("Ministry:{}",ministries[i] );
        println!("Zone:{}", zones [i]);
        println!("---------------------");
    }
}
