use fluence::sdk::*;

#[allow(unused_imports)]
use log::info;
use httparse::Request;
use httparse::Status;


#[allow(dead_code)]
fn init() {
    logger::WasmLogger::init_with_level(log::Level::Info).unwrap();
}


#[allow(dead_code)]
fn main() {
    println!("{}", entry_point("GET / HTTP/1.1\r\nHost: urh.ru\r\n\r\n{\"hello\":\"world\"}".to_owned()));
}



#[invocation_handler(init_fn = init)]
fn entry_point(name: String) -> String {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = {  Request::new(&mut headers) };


    let parsed = match req.parse(name.as_bytes()) { // I know here is better way, but i forgot it
        Err(e) => return format!("HTTP Parse Error : {}", e.to_string()),
        Ok(Status::Partial) => return "Can't process incomplete request".to_owned(),
        Ok(Status::Complete(p)) => p,
    };

    let body = String::from(&name[parsed..]);

    match req.path.unwrap() {
        "/" => serve_root(req, body),
        _ => serve_wrong(req, body),
    }
}




fn serve_root(r: Request, body: String) -> String{
    format!("200 OK\r\n\r\n{}", body)
}



fn serve_wrong(r: Request, body: String) -> String{
    format!("400 WRONG REQUEST \r\n\r\n{} {} [ {} ]", r.path.unwrap(), r.method.unwrap(), body)
}


