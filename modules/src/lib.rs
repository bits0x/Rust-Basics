/* 
    Note: By default a child module and everything inside it is private from the 
    perspective of the parent module
*/

mod front_of_house;
use self::front_of_house::hosting;

fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // using use to create the scope 
    hosting::add_to_waitlist();

    //---------------------------------

    let mut meal = front_of_house::Breakfast::summer("wheat");

    meal.toast = String::from("rice");

    // can not use the below method to make struct instance as some feild are private
    // let mut meal = front_of_house::Breakfast {
    //     toast: String::from("rice"),
    //     seasonal_fruit: String::from("peaches")
    // };

}

//--------------------------------------

fn serve_order() {

}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order(); // fn inside the module can called directly
        super::serve_order(); // super allows us to reference the parent module 
    }

    fn cook_order() {

    }
}