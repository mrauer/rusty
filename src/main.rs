use reqwest::blocking::Client;
use serde_json::Value;

fn main() {
    // Create a reqwest Client
    let client = Client::new();

    // Define the URL of the API endpoint
    let url = "https://reqres.in/api/users/8";

    // Make a GET request to the API endpoint
    match client.get(url).send() {
        // If the request is successful, check the status code
        Ok(response) => {
            match response.status() {
                // If the status code is 200 OK, print a success message
                reqwest::StatusCode::OK => {
                    // Deserialize the JSON response
                    let json: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
                    // Extract the email address
                    let email = json["data"]["email"].as_str().unwrap_or("Email not found");
                    println!("Email: {}", email);
                }
                // For any other status code, print the code and reason phrase
                status => println!("Request failed with status code: {:?}", status),
            }
        }
        // If the request fails, print the error message
        Err(e) => println!("Error: {}", e),
    }
}
