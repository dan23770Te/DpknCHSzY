use rocket::get;
use rocket::post;
use rocket::delete;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::response::status;
use std::collections::HashMap;

#[macro_use]
extern crate rocket;

// Define the model for an item in the inventory
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    id: u32,
    name: String,
    quantity: u32,
}

// Define the inventory management service
#[derive(Debug)]
struct InventoryService {
    items: HashMap<u32, Item>,
    next_id: u32,
}

impl InventoryService {
    // Initialize a new inventory service
    fn new() -> InventoryService {
        InventoryService {
            items: HashMap::new(),
            next_id: 1,
        }
    }

    // Add a new item to the inventory
    fn add_item(&mut self, name: String, quantity: u32) -> u32 {
        let item = Item {
            id: self.next_id,
            name,
            quantity,
        };
        self.items.insert(self.next_id, item);
        self.next_id += 1;
        self.next_id - 1
    }

    // Remove an item from the inventory by ID
    fn remove_item(&mut self, id: u32) -> Option<Item> {
        self.items.remove(&id)
    }

    // Get all items in the inventory
    fn list_items(&self) -> Vec<Item> {
        self.items.values().cloned().collect()
    }

    // Get an item by ID
    fn get_item(&self, id: u32) -> Option<Item> {
        self.items.get(&id).cloned()
    }
}

#[post("/add", format = "json", data = "<item>")]
fn add_item_route(service: rocket::State<InventoryService>, item: Json<Item>) -> status::Created<Json<Item>> {
    let mut service = service.lock().unwrap();
    let added_item_id = service.add_item(item.name.clone(), item.quantity);
    status::Created::new("/inventory/items").json(service.get_item(added_item_id).unwrap())
}

#[delete("/remove/<id>")]
fn remove_item_route(service: rocket::State<InventoryService>, id: u32) -> status::Ok<Json<Item>> {
    let mut service = service.lock().unwrap();
    status::Ok::new().json(service.remove_item(id).unwrap())
}

#[get("/items")]
fn list_items_route(service: rocket::State<InventoryService>) -> Json<Vec<Item>> {
    let service = service.lock().unwrap();
    Json(service.list_items())
}

#[get("/items/<id>")]
fn get_item_route(service: rocket::State<InventoryService>, id: u32) -> Option<Json<Item>> {
    let service = service.lock().unwrap();
    service.get_item(id).map(Json)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(InventoryService::new())
        .mount("/inventory", routes![
            add_item_route,
            remove_item_route,
            list_items_route,
            get_item_route,
        ])
}
