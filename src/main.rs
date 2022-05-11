mod stocks;
use serde_json::{json, Value};

use stocks::structs::stock::Stock;
use stocks::structs::order::Order;
use stocks::enums::order_types::OrderType;

use std::env;
use rand::prelude::*;
use std::str::FromStr;


fn main() {

    let args: Vec<String> = env::args().collect();
    let action: &String = &args[1];
    let name: &String = &args[2];
    let amount: i32 = i32::from_str(&args[3]).unwrap();
    let price: f32 = f32::from_str(&args[4]).unwrap();

    let mut new_order: Order = Order::open_order(amount, OrderType::Long, &name.as_str(), price, None, None);
    print_current_price_and_value(&new_order);

    match action.as_str() {
        "buy" => {
            println!("The value of your investment is: {}", new_order.current_value());
        }
        "sell" => {
            let mut rng = rand::thread_rng();

            let new_price_ref: f32 = rng.gen();
            let new_price: f32 = new_price_ref * 100 as f32;

            new_order.stock.update_price(new_price);

            let sale_profit: f32 = Order::close_order(new_order);
            println!("Here is the profit you made: {}", sale_profit);
        }

        _ => {
            panic!("Only 'buy' and 'sell' actions are supported");
        }
    }
}

fn print_current_price_and_value(new_order: &Order) {
    println!("The current price is: {}", new_order.current_value());
    println!("The current profit is: {}", new_order.current_profit());
}