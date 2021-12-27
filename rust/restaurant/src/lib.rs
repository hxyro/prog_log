#![allow(dead_code, unused_variables)]
mod front_of_house {
  pub mod hosting {
       pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {
            super::super::front_of_house::hosting::add_to_waitlist();
        }

        fn serve_order() {}

        fn take_payment() {}
    }
}

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
