use std::env;

fn main() {
	let sequence = env::args().nth(1).unwrap();
	let mut vals = sequence.chars().map(|c| c.to_digit(10).unwrap());
	let first = vals.next().unwrap();

	let (almost_total, last) = vals.fold((0, first), |(acc, last), n| {
		let next_acc = if last == n { acc + n } else { acc };
		(next_acc, n)
	});

	let ans = if last == first { almost_total + first } else { almost_total };
	println!("{}", ans);
}