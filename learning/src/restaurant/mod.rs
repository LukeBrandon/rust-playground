#[derive(Debug)]
pub struct Order {
	pub meal_name: String,
	pub table: u8,
}

#[derive(Debug)]
pub struct SeatCustomer {
	pub table: u8,
	pub customer_name: String
}


pub enum RestaurantAction {
	SeatCustomer(SeatCustomer),
	TakeOrder(Order),
	DeliverFood(Order),
	CleanTable(u8),
}

pub struct TableState {
	pub customer_seated: bool,
	pub order_taken: bool,
	pub food_delivered: bool,
	pub table_cleaned: bool,
}

impl TableState {
	/// Constructor: Generates a new TableState where no actions have been taken
	pub fn new() -> TableState {
		let table = TableState {
			customer_seated: false,
			order_taken: false,
			food_delivered: false,
			table_cleaned: false,
		};

		return table;
	}

	pub fn perform_action(&mut self, action: &RestaurantAction) {
		match action {
			RestaurantAction::SeatCustomer(seating) => {
				self.customer_seated = true;
				println!("Customer seated: {:?}", seating);
			},
			RestaurantAction::TakeOrder(order) => {
				self.order_taken = true;
				println!("Order taken: {:?}", order);
			},
			RestaurantAction::DeliverFood(delivery) => {
				self.food_delivered = true;
				println!("Food delivered: {:?}", delivery);
			},
			RestaurantAction::CleanTable(table) => {
				self.table_cleaned = true;
				println!("Table cleaned: {:?}", table);
			}
		}
	}
}
