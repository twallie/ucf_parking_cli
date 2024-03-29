use reqwest::Response;
use serde::{Serialize, Deserialize};

enum RequestError {
    APIResponseError,
    BodyExtractionError,
    JSONParsingError
}

#[derive(Serialize, Deserialize)]
struct Garage {
    max_spaces: u16,
    name: String,
    percent_full: f32,
    spaces_filled: u16,
    spaces_left: u16
}

#[derive(Serialize, Deserialize)]
struct UCFGaragesAPIData {
    garages: GarageArray
}

type GarageArray = [Garage; 7];

#[tokio::main]
async fn main() {
    let garages: GarageArray = match build_garage_array(true).await {
        Ok(v) => {
            v
        },
        Err(_) => {
            return;
        }
    };

    print_garages_filled_and_total(&garages, "/");
}

async fn build_garage_array(print_if_error: bool) -> Result<GarageArray, RequestError> {
   return match build_ucf_garages_api_object().await {
        Ok(v) => {
            Ok(v.garages)
        },
        Err(e) => {
            if print_if_error {
                print_error_message(&e);
            }
            Err(e)
        }
    };
 
}

fn print_error_message(e: &RequestError) {
    match e {
        RequestError::APIResponseError => {
            println!("No response from API.");
        },
        RequestError::BodyExtractionError => {
            println!("Couldn't parse API response body.");
        },
        RequestError::JSONParsingError => {
            println!("Couldn't parse body into JSON.");
        }
    }
}

fn print_garages_filled_and_total(garages: &GarageArray, delimiter: &str) {
    for garage in garages {
        println!("{}\t{}{delimiter}{}", garage.name, garage.spaces_filled, garage.max_spaces);
    }
}

async fn build_ucf_garages_api_object() -> Result<UCFGaragesAPIData, RequestError> {
    let res = match create_ucf_parking_api_response().await {
        Ok(v) => {
            v
        },
        Err(e) => {
            return Err(e);
        }
    };

    let string_body = match extract_response_body_as_string(res).await {
        Ok(v) => {
            v
        },
        Err(e) => {
            return Err(e);
        }
    };

    let data = match create_ucf_garages_api_object(&string_body) {
        Ok(v) => {
            v
        },
        Err(e) => {
            return Err(e);
        }
    };

    Ok(data)
}

fn create_ucf_garages_api_object(body: &String) -> Result<UCFGaragesAPIData, RequestError> {
    match serde_json::from_str(body) {
        Ok(v) => {
            return Ok(v)
        },
        Err(_) => {
            return Err(RequestError::JSONParsingError)
        }
    };
}

async fn extract_response_body_as_string(res: Response) -> Result<String, RequestError> {
    match res.text().await {
        Ok(v) => {
            return Ok(v)
        },
        Err(_) => {
            return Err(RequestError::BodyExtractionError);
        }
    }
}

async fn create_ucf_parking_api_response() -> Result<Response, RequestError> {
    let request_result = 
        reqwest::get("https://api.ucfgarages.com/")
        .await;

    match request_result {
        Ok(req) => {
            return Ok(req);
        },
        Err(_) => {
            return Err(RequestError::APIResponseError);
        }
    };

}