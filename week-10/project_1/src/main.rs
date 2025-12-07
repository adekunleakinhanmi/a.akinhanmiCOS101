struct Laptop{
    name: String,
    cost: u32,
}
impl Laptop{
    fn cost_for_quantity(&self,qty:u32)->u32{
        self.cost*qty
    }
}

fn main() {
    let lap1= Laptop{
        name:String::from("HP"),
        cost:650_000,
    };
    let lap2= Laptop{
        name:String::from("IBM"),
        cost:755_000,
    };
    let lap3= Laptop{
        name:String::from("Toshiba"),
        cost:550_000,
    };
    let lap4= Laptop{
        name:String::from("Dell"),
        cost:850_000,
    };
    let qty=3;

    let total_cost = lap1.cost_for_quantity(qty)+lap2.cost_for_quantity(qty)+lap3.cost_for_quantity(qty)+lap4.cost_for_quantity(qty);

    println!("Total cost for purchasing 3 from each brand = {}",total_cost );           


}
