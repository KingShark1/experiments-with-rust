fn fibbionaci(n: i32){
	let mut a = 0;
	let mut b = 1;
	println!("0 : {}", a);
	println!("1 : {}", b);
	
	for x in 2..n
	{
		let temp = a + b;
		println!("{} : {}", x, temp);
		a = b;
		b = temp;
	}
	

}

fn main(){
	println!("Program to print fibbionaci sequence upto N");
	println!("Enter N : ");
	let n = 10;
	fibbionaci(n);
}