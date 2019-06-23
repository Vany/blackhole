use fluence::sdk::*;

#[allow(unused_imports)]
use log::info;
use httparse::Request;
use httparse::Status;
use url::Url;

#[allow(dead_code)]
fn init() {
    logger::WasmLogger::init_with_level(log::Level::Info).unwrap();
}


#[allow(dead_code)]
fn main() {
    println!("{}", entry_point("GET /vks/v1/upload HTTP/1.1\r\nHost: urh.ru\r\n\r\n{\"keytext\":\"ZZZ\"}".to_owned()));
}



// #[invocation_handler(init_fn = init)]
fn entry_point(name: String) -> String {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = {  Request::new(&mut headers) };


    let parsed = match req.parse(name.as_bytes()) { // I know here is better way, but i forgot it
        Err(e) => return format!("HTTP Parse Error : {}", e.to_string()),
        Ok(Status::Partial) => return "Can't process incomplete request".to_owned(),
        Ok(Status::Complete(p)) => p,
    };

    let body = String::from(&name[parsed..]);

    let p = req.path.unwrap();

    let u;
    if p[0..1].eq("/") {
        u =  Url::parse( &format!("http://localhost{}", p));
    } else {
        u = Url::parse(p);
    }
    let u = u.unwrap();


    let path = u.path();


    println!(">>> {} <<<", u.path());

    if path.eq("/") { serve_root(req, body) }
    else if path.contains("/vks/v1/upload") { serve_upload(req, body) }
    else { serve_wrong(req, body) }
}




fn serve_root(r: Request, body: String) -> String{
    format!("200 OK\r\n\r\n{}", body)
}


use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Upload {
    keytext: String
}

fn serve_upload(r: Request, body: String) -> String{
    let input : Upload = serde_json::from_str(body.as_str()).unwrap();
    format!("200 OK\r\n\r\n{}", input.keytext)
}


fn serve_wrong(r: Request, body: String) -> String{
    format!("400 WRONG REQUEST \r\n\r\n{} {} [ {} ]", r.path.unwrap(), r.method.unwrap(), body)
}


