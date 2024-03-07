// This is where we are going to create our module
mod pizza_order{
    // pub means public
    pub struct Pizza{
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }
    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza { 
                dough: String::from("regular dough"),
                cheese: String::from("mozzerella"),
                topping: String::from(topping),
            }
        }
    }
    // next modules
    pub mod help_customer{
        // although the help_customer is public, this function is not public
        fn seat_at_table(){
            println!("Customer seated at table");
        }
        pub fn take_order(){
            seat_at_table();
            let cust_pizza: super::Pizza = super::Pizza::lunch("veggies");
            serve_customer(cust_pizza);
        }
        fn serve_customer(cust_pizza: super::Pizza){
            println!("Customer is server a regular pizza with {}", cust_pizza.topping);
        }
    }
}

pub fn order_food(){
    crate::restaurant::pizza_order::help_customer::take_order();
}