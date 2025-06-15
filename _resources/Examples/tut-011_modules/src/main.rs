// Declare that we want to use the restaurant module here
mod restaurant;

// Declare a specific function we'll use to access the
// pizza_order module
use crate::restaurant::order_food;

fn main() {
    order_food();
}
