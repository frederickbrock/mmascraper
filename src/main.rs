

extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate bytes;
extern crate select;
extern crate tendril;
extern crate mongodb;

use scrapper::*;
use hyper::rt::{self, Future, Stream};

mod scrapper;

fn main() {
    
    let fut = webcrawler::crawl_for_fighters().map(|users| {
        println!("completed {:?}", users.len());
        for user in &users {
            let fighter_name = &user.name;
            println!("persisting {} to mongo fighter collection", fighter_name);
            match webcrawler::persist_fighter_name(user) {
                true  => println!("successfully persisted"),
                false => println!("something went wrong"),
                _ => println!("unknown error occurred"),
            }
        }
        
    }).map_err(|_err| {
        println!("error that I cannot print out because of missing Trait with no idea how to implement it");
    });

    rt::run(fut);
    
}


