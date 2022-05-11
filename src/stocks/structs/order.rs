use chrono::{Local, DateTime};
use super::stock::Stock;
use super::super::enums::order_types::OrderType;


pub struct Order {
    pub date: DateTime<Local>,
    pub stock: Stock,
    pub number: i32,
    pub order_type: OrderType
}

impl Order {

    pub fn new(stock: Stock, number: i32, order_type: OrderType) -> Order {

        let today: DateTime<Local> = Local::now();
        return Order{date: today, stock, number, order_type}

    }

    pub fn current_value(&self) -> f32 {
        return self.stock.current_price * self.number as f32
    }

    pub fn current_profit(&self) -> f32 {
        let current_price: f32 = self.current_value();
        let initial_price: f32 = self.stock.open_price * self.number as f32;

        match self.order_type {
            OrderType::Long => return current_price - initial_price,
            OrderType::Short => return initial_price - current_price
        }
    }

    pub fn close_order(order: Order) -> f32 {
        println!("Order for {} is being closed", &order.stock.name);
        return order.current_profit()
    }

    pub fn open_order(
        number: i32, 
        order_type: OrderType, 
        stock_name: &str, 
        open_price: f32, 
        stop_loss: Option<f32>, 
        take_profit: Option<f32>
    ) -> Order {

        println!("Order for {} is beign made", &stock_name);
        let mut stock: Stock = Stock::new(stock_name, open_price);

        match stop_loss {
            Some(value) => stock = stock.with_stop_loss(value),
            None => ()
        }
        match take_profit {
            Some(value) => stock = stock.with_take_profit(value),
            None => ()
        }
    
        return Order::new(stock, number, order_type)
    }
    
}