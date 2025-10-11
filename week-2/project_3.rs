fn main(){
	let cp:f64= 510_000.0; //where cp is cost price
	let r:f64=5.0;
	let t:f64= 3.0;

	//Using the formula of Compound Interest to calculate depreciation
	let np= cp*(1.0 - (r/100.0)).powf(t); //where np is new price

	println!("The value of the TV after 3 years would be {}",np );
}