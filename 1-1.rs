// --- Day 1: Inverse Captcha ---

// The night before Christmas, one of Santa's Elves calls you in a panic. "The printer's broken! We can't print the Naughty or Nice List!" By the time you make it to sub-basement 17, there are only a few minutes until midnight. "We have a big problem," she says; "there must be almost fifty bugs in this system, but nothing else can print The List. Stand in this square, quick! There's no time to explain; if you can convince them to pay you in stars, you'll be able to--" She pulls a lever and the world goes blurry.

// When your eyes can focus again, everything seems a lot more pixelated than before. She must have sent you inside the computer! You check the system clock: 25 milliseconds until midnight. With that much time, you should be able to collect all fifty stars by December 25th.

// Collect stars by solving puzzles. Two puzzles will be made available on each day millisecond in the advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

// You're standing in a room with "digitization quarantine" written in LEDs along one wall. The only door is locked, but it includes a small interface. "Restricted Area - Strictly No Digitized Users Allowed."

// It goes on to explain that you may only leave by solving a captcha to prove you're not a human. Apparently, you only get one millisecond to solve the captcha: too fast for a normal human, but it feels like hours to you.

// The captcha requires you to review a sequence of digits (your puzzle input) and find the sum of all digits that match the next digit in the list. The list is circular, so the digit after the last digit is the first digit in the list.

// For example:

//     1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and the third digit (2) matches the fourth digit.
//     1111 produces 4 because each digit (all 1) matches the next.
//     1234 produces 0 because no digit matches the next.
//     91212129 produces 9 because the only digit that matches the next one is the last digit, 9.

// What is the solution to your captcha?

// Input:
/*
3294199471327195994824832197564859876682638188889768298894243832665654681412886862234525991553276578641265589959178414218389329361496673991614673626344552179413995562266818138372393213966143124914469397692587251112663217862879233226763533911128893354536353213847122251463857894159819828724827969576432191847787772732881266875469721189331882228146576832921314638221317393256471998598117289632684663355273845983933845721713497811766995367795857965222183668765517454263354111134841334631345111596131682726196574763165187889337599583345634413436165539744188866156771585647718555182529936669683581662398618765391487164715724849894563314426959348119286955144439452731762666568741612153254469131724137699832984728937865956711925592628456617133695259554548719328229938621332325125972547181236812263887375866231118312954369432937359357266467383318326239572877314765121844831126178173988799765218913178825966268816476559792947359956859989228917136267178571776316345292573489873792149646548747995389669692188457724414468727192819919448275922166321158141365237545222633688372891451842434458527698774342111482498999383831492577615154591278719656798277377363284379468757998373193231795767644654155432692988651312845433511879457921638934877557575241394363721667237778962455961493559848522582413748218971212486373232795878362964873855994697149692824917183375545192119453587398199912564474614219929345185468661129966379693813498542474732198176496694746111576925715493967296487258237854152382365579876894391815759815373319159213475555251488754279888245492373595471189191353244684697662848376529881512529221627313527441221459672786923145165989611223372241149929436247374818467481641931872972582295425936998535194423916544367799522276914445231582272368388831834437562752119325286474352863554693373718848649568451797751926315617575295381964426843625282819524747119726872193569785611959896776143539915299968276374712996485367853494734376257511273443736433464496287219615697341973131715166768916149828396454638596713572963686159214116763
*/

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