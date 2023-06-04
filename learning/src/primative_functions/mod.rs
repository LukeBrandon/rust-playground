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

/// A module of functions for i32's
pub mod ints {
	/// This will add 6 to an int
	pub fn mutate_int(number: &mut i32) {
		*number += 6;
	}

	/// This will print an int
	pub fn print_int(number: &i32) {
		println!("integer is : {}", number)
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
