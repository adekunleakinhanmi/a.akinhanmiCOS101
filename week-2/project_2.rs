fn main(){
	let tq:f64=10.0; //where tq is total quantity.
	let t:f64 = 450_000.0;
	let m:f64 = 1_500_000.0;
	let h:f64 = 750_000.0;
	let d:f64 = 2_850_000.0;
	let a:f64 = 250_000.0;


	let tf:f64= 2.0;
	let mf:f64= 1.0;
	let hf:f64= 3.0;
	let df:f64= 3.0;
	let af:f64= 1.0;

	let sum = (t*tf)+(m*mf)+(h*hf)+(d*df)+(a*af);

	println!("The Sum of the Sales record is {}", sum);
	let average= sum/tq;
	println!("The average is {}", average);

}