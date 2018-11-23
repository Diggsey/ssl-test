use std::time::Duration;

use reqwest;
use url::Url;

fn main() {
    let endpoint = "https://dev_services.optibet.lv/api/passfort/passfort.php";
    let maybe_proxy_url = Some("socks5h://webhooks:QCFK%23D3Ds%5DHZGN%25ww%5By8Eg97JUwaHH43@35.195.138.186:33080");
    // Make sure we have a timeout
    let mut client_builder = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(5));

    // If we're using a proxy, configure that here
    if let Some(proxy_url) = maybe_proxy_url {
        // Construct the proxy scheme from the URL
        let proxy_scheme = reqwest::ProxyScheme::parse(
            Url::parse(proxy_url).expect("Invalid proxy URL")
        ).expect("Unknown proxy scheme");

        // Configure *all* requests to go via our proxy
        client_builder = client_builder.proxy(
            reqwest::Proxy::all(proxy_scheme).expect("Failed to configure proxy")
        );
    }

    let client = client_builder.build()
        .expect("Failed to build reqwest client");

    // Send the get request
    let mut response = client.get(endpoint)
        .bearer_auth("dummy_secret")
        .send()
        .expect("Failed to send request");

    // Read the response text
    let response_text = response.text()
        .expect("Failed to read response text");

    // Check status code
    let status_code = response.status();
    println!("{}: {}", status_code, response_text);
}
