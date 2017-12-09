// --- Day 4: High-Entropy Passphrases ---

// A new system policy has been put in place that requires all accounts to use a passphrase instead of simply a password. A passphrase consists of a series of words (lowercase letters) separated by spaces.

// To ensure security, a valid passphrase must contain no duplicate words.

// For example:

//     aa bb cc dd ee is valid.
//     aa bb cc dd aa is not valid - the word aa appears more than once.
//     aa bb cc dd aaa is valid - aa and aaa count as different words.

// The system's full passphrase list is available as your puzzle input. How many passphrases are valid?
use std::collections::HashSet;

fn solve(input: &str, transform: fn(word: &str) -> String) -> String {
	let rows = input.split("\n").filter(|&s| !s.is_empty());
	let valid_rows = rows.map(|row| {
		let mut encountered = HashSet::new();
		let words = row.trim().split_whitespace();

		for word in words {
			let transformed_word = transform(word);
			if encountered.contains(transformed_word.as_str()) {
				return false;
			}

			encountered.insert(transformed_word);
		}

		return true;
	});

	let ans = valid_rows.fold(0, |count, valid| {
		if valid { count + 1 } else { count }
	});

	ans.to_string()
}

pub fn part1(input: &str) -> String {
	solve(input, |word| word.to_string())
}

// --- Part Two ---

// For added security, yet another system policy has been put in place. Now, a valid passphrase must contain no two words that are anagrams of each other - that is, a passphrase is invalid if any word's letters can be rearranged to form any other word in the passphrase.

// For example:

//     abcde fghij is a valid passphrase.
//     abcde xyz ecdab is not valid - the letters from the third word can be rearranged to form the first word.
//     a ab abc abd abf abj is a valid passphrase, because all letters need to be used when forming another word.
//     iiii oiii ooii oooi oooo is valid.
//     oiii ioii iioi iiio is not valid - any of these words can be rearranged to form any other word.

// Under this new system policy, how many passphrases are valid?
pub fn part2(input: &str) -> String {
	solve(input, |word| {
		let mut chars: Vec<char> = word.chars().collect();
		chars.sort_unstable();
		chars.into_iter().collect()
	})
}