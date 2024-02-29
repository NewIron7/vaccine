mod request;

use request::*;

fn main() {
    // the first argument is an url
    // get it and handle errors
    let url = std::env::args().nth(1).expect("missing url");

    let response = get(&url).expect("get request failed");
    println!("Status: {}", response.status());
    //println!("Headers:\n{:#?}", response.headers());
    // println!("Body:\n{}", response.text().expect("could not read response body"));
    let forms = get_form_data(&response.text().expect("could not read response body"));
    println!("Forms:\n{:#?}", forms);
}



