use budgetteer::budget_optimize;

use std::io::{self, Error};

pub mod budgetteer;


fn main() {

    let keyb_name = String::from("keyboards");
    let usb_name = String::from("usb drivers");

    let keyb_prices  = require( &keyb_name);
    let drivers = require( &usb_name );

    if let Ok(budget) = budget_optimize(72, keyb_prices, drivers){
        println!("{}", budget );
    } else  {
        println!("-1");
    }
}

fn require( item: &str ) -> Vec<u64> {
    println!("Insert space separate cost of {}: \n", item);
    let mut item_list = String::new();
    if io::stdin().read_line(&mut item_list).unwrap() < 1 {
        panic!("No way this is a {} price list", item);
    }

    str_to_vec(&item_list)
}


fn str_to_vec( origin : &str ) -> Vec<u64> {

    println!("Trying to split {:?} ", origin);

    origin.trim()
        .split(' ')
        .map(|x| x.replace(" ", "")
            .parse::<u64>().unwrap())
        .collect()

}