fn main() {
    //Name vector
    let name = vec!["Mary","Sam","Sally","Greg","Ade","Mark","June","Ife"];

    //Age vector
    let age = vec![16,17,20,22,9,23,23,12];

    println!("Age allocation:");

    //loop to iterate elements in vector
    for i in 0..age.len()
    {
        //iterating through 1 on the vector
        println!("{} is {} years old",name[i],age[i]);
    }
}
