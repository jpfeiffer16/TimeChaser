extern crate rustc_serialize;
use std::io::prelude::*;
use std::fs::File;
use std::io::Error;
use std::env;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct HoursData  {
    ticket: String,
    description: String,
    time: String,
    status: String,
    //TODO: Needs to be implemented
    // touched: String,
}


fn get_hours() -> Result<Vec<HoursData>, Error> {
    let mut file = try!(File::open("hours.json"));
    let mut text = String::new();
    try!(file.read_to_string(&mut text));
    let data : Vec<HoursData> = json::decode(&text.to_string()).unwrap();
    Ok(data)
}

fn write_hours(hours: &Vec<HoursData>) -> Result<(), Error> {
  
  let mut f = try!(File::create("hours.json"));
  let encoded = json::encode(&hours).unwrap();
  try!(f.write_all(encoded.as_bytes()));
  Ok(())
}

fn list_hours(hours: &Vec<HoursData>) -> Result<(), Error> {
    //TODO: List hours to stdout here.
    println!("TICKET            TIME            DESC        STATUS");
    for x in hours.iter() {
        // let this_hour = hours[x];
        println!("{}         {}          {}     {}", x.ticket, x.time, x.description, x.status);
    }
    Ok(())
}

// fn start_ticket(data: &Vec<HoursData>, ticket_number: String) {
//     for hour_item in data.iter() {
//         if hour_item.ticket == ticket_number {
//             hour_item.status == "started".to_string();
//         }
//     }
// }

// fn create_ticket(data: &Vec<HoursData>, ticket_number: String, desc: String) {
//     data.push(HoursData { ticket: ticket_number, description: desc, time: "Test".to_string(), status: "stopped".to_string() });
// }

fn main() {
    // let test_data = vec!{
    //     HoursData { ticket : "TEST-123".to_string(), description: "Test Description".to_string(), time: "12".to_string() }
    // };
    // let args = env::args();
    // let args = args.as_slice();
    let args: Vec<_> = env::args().collect();
    let mut data = get_hours().unwrap();
    if args.len() > 1 {
        if args[1] == "add".to_string() {
            let mut ticket : String = "UNKNOWN".to_string();
            let mut desc : String = "NOTHING".to_string();
            if args.len() > 2 {
                ticket = args[2].clone();
            }
            if args.len() > 3 {
                desc = args[3].clone();
            }
            // create_ticket(&data, ticket, desc);
            data.push(HoursData { ticket: ticket, description: desc, time: "Test".to_string(), status: "stopped".to_string() });
            write_hours(&data);
            list_hours(&data);
        } else if args[1] == "start".to_string() {
            //TODO: If ticket does not exist here we need to create and start it, otherwise just start it up.
            // let remaining: Vec<HoursData> = data.to_vec().retain(|x| x.ticket == args[2]);
            // if remaining.len() > 0 {
            //     start_ticket(args[1]);
            // }
            
            // let num = 0;
            // // let index = 0;
            // for hour_item in data {
            //     if (hour_item.ticket == args[1]) {
            //         hour_item.status = "started";
            //     }
            // }
            for hour_item in data.iter() {
                if hour_item.ticket == args[1] {
                    hour_item.status == "started".to_string();
                }
            }
            // start_ticket(&data, args[1].clone());
            list_hours(&data);
        } else if args[1] == "stop".to_string() {
            list_hours(&data);
        } else if args[1] == "delete".to_string() {
            if args.len() > 2 {
                data.retain(|x| x.ticket != args[2]);
                write_hours(&data);
            }
            list_hours(&data);  
        } else {
            list_hours(&data).unwrap();
        }
    } else {
        list_hours(&data).unwrap();
    }
}