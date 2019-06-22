use fluence::sdk::*;

//use rocket::request::Request;


#[invocation_handler]
fn entry_point(name: String) -> String {
    // let mut x = rocket::request::Request;
    format!("KEYSERVER RESP:  {}", name)
}