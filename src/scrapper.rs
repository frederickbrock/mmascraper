




pub mod webcrawler {

// use std::io::{self, Write};
use std::str;
use bytes::{Bytes};
use hyper::{Client};

// use hyper::client::{self,ResponseFuture};
// use hyper::rt::{self, Future, Stream};
use hyper::rt::{self, Future, Stream};
use hyper::body::{self, Body};

use hyper_tls::HttpsConnector;
use hyper::http::{Result, Response};

use select::document::{Document};
use select::node::{Node};
use select::predicate::{Attr, Class, Name, Predicate};

use mongodb::{Bson, bson, doc};
use mongodb::*;
use mongodb::db::ThreadedDatabase;

fn capture_fighter_basic_info(body: &mut Bytes) -> Vec<Fighter> {
    let doc = Document::from(str::from_utf8(body).unwrap());
    let mut fighters = Vec::new();

    fn split_win_loss(winloss: &str) -> (i32, i32, i32) {
            
            let winloss_str: Vec<&str> = winloss.split_whitespace().collect();
            let components: Vec<&str> = winloss_str[0].split("-").collect();
            let record: Vec<i32> = components.iter()
                                             .map(|v| i32::from_str_radix(v,10).unwrap())
                                             .collect();
            (record[0], record[1], record[2])
    }

    fn extract_first_descendant<'a>(tag: &'a str, node: &'a Node) -> String {
        match node.find(Attr("class", tag).descendant(Class("field__item"))).next() {
            Some(val) => {
                val.text()
            }
            None => {
                "".to_string()
            }
        }
    }

    fn extract_fighter_record<'a>(node: &'a Node) -> (i32,i32,i32) {
         let mut fighter_record: (i32,i32,i32) = (0,0,0);
         let rec_node = node.find(Attr("class","c-listing-athlete__record")).next();
         let rec_node = rec_node.unwrap().first_child();
         split_win_loss(&rec_node.unwrap().text().trim())
    }


    for node in doc.find(Attr("class","c-listing-athlete-flipcard__inner")) {

        let mut fighter_name = node.find(Attr("class", "c-listing-athlete__name"));
        let mut fighter_name = fighter_name.next().unwrap().text().trim().to_string();

        let fighter_nickname    = extract_first_descendant("c-listing-athlete__nickname", &node);
        let figher_nickname     = &fighter_nickname[1..fighter_nickname.len() - 2];
        let fighter_weightclass = extract_first_descendant("c-listing-athlete__title", &node);
        let fighter_record = extract_fighter_record(&node);
        
        println!("fighter ( {}, {}, {}, {}-{}-{} )",
                            fighter_name,
                            fighter_nickname,
                            fighter_weightclass, 
                            fighter_record.0, 
                            fighter_record.1,
                            fighter_record.2);
        
        fighters.push(Fighter{
                            name: fighter_name,
                            link: String::from("http://www.action.com"), 
                            nickname: fighter_nickname.to_string(),
                            weightclass: fighter_weightclass,
                            win: fighter_record.0,
                            loss: fighter_record.1,
                            draw: fighter_record.2,
                          });
    }

    fighters
}

fn capture_fighter_stats(body: &mut Bytes) -> Vec<FighterStat> {
    let fighter_stats = Vec::new();
    fighter_stats
}

pub fn crawl_for_fighters() -> impl Future<Item=Vec<Fighter>, Error=WebScrapeError>{

        let ufc_root: String = String::from("https://www.ufc.com/athletes/all?filters%5B0%5D=status%3A23");
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_,hyper::Body>(https);
    
        client.get(ufc_root.parse::<hyper::Uri>().unwrap())
            .and_then(|res| {
                println!("http status code: {}", res.status());
                res.into_body().concat2()
            })
            .from_err::<WebScrapeError>() //not sure why I need to call this
            .and_then(|body| { //this is the second step
                
                let mut html_body = Bytes::from(body);
                let fighters: Vec<Fighter> = capture_fighter_basic_info(&mut html_body);
                
                Ok(fighters)
            }).from_err()
}


pub fn crawl_for_fighter_stats(fighter_name: &str ) -> impl Future<Item=Vec<FighterStat>, Error=WebScrapeError> {
        
        let sherdog_root: String = String::from("https://www.ufc.com/athletes/all?filters%5B0%5D=status%3A23");
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_,hyper::Body>(https);
    
        client.get(sherdog_root.parse::<hyper::Uri>().unwrap())
            .and_then(|res| {
                println!("http status code: {}", res.status());
                res.into_body().concat2()
            })
            .from_err::<WebScrapeError>() //not sure why I need to call this
            .and_then(|body| { //this is the second step
                
                let mut html_body = Bytes::from(body);
                let fighters: Vec<FighterStat> = capture_fighter_stats(&mut html_body);
                
                Ok(fighters)
            }).from_err()
}

pub fn persist_fighter_name(sub: &Fighter) -> bool {
    let client = 
        mongodb::Client::connect("localhost",27017)
                        .expect("Failed to connect");

    let col = client.db("local").collection("fighter");
    

    let doc = doc!{ "name" : &sub.name,
                    "weightclass": &sub.weightclass,
                    "nickname": &sub.nickname,
                    "win":  &sub.win.to_string(),
                    "loss": &sub.loss.to_string(),
                    "draw": &sub.draw.to_string(),
                  };
    
    let result = col.insert_one(doc.clone(),None).ok().expect("Failed to insert document");

    result.acknowledged && result.write_exception == None
}

#[derive(Deserialize, Debug)]
pub struct Fighter {
     pub name: String,
     pub link: String,
     pub weightclass: String,
     pub nickname: String,
     pub win: i32,
     pub loss: i32,
     pub draw: i32,
}

#[derive(Deserialize, Debug)]
pub struct FighterStat {
    pub name: String,
    pub value: StatValue,
}

#[derive(Deserialize,Debug)]
pub enum StatValue {
    StringValue(String),
    IntegerValue(i32),
    FloatValue(i32)
}

pub enum WebScrapeError {
    Http(hyper::Error),
}

impl From<hyper::Error> for WebScrapeError {
    fn from(err: hyper::Error) -> WebScrapeError {
        WebScrapeError::Http(err)
    }
}

}


