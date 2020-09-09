#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::State;

use rsbacktester::*;

#[get("/")]
fn hello(state: State<Engine>) -> String {
    format!("{}", state.prices.ticks.len())
}

fn main() {
    let config = init_engine(&"ticks.csv", 10000);
    
    rocket::ignite()
    .mount("/", routes![hello])
    .manage(config)
    .launch();
}