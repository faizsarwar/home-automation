#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_cors;
use rocket::http::Method; 
use rocket_cors::{
    AllowedHeaders, AllowedOrigins,
    Cors, CorsOptions 
};
use rocket::http::RawStr;
use std::fs::File;
use std::error;
use std::io::prelude::*;
use std::collections::HashMap;
use rocket_contrib::templates::{Template, handlebars};
use rppal::gpio::Gpio;
use serde::Serialize;
// use bson::doc;
// use mongodb::{options::ClientOptions, Client};
use serde_json::{json, Value};
use bson::Bson::Document;
use rocket_contrib::json::Json;
mod mongo;
use mongo::{all,insert,update_collection,delete_collection,Todo,get_data};
use mongodb::{bson::{doc,oid::ObjectId},error::Error};
use std::thread;



fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[     
        "http://localhost:8080",
        "http://0.0.0.0:8080",
        "http://rust-react-todo-app.surge.sh/"
    ]);

    CorsOptions { 
        allowed_origins,
        allowed_methods: vec![Method::Get,Method::Post,Method::Delete,Method::Put].into_iter().map(From::from).collect(), 
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
            "Access-Control-Allow-Headers", 
            "Access-Control-*",
            "Origin", 
            "X-Requested-With", 
            "Content-Type", 
            "Accept"
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}


fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}


fn TurnOn1(SwitchNumber:String)-> Result<(), Box<dyn error::Error>>{
    
    // let mut pin = Gpio::new()?.get(4)?.into_output();
    
    let mut state="ON";
    // let mut pin = Gpio::new()?.get(17)?.into_output();
    // pin.set_low();
    while state=="ON"{
        println!("{} on hua aaww  {}",state,SwitchNumber);
        let mut file=File::open(format!("state{}.txt",SwitchNumber)).expect("error opnening state file");    //opening file for that switch
        let mut contents=String::new();
        file.read_to_string(&mut contents).expect("error reading state file");    
        state=string_to_static_str(contents);
    }

        // pin.set_reset_on_drop(true);
        println!("exiting now 1");
        Ok(())
}

fn TurnOn2(SwitchNumber:String)-> Result<(), Box<dyn error::Error>>{
    let mut state="ON";

    // let mut pin = Gpio::new()?.get(17)?.into_output();
    // pin.set_low();
        while state=="ON"{
            println!("{} on hua aaww  {}",state,SwitchNumber);
            let mut file=File::open(format!("state{}.txt",SwitchNumber)).expect("error opnening state file");    //opening file for that switch
            let mut contents=String::new();
            file.read_to_string(&mut contents).expect("error reading state file");    
            state=string_to_static_str(contents);
        }
            // pin.set_reset_on_drop(true);
        println!("exiting now 2");
        Ok(())
}

fn TurnOn3(SwitchNumber:String)-> Result<(), Box<dyn error::Error>>{
    let mut state="ON";

    // let mut pin = Gpio::new()?.get(27)?.into_output();
    // pin.set_low();
        while state=="ON"{
            println!("{} on hua aaww  {}",state,SwitchNumber);
            let mut file=File::open(format!("state{}.txt",SwitchNumber)).expect("error opnening state file");    //opening file for that switch
            let mut contents=String::new();
            file.read_to_string(&mut contents).expect("error reading state file");    
            state=string_to_static_str(contents);
        }
            // pin.set_reset_on_drop(true);
        println!("exiting now 3");
        Ok(())
}

fn TurnOn4(SwitchNumber:String)-> Result<(), Box<dyn error::Error>>{
    let mut state="ON";

    // let mut pin = Gpio::new()?.get(22)?.into_output();
    // pin.set_low();
        while state=="ON"{
            println!("{} on hua aaww  {}",state,SwitchNumber);
            let mut file=File::open(format!("state{}.txt",SwitchNumber)).expect("error opnening state file");    //opening file for that switch
            let mut contents=String::new();
            file.read_to_string(&mut contents).expect("error reading state file");    
            state=string_to_static_str(contents);
        }
        // pin.set_reset_on_drop(true);
        println!("exiting now 4");
        Ok(())
}








#[get("/")]
fn index() -> Template {
    // let pin4 = Gpio::new().unwrap().get(17).unwrap().into_output();

    // FROM DATABASE
    // let switches=get_data("switch".to_string());
    // let states=get_data("state".to_string());   //stAtes on or off
    // let switch_numbers=get_data("switch_number".to_string());


    let mut file1=File::open("state1.txt").expect("w");
    let mut contents1=String::new();
    file1.read_to_string(&mut contents1).expect("w");

    let mut file2=File::open("state2.txt").expect("w");
    let mut contents2=String::new();
    file2.read_to_string(&mut contents2).expect("w");

    let mut file3=File::open("state3.txt").expect("w");
    let mut contents3=String::new();
    file3.read_to_string(&mut contents3).expect("w");

    let mut file4=File::open("state4.txt").expect("w");
    let mut contents4=String::new();
    file4.read_to_string(&mut contents4).expect("w");

    #[derive(Serialize)]
    struct Context {
        state1:(String,String,String),
        state2: (String,String,String),
        state3: (String,String,String),
        state4: (String,String,String),
      }

    let context = Context {
        //for file system uncomment this
        state1: (contents1,String::from("switch1"),String::from("1")),
        state2: (contents2,String::from("switch2"),String::from("2")),   // (ON/OF,switch,switch#)
        state3: (contents3,String::from("switch3"),String::from("3")),
        state4: (contents4,String::from("switch4"),String::from("4")),
        //for database system uncomment this
        // state1: (states[0].clone(),switches[0].clone(),switch_numbers[0].clone()),
        // state2: (states[1].clone(),switches[1].clone(),switch_numbers[1].clone()),
        // state3: (states[2].clone(),switches[2].clone(),switch_numbers[2].clone()),
        // state4: (states[3].clone(),switches[3].clone(),switch_numbers[3].clone()),
    };
      
      Template::render("home", context)     // khali dictionary ya struct ayga bs
    }

#[get("/<name>/<SwitchNumber>")]
fn switch1(name: &RawStr,SwitchNumber:&RawStr)->Template{

    let mut state;    
    if name.as_str()=="ON"{
    state="ON"; 
    let mut file=File::create(format!("state{}.txt",SwitchNumber)).expect("ee");   //opening ile
    file.write_all(b"ON").expect("error writting");                                //writting in file

  
    if SwitchNumber=="1"{
    thread::spawn( move || {
        TurnOn1(String::from("1")).unwrap()
    });
    }
    else if SwitchNumber=="2"{
        thread::spawn( move || {
        TurnOn2(String::from("2")).unwrap()
    });
    }
    else if SwitchNumber=="3"{
        thread::spawn( move || {
        TurnOn3(String::from("3")).unwrap()
    });
    }
    else {
        thread::spawn( move || {
        TurnOn4(String::from("4")).unwrap()
    });
    }  

    let x11=[("name", "Faiz"),("last_name","Faiz"),("state",&state),("SwitchNumber",&SwitchNumber)];    
    let context: HashMap<&str, &str> = x11
    .iter().cloned().collect();
    return Template::render("switch1", &context);   

}      
    else if name.as_str()=="OFF"{

        println!("{} OFF hua",SwitchNumber);
        let mut file=File::create(format!("state{}.txt",SwitchNumber)).expect("ee");  //opening file
        file.write_all(b"OFF").expect("error writting");     //writing in file

        state="OFF";
        let x11=[("name", "Faiz"),("last_name","Faiz"),("state",&state)];
        let context: HashMap<&str, &str> = x11
        .iter().cloned().collect();
        return Template::render("switch1", &context);   

    }
    else{
        let x=[("name", "Faiz"),("last_name","Faiz"),("state1","OFF")];
        let context: HashMap<&str, &str> = x
        .iter().cloned().collect();    
        return Template::render("switch1", &context);   

    }

}


fn main() {
     rocket::ignite().mount("/", routes![index,switch1]).attach(make_cors()).attach(Template::fairing()).launch();
}
