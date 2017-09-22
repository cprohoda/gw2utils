use hyper::header::{Headers, Authorization, Bearer};
use std::fs::File;
use std::io::Result;

pub fn set_auth(headers: &mut Headers, apikey_location: &str) -> Result<()> {
    let apikey = read_apikey(apikey_location);

    headers.set(
        Authorization(
            Bearer{
                token: apikey.unwrap(),
            }
        )
    );
    Ok(())
}

fn read_apikey(apikey_location: &str) -> Result<String> {
    let mut apikey_file = File::open(apikey_location)?;
    let mut  apikey = String::new();
    apikey_file.read_to_string(&mut apikey)?;

    if apikey.rfind('\n') == Some(apikey.len()-1) { // strip last char if newline
        apikey.pop();
    }

    Ok(apikey)
}
