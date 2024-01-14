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
    garages: Vec<Garage>
}

#[tokio::main]
async fn main() {
    let garages = match build_ucf_garages_api_object().await {
        Ok(v) => {
            v.garages
        },
        Err(_) => {
            println!("ERROR");
            return;
        }
    };

    for garage in garages {
        println!("{}\t{}/{}", garage.name, garage.spaces_filled, garage.max_spaces);
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