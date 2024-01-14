use reqwest::Response;
use serde::{Serialize, Deserialize};

struct NoAPIResponseError();

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
    print_data().await;
}

async fn print_data() -> Result<(), NoAPIResponseError> {
    let request_result = reqwest::get("https://api.ucfgarages.com/")
        .await;
    
    let req: Response = match request_result {
        Ok(req) => {
            req
        },
        Err(_) => {
            return Err(NoAPIResponseError());
        }
    };

    let body = req.text().await.unwrap();
    let data: UCFGaragesAPIData = serde_json::from_str(&body).unwrap();
    println!("{} has {} spaces left", &data.garages[0].name, &data.garages[0].spaces_left);

    Ok(())
}