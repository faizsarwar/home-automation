#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::RawStr;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rppal::gpio::Gpio;
use std::thread;
use serde::Serialize;
use rppal::system::DeviceInfo;

#[macro_use] extern crate rocket;



fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}


fn TurnOn1(SwitchNumber:String)-> Result<(), Box<dyn Error>>{
    let mut state="ON";
    let mut pin = Gpio::new()?.get(4)?.into_output();
    pin.set_low();

    while state=="ON"{
        println!("{} on hua aaww  {}",state,SwitchNumber);
        let mut file=File::open(format!("state{}.txt",SwitchNumber)).expect("error opnening state file");    //opening file for that switch
        let mut contents=String::new();
        file.read_to_string(&mut contents).expect("error reading state file");    
        state=string_to_static_str(contents);
    }
     pin.set_high();	
     pin.set_reset_on_drop(true);
        println!("exiting now 1");
        Ok(())
}

fn TurnOn2(SwitchNumber:String)-> Result<(), Box<dyn Error>>{
    let mut state="ON";

    let mut pin = Gpio::new()?.get(17)?.into_output();
    pin.set_low();
        while state=="ON"{
            println!("{} on hua aaww  {}",state,SwitchNumber);
            let mut file=File::open(format!("state{}.txt",SwitchNumber)).expect("error opnening state file");    //opening file for that switch
            let mut contents=String::new();
            file.read_to_string(&mut contents).expect("error reading state file");    
            state=string_to_static_str(contents);
        }
	pin.set_high();
        pin.set_reset_on_drop(true);
        println!("exiting now 2");
        Ok(())
}

fn TurnOn3(SwitchNumber:String)-> Result<(), Box<dyn Error>>{
    let mut state="ON";

    let mut pin = Gpio::new()?.get(27)?.into_output();
    pin.set_low();
        while state=="ON"{
            println!("{} on hua aaww  {}",state,SwitchNumber);
            let mut file=File::open(format!("state{}.txt",SwitchNumber)).expect("error opnening state file");    //opening file for that switch
            let mut contents=String::new();
            file.read_to_string(&mut contents).expect("error reading state file");    
            state=string_to_static_str(contents);
        }
        pin.set_high();
        pin.set_reset_on_drop(true);
        println!("exiting now 3");
        Ok(())
}

fn TurnOn4(SwitchNumber:String)-> Result<(), Box<dyn Error>>{
    let mut state="ON";

    let mut pin = Gpio::new()?.get(22)?.into_output();
    pin.set_low();
        while state=="ON"{
            println!("{} on hua aaww  {}",state,SwitchNumber);
            let mut file=File::open(format!("state{}.txt",SwitchNumber)).expect("error opnening state file");    //opening file for that switch
            let mut contents=String::new();
            file.read_to_string(&mut contents).expect("error reading state file");    
            state=string_to_static_str(contents);
        }
	pin.set_high();
        pin.set_reset_on_drop(true);
        println!("exiting now 4");
        Ok(())
}







#[get("/")]
fn index() -> Template {
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
        state1: (String,String,u8),
        state2: (String,String,u8),
        state3: (String,String,u8),
        state4: (String,String,u8),
      }

    let context = Context {
        state1: (contents1,String::from("switch1"),1),
        state2: (contents2,String::from("switch2"),2),
        state3: (contents3,String::from("switch3"),3),
        state4: (contents4,String::from("switch4"),4)
      };
      
      Template::render("home", context)     // khali dictionary ya struct ayga bs
}


#[get("/<name>/<SwitchNumber>")]
fn switch1(name: &RawStr,SwitchNumber:&RawStr)->Template{
    let mut state;    
    if name.as_str()=="ON"{
    state="ON"; 
    let mut file=File::create(format!("state{}.txt",SwitchNumber)).expect("ee");
    file.write_all(b"ON").expect("error writting");
    if SwitchNumber=="1"{
	thread::spawn(move ||
        TurnOn1(String::from("1")).unwrap()
	);
    }
    else if SwitchNumber=="2"{
	thread::spawn(move ||
        TurnOn2(String::from("2")).unwrap()
	);    
	}
    else if SwitchNumber=="3"{
	thread::spawn(move ||
        TurnOn3(String::from("3")).unwrap()
	);
    }
    else {
	thread::spawn(move ||
        TurnOn4(String::from("4")).unwrap()
	);
    }


    let x11=[("name", "Faiz"),("last_name","Faiz"),("state",&state),("SwitchNumber",&SwitchNumber)];    
    let context: HashMap<&str, &str> = x11
    .iter().cloned().collect();
    return Template::render("switch1", &context);    
}      
    else if name.as_str()=="OFF"{

        println!("{} OFF hua",SwitchNumber);
        let mut file=File::create(format!("state{}.txt",SwitchNumber)).expect("ee");
        file.write_all(b"OFF").expect("error writting");
        state="OFF";
        let x11=[("name", "Faiz"),("last_name","Faiz"),("state",&state)];
        let context: HashMap<&str, &str> = x11
        .iter().cloned().collect();
        return Template::render("switch1", &context);   
    
    }
    else if name.as_str()=="favicon.ico"{
        let mut file=File::open("state.txt").expect("w");
        let mut contents=String::new();
        file.read_to_string(&mut contents).expect("w");    
        state=string_to_static_str(contents);
        let x=[("name", "Faiz"),("last_name","Faiz"),("state",&state)];
        let context: HashMap<&str, &str> = x
        .iter().cloned().collect();
       return Template::render("switch1", &context);      
    }    
    else{
        let mut file=File::open("state1.txt").expect("w");
        let mut contents=String::new();
        file.read_to_string(&mut contents).expect("w");    
        state=string_to_static_str(contents);
        let x=[("name", "Faiz"),("last_name","Faiz"),("state1",&state)];
        let context: HashMap<&str, &str> = x
        .iter().cloned().collect();
       return Template::render("switch1", &context);    
    }
}


fn main() {
    rocket::ignite().mount("/", routes![index,switch1]).attach(Template::fairing()).launch();
}
