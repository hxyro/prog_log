#![allow(dead_code, unused_variables)]
mod front_of_house;
pub use crate::front_of_house::hosting;

mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast
        {
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("avo"),
            }
        }
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn eat_at_restaurant()
{
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    
    println!("I'd like {} toast please", meal.toast);
    println!("{:#?}", meal);

    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
use crate::front_of_house::hosting::add_to_waitlist;
fn test()
{
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
