use reqwest::{blocking::Client, blocking::Response};
use scraper::Html;

/// Function that send a post request to a given url
/// with a given body
/// This function is synchronous, it uses the blocking of the reqwest library
pub fn post(url: &String, body: &String, content_type: &String) -> Result<Response, reqwest::Error> {
    // create a client
    // with de content type
    let client = Client::new();
    // convert the &String into a String
    let body_string = body.to_string();
    // send a post request to the given url with the given body
    let response = client
        .post(url)
        .header("Content-Type", content_type)
        .body(body_string)
        .send()?;
    Ok(response)
}

/// Function that send a get request to a given url
/// This function is synchronous, it uses the blocking of the reqwest library
/// Use a fake google user agent
pub fn get(url: &String) -> Result<Response, reqwest::Error> {
    // create a client
    // add a user agent to the client
    let client = Client::new();
    let client = client.get(url);

    let response = client.send()?;
    // return the result
    Ok(response)
}

/// Struct that represents a form data
#[derive(Debug)]
pub struct FormData {
    pub url: String,
    pub method: String,
    pub params: Vec<String>,
}

/// Function that looks for form data in a body
/// it looks for input tags in form tags with the method post and with all type vulnerable to sql injection
/// and returns a vector of FormData with the url of the form and the names of the inputs tags
/// and the name of the input tag
pub fn get_form_data(body: &String) -> Vec<FormData> {
    let document = Html::parse_document(body);
    let selector_form = scraper::Selector::parse("form").unwrap();
    let form_tags = document.select(&selector_form);
    let mut form_data: Vec<FormData> = Vec::new();
    for form_tag in form_tags {
        let method = form_tag.value().attr("method");
        let method = method.map(|s| s.to_lowercase());
        let method = method.unwrap_or("post".to_string());
        let action = form_tag.value().attr("action").unwrap_or("");
        let selector_input = scraper::Selector::parse("input").unwrap();
        let input_tags = form_tag.select(&selector_input);
        let mut params: Vec<String> = Vec::new();
        for input_tag in input_tags {
            // check type
            let input_type = input_tag.value().attr("type").unwrap_or("");
            if input_type != "text" && input_type != "password" {
                continue;
            }
            let name = input_tag.value().attr("name").unwrap_or("");
            params.push(name.to_string());
        }
        form_data.push(FormData {
            url: action.to_string(),
            method: method,
            params: params,
        });
    }
    form_data
}
