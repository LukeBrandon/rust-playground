mod primative_functions;

mod restaurant;


fn perform_primative_actions() {
	let mut number: i32 = 12;
	primative_functions::ints::mutate_int(&mut number);
	primative_functions::ints::print_int(&number);

	let mut flynn: String = String::from("Flynn is a ");
	primative_functions::strings::mutate_string(&mut flynn);
	primative_functions::strings::print_string(&flynn);

	let mut true_or_false: bool = true;
	primative_functions::booleans::mutate_boolean(&mut true_or_false);
	primative_functions::booleans::print_boolean(&true_or_false);
}

fn perform_restaurant_actions() {
	let mut table_state: restaurant::TableState = restaurant::TableState::new();

	let seat_customer = restaurant::SeatCustomer{
		table: 1,
		customer_name: "Ricky".to_string()
	};

	let order = restaurant::Order { 
		meal_name: "Chicken alfredo".to_string(), 
		table: 1
	};

	let food_delivery = restaurant::Order { 
		meal_name: "Chicken alfredo".to_string(), 
		table: 1
	};

	table_state.perform_action(&restaurant::RestaurantAction::SeatCustomer(seat_customer));
	table_state.perform_action(&restaurant::RestaurantAction::TakeOrder( order ));
	table_state.perform_action(&restaurant::RestaurantAction::DeliverFood(food_delivery));
	table_state.perform_action(&restaurant::RestaurantAction::CleanTable(1));
}

fn main() {
	println!("----- Performing primative actions -----");
	perform_primative_actions();

	println!("\n----- Performing Restaurant actions -----");
	perform_restaurant_actions();

    println!("\nDone.");
}
