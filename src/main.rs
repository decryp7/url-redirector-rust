use std::collections::HashMap;
use warp::{http::Uri, Filter, http::header::HeaderMap, path::FullPath};
use urlencoding::encode;

const NEW_HOST: &str = "https://new.host";
const OLD_HOST: &str = "https://old.host";
const OLD_PARAMETER_VALUE: &str = "old_parameter";
const NEW_PARAMETER_VALUE: &str = "new_parameter";
const TARGET_PARAMETER: &str = "target_parameter";

#[tokio::main]
async fn main() {
    //host a replacement webserver to accept request as the old host and respond
    //with a redirect to the new host
    let index = warp::any() //accept any path
        .and(warp::header::headers_cloned()) //check header for target host for verification
        .and(warp::path::full()) //get the full path
        .and(warp::query::<HashMap<String, String>>()) //get all the parameters passed with the URL
        .map(|h :HeaderMap, p: FullPath, q: HashMap<String, String>| {
            let mut new_url = p.as_str().to_owned();
            if h["host"] == OLD_HOST {
                new_url = new_url.replace(OLD_HOST, NEW_HOST);
                if q.contains_key(TARGET_PARAMETER) {
                    new_url.push_str(format!("id={}", encode(q[TARGET_PARAMETER].replace(OLD_PARAMETER_VALUE, NEW_PARAMETER_VALUE).as_str())).as_str());
                }
            }

            println!("Redirecting from {} to {}...", p.as_str(), &new_url);
            warp::redirect::temporary(new_url.parse::<Uri>().expect("Unable to parse the url!"))
        });

    //use port other then 80 to avoid conflict for web development
    warp::serve(index).run(([127, 0, 0, 1], 8787)).await;
}
