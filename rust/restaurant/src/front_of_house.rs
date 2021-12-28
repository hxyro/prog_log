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
