use std::env;

fn main() {
	let sequence = env::args().nth(1).unwrap();
	let vals: Vec<u32> = sequence.chars()
		.map(|c| c.to_digit(10).unwrap())
		.collect();
	let offset = vals.len() / 2;

	let ans = vals[..offset].iter().zip(vals[offset..].iter())
		.filter(|&(a, b)| a == b)
		.fold(0, |acc, (a, b)| acc + a + b);

	println!("{}", ans);
}