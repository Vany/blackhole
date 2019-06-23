
use fluence::sdk::*;
use std::cell::RefCell;

#[allow(unused_imports)]
use log::info;
use httparse::Request;
use httparse::Status;
use url::Url;

thread_local! {
    static POSTS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

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

    if path.eq("/") {
        if req.method == Some("GET") {
            list(req)
        } else if req.method == Some("POST") {
            post(req, body)
        } else {
            serve_wrong(req, body)
        }
    }
    else {
        serve_wrong(req, body)
    }
}

fn template(content: String) -> String {
    format!("
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>Guest BOOK!</title>
</head>
<body>{}</body>
</html>
", content)
}

fn post_template(post: &String) -> String {
    format!("<p>{}</p>", post)
}

fn list_template(posts: &[String]) -> String {
    let list: String = posts.into_iter().map(post_template).collect();
    format!("<div>{}</div>", list)
}

fn list(req: Request) -> String {
    format!("200 OK\r\n\r\n{}", POSTS.with(
        |posts| template(
            list_template(
                &posts.borrow()
            )
        )))
}

fn post(req: Request, body: String) -> String {
    POSTS.with(|posts| posts.borrow_mut().push(body));
    format!("200 OK\r\n\r\n{}", POSTS.with(
        |posts| template(
            list_template(
                &posts.borrow()
            )
        )))
}

fn serve_wrong(r: Request, body: String) -> String{
    format!("400 WRONG REQUEST \r\n\r\n{} {} [ {} ]", r.path.unwrap(), r.method.unwrap(), body)
}


