use fluence::sdk::*;
use std::cell::RefCell;

#[allow(unused_imports)]
use log::info;
use httparse::Request;
use httparse::Status;
use url::Url;
use url::form_urlencoded::parse as urlencoded_parse;

#[derive(Default)]
struct Post {
    text: String,
}

impl Post {
    fn from_urlencoded(data: &[u8]) -> Post {
        let mut post = Post::default();
        for key_val in urlencoded_parse(data) {
            if key_val.0 == "text" {
                post.text = key_val.1.to_string();
            }
        }
        post
    }
}

thread_local! {
    static POSTS: RefCell<Vec<Post>> = RefCell::new(Vec::new());
}

#[allow(dead_code)]
fn init() {
    logger::WasmLogger::init_with_level(log::Level::Info).unwrap();
}


#[allow(dead_code)]
fn main() {
    println!(
        "{}",
        entry_point(
            "GET /vks/v1/upload HTTP/1.1\r\nHost: urh.ru\r\n\r\n{\"keytext\":\"ZZZ\"}".to_owned()
        )
    );
}

#[invocation_handler(init_fn = init)]
pub fn entry_point(raw: String) -> String {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = { Request::new(&mut headers) };

    let parsed = match req.parse(raw.as_bytes()) {
        // I know here is better way, but i forgot it
        Err(e) => return format!("HTTP Parse Error : {}", e.to_string()),
        Ok(Status::Partial) => return "Can't process incomplete request".to_owned(),
        Ok(Status::Complete(p)) => p,
    };

    let p = req.path.unwrap();
    let u;
    if p[0..1].eq("/") {
        u = Url::parse(&format!("http://localhost{}", p));
    } else {
        u = Url::parse(p);
    }
    let u = u.unwrap();
    let path = u.path();

    if path.eq("/") {
        if req.method == Some("GET") {
            list(req)
        } else if req.method == Some("POST") {
            post(req, &raw[parsed..])
        } else {
            serve_wrong(req)
        }
    } else {
        serve_wrong(req)
    }
}

fn template(content: String) -> String {
    format!(
        "
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>Guest BOOK!</title>
</head>
<body>
<div>
<form action=\"/\" type=\"POST\">
<input type=\"text\" name=\"text\"><br>
<input type=\"submit\" value=\"Submit\">
</form>
</div>
{}</body>
</html>
",
        content
    )
}

fn post_template(post: &Post) -> String {
    format!("<p>{}</p>", post.text)
}

fn list_template(posts: &[Post]) -> String {
    let list: String = posts.into_iter().map(post_template).collect();
    format!("<div>{}</div>", list)
}

fn list(_: Request) -> String {
    let resp = POSTS.with(|posts| template(list_template(&posts.borrow())));
    format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Type: text/html\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\r\n\
        {}",
        resp.as_bytes().len(),
        resp
    )
}

fn post(_: Request, body: &str) -> String {
    if body.len() == 0 {
        format!("400 Bad Request")
    } else {
        let post = Post::from_urlencoded(body.as_bytes());
        if post.text.len() == 0 {
            format!("400 Bad Request")
        } else {
            POSTS.with(|posts| posts.borrow_mut().push(post));
            let resp = POSTS.with(|posts| template(list_template(&posts.borrow())));
            format!(
                "HTTP/1.1 200 OK\r\n
                Content-Type: text/html\r\n\
                Content-Length: {}\r\n\
                Connection: close\r\n\r\n\
                {}",
                resp.as_bytes().len(),
                resp
            )
        }
    }
}

fn serve_wrong(_: Request) -> String {
    format!("HTTP/1.1 400 Bad Request \r\n\r\n")
}

#[cfg(test)]
mod test {
    use crate::{ entry_point, POSTS};
    #[test]
    fn it_works () {
        entry_point(
            "GET / HTTP/1.1\r\nHost: urh.ru\r\n\r\n".to_owned()
        );
        POSTS.with(|posts| assert_eq!(posts.borrow().len(), 0));
        entry_point(
            "POST / HTTP/1.1\r\nHost: urh.ru\r\n\r\ntext=BlaBlaBla".to_owned()
        );
        entry_point(
            "GET / HTTP/1.1\r\nHost: urh.ru\r\n\r\n".to_owned()
        );
        POSTS.with(|posts| assert_eq!(posts.borrow()[0].text, "BlaBlaBla"));
    }
}
