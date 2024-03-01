mod request;
use request::*;

mod exploit;
use exploit::*;

mod payloads;

fn main() {
    // the first argument is an url
    // get it and handle errors
    let url = std::env::args().nth(1).expect("missing url");

    let response = get(&url).expect("get request failed");
    // println!("Status: {}", response.status());
    //println!("Headers:\n{:#?}", response.headers());
    // println!("Body:\n{}", response.text().expect("could not read response body"));
    let forms = get_form_data(&response.text().expect("could not read response body"));
    // set the correct url for the forms
    let forms: Vec<FormData> = forms
        .into_iter()
        .map(|form| FormData {
            url: create_url(&url, &form.url),
            method: form.method,
            params: form.params,
        })
        .collect();
    // println!("Forms:\n{:#?}", forms);

    // loop forms 
    for form in forms {
        let sample = sample_form(&url, &form);
        if !test_form(&url, &form, sample) {
            continue;
        }
    }
}



