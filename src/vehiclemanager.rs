extern crate mavlink;

use vehicle::Vehicle;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use mavlink::combined;

pub struct VehicleManager {
    /// THE MUTEX WAS IMPORTANT
    /// Vector of vehicles - each wrapped in their own Arc<Mutex<..>> for safe
    /// multithreaded access.
    pub vehicles: Vec<Arc<Mutex<Vehicle>>>
}

impl VehicleManager {
    pub fn new() -> VehicleManager {
        return VehicleManager { vehicles: Vec::new() };
    }

    /// Adds a particular vehicle to the list of currently managed vehicles and starts the connection
    pub fn add_and_start_vehicle(&mut self, vehicle: Vehicle) -> Arc<Mutex<Vehicle>> {
       let new_vehicle = Arc::new(Mutex::new(vehicle));
       let veh_clone = new_vehicle.clone();
       let veh_return = new_vehicle.clone();
       self.vehicles.push(new_vehicle);


       thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(500));
                let mut current_vehicle = veh_clone.lock().unwrap();

                // Generally -> try to reconnect to the vehicle if we need to
                if current_vehicle.is_reconnect_requested() || current_vehicle.conn.is_none() {
                    println!("trying to (re)start connection (vehiclemanager)");

                    // If we're reconnecting to the vehicle - we should re-request for params/data stream
                    let is_connection_successful = current_vehicle.try_start_connection();

                    // If any of these sends fail - we'll be requested to reconnect
                    if is_connection_successful {
                      current_vehicle.try_send_message(mavlink::request_parameters());
                      current_vehicle.try_send_message(mavlink::request_stream());
                    }
                } else {
                    // Send heartbeat messages
                    current_vehicle.try_send_message(mavlink::heartbeat_message());
                }
                // Note that the lock is RELEASED HERE IMPLICITLY: we shouldn't delay its release
            }
       });

       return veh_return;
   }

}



