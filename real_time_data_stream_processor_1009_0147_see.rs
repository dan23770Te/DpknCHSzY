use rocket::get;
use rocket_contrib::json::Json;
use rocket::State;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::outcome::Outcome::{Success, Failure};
use rocket::Request;
use rocket::Outcome;

// Struct representing a data message
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct DataMessage {
    timestamp: Instant,
    payload: String,
}

// Struct to manage the data stream
struct StreamManager {
    data_queue: Arc<Mutex<VecDeque<DataMessage>>>,
}

// Implementation for StreamManager
impl StreamManager {
    fn new() -> Self {
        StreamManager {
            data_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    // Method to add data to the stream
    fn add_data(&self, data: DataMessage) {
        let mut data_queue = self.data_queue.lock().unwrap();
        data_queue.push_back(data);
    }

    // Method to get data from the stream
    fn get_data(&self) -> Vec<DataMessage> {
        let data_queue = self.data_queue.lock().unwrap();
        let mut data = Vec::new();
        std::mem::swap(&mut data, &mut data_queue);
        data
    }
}

// Rocket fairing to handle data stream
struct DataStreamFairing;

// Implementation for DataStreamFairing
impl Fairing for DataStreamFairing {
    fn info(&self) -> Info {
        Info {
            name: "Data Stream Manager",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut rocket::Response<'static>) -> Outcome {
        let mut outcome = Failure((Status::InternalServerError, "Internal Error"));

        if let Some(stream_manager) = request.rocket().state::<StreamManager>() {
            if let Ok(data) = stream_manager.get_data() {
                if !data.is_empty() {
                    let json_data = Json(data);
                    outcome = Success(response.set_body(json_data.into_value().into_body()));
                }
            }
        }

        outcome
    }
}

#[get("/stream")]
fn stream_data(stream_manager: &State<StreamManager>) -> Json<Vec<DataMessage>> {
    let data = stream_manager.get_data();
    Json(data)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![stream_data])
        .attach(DataStreamFairing)
        .manage(StreamManager::new())
}

// Helper function to simulate data generation
fn simulate_data_generation(stream_manager: &State<StreamManager>) {
    loop {
        let data = DataMessage {
            timestamp: Instant::now(),
            payload: "Data Payload".to_string(),
        };
        stream_manager.add_data(data);
        std::thread::sleep(Duration::from_millis(1000));
    }
}

// Main function to start the server
fn main() {
    // Start the Rocket server in a separate thread
    std::thread::spawn(move || {
        rocket().launch();
    });

    // Simulate data generation in the main thread
    let stream_manager = StreamManager::new();
    simulate_data_generation(&stream_manager);
}
