use std::io::{stdin, stdout, Write};

fn main() {
	println!("\nThis tool calculates the n-th fibonacci number.\n");

	loop {
		println!("Please enter the target number (n).");

		// read input
		let target = read_number();

		// calculate result
		let mut list = vec![-1; target];
		let _result = calculate_value(target - 1, &mut list);
		println!("Result is {:?}", list);

		// println!("\nDo you want to get another number? (Y/N)");
		// if read_input().to_lowercase().contains("n") {
		//     break;
		// }
	}
}

fn read_input() -> String {
	print!("> ");
	stdout().flush().expect("Could not flush.");
	let mut input = String::new();
	stdin().read_line(&mut input).expect("Failed to read line.");
	input
}

fn read_number() -> usize {
	loop {
		let input = match read_input().trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("You have to use a number.");
				continue;
			}
		};
		return input;
	}
}

// returns the fibonacci value for the n-th number using recursion.
// usize type represent index, i32 represent values
fn calculate_value(n: usize, record: &mut Vec<i128>) -> i128 {
	// non-base case
	if n > 1 {
		if record[n] != -1 {
			// negatives indicate unset space
			return record[n]; // in case it already has been found
		}
		record[n] = calculate_value(n - 2, record) + calculate_value(n - 1, record);
		return record[n];
	}

	// base cases (0 or 1)
	record[n] = n as i128;
	record[n]
}
