
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use random_string::generate;
use random_string::Charset;
use mongodb::{
    sync::{Client, Collection},
    bson::doc,
};
use rocket::response::{
    Response,
    Redirect,
    content::{self, Html},
};
use rocket::http::Status;
use rocket::request::Form;
use std::fs;
use std::io::Cursor;


#[derive(FromForm)]
pub struct UrlBody {
    url: String,
}

fn get_url_collection() -> Collection {
    match Client::with_uri_str("mongodb://localhost:27017") {
        Ok(client) => client.database("ienza-tech").collection("urls"),
        Err(e) => {
            println!("Error while communicating with MONGODB{:?}", e);
            panic!()
        },
    }
}

fn get_id() -> String {
    let charset = Charset::new("123456789abcdefghijklmnopqrstuvwxyz");
    match charset {
        Some(charset) => generate(6, &charset).to_string(),
        None => "Somehow this broke".to_string()
    }
}

fn get_redirect(redirect_url: String) -> Redirect{
    Redirect::to(redirect_url)
}

#[get("/errors/internal")]
fn get_internal_error_message() -> String {
    "There was an interal server error while fetching your URL, sorry lol".to_string()
}

#[get("/errors/bad")]
fn get_bad_message() -> String {
    "The URL you requested is not in our database".to_string()
}

/// Given an ID get th
#[get("/<id>")]
fn get_url(id: String) -> Redirect {
    let collection = get_url_collection();
    match collection.find_one(doc! { "id": id.clone()}, None) {
        Ok(Some(result)) => match result.get_str("url") {
            Ok(url) =>  get_redirect(url.to_string()),
            Err(e) => {
                println!("Database error: Found object in database but could not get URL: {:?}", e);
                get_redirect("/errors/internal".to_string())
            }
        },
        Ok(None) => get_redirect("/errors/bad".to_string()),
        Err(e) => {
            println!("Database error while trying to get the URL for the given id: {:?}", e);
            get_redirect("/errors/internal".to_string())
        }
    }
}

#[get("/manage/all")]
fn get_all() -> Response< 'static> {
    let mut vars = String::from("{");
    let collection = get_url_collection();
    match collection.find(doc! {}, None) {
        Ok(cursor) => {
            for i in cursor {
              match i {
                Ok(doc) => {
                    match doc.get_str("id") {
                        Ok(id) => match doc.get_str("url") {
                            Ok(url) => {
                                vars.push_str(&"\"".to_string());
                                vars.push_str(&id.to_string());
                                vars.push_str(&"\":".to_string());
                                vars.push_str(&url.to_string());
                                vars.push_str(&",".to_string());
                            },
                            Err(e) => println!("Error getting doc {:?}", e)
                        },
                        Err(e) => println!("Error getting doc {:?}", e)
                    }
                    vars.push_str(&doc.to_string());
                    vars.push_str(&",".to_string());
                },
                Err(e) => println!("Error getting doc {:?}", e)
              }
            }
        },
        Err(e) => {
            println!("Database error while getting all docs {:?}", e);
        }
    }
    vars.push_str(&"}".to_string());
    let mut response = Response::new();
    response.set_sized_body(Cursor::new(vars));
    response.adjoin_raw_header("Access-Control-Allow-Origin", "https://manage.url.ienza.tech");
    response.set_status(Status::Accepted);
    return response;
}


/// Add a url to the database (only if needed) and get the id of the object for the short url
#[post("/manage/add", data = "<body>")]
fn add_url(body: Form<UrlBody>) -> String {

    let url = body.url.clone();
    let collection = get_url_collection();

    match collection.find_one(doc! { "url": url.clone()}, None) {
        Ok(Some(result)) => match result.get_str("id") {
            Ok(id) => return id.to_string(),
            Err(e) => {
                println!("Database error while getting the ID that matches the URL: {:?}", e);
                return "Internal Database Error".to_string();
            }
        },
        Ok(None) => {
            let mut n = 30;
            while n > 0 {
                let id = get_id();
                match collection.find_one(doc! { "id": id.clone()}, None) {
                    Ok(Some(_result)) => {
                        n -= 1;
                        continue;
                    },
                    Ok(None) => {
                        match collection.insert_one(doc! {"url": url.clone(), "id": id.clone()}, None) {
                            Ok(_result) => return format!("Your new url is: https://url.ienza.tech/{}", id),
                            Err(e) => {
                                println!("Database error while inserting key and URL to db: {:?}", e);
                                return "Internal Database Error".to_string();
                            }
                        }
                    },
                    Err(e) => {
                        println!("Server error while making sure key is unique: {:?}", e);
                        return "Internal Database Error".to_string();
                    }
                }
            }
            return "Unable to generate a unique key after 30 attemps".to_string();
        }
        Err(e) => {
            println!("Database error while checking for existing key for the URL: {:?}", e);
            return "Internal Database Error".to_string();
        }
    }
}

/// Ignite the rocket and then sit patiently and wait while it crushes the game
fn main() {
    rocket::ignite().mount("/", routes![get_all, get_url, add_url, get_bad_message, get_internal_error_message]).launch();
}
