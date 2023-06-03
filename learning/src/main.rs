

mod functions {
	pub mod strings {
		/// This will appeand a word on to a string
		pub fn mutate_string(flynn: &mut String) {
			flynn.push_str("beaut");
		}

		/// This will print a string
		pub fn print_string(flynn: &String) {
			println!("string is: {}",flynn);
		}
	}

	pub mod ints {
		/// This will add 6 to an int
		pub fn mutate_int(number: &mut i32) {
			*number += 6;
		}

		/// This will print an int
		pub fn print_int(number: &i32) {
			println!("int 32 : {}", number)
		}

	}

	pub mod booleans {
		pub fn mutate_boolean(boolean: &mut bool) {
			*boolean = false;
		}

		pub fn print_boolean(boolean: &bool) {
			println!("boolean is: {}", boolean)
		}

	}


}


fn main() {
	let mut number: i32 = 12;
	functions::ints::mutate_int(&mut number);
	functions::ints::print_int(&number);

	let mut flynn: String = String::from("Flynn is a ");
	functions::strings::mutate_string(&mut flynn);
	functions::strings::print_string(&flynn);

	let mut true_or_false: bool = true;
	functions::booleans::mutate_boolean(&mut true_or_false);
	functions::booleans::print_boolean(&true_or_false);

    println!("Done.");
}
