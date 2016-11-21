#![allow(unused_imports)]
#![allow(dead_code)]

use vehicle::Vehicle;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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
                let mut current_vehicle = veh_clone.lock().unwrap();
                if current_vehicle.is_reconnect_requested() || current_vehicle.conn.is_none() {
                    println!("trying to start connection (vehiclemanager)");
                    current_vehicle.try_start_connection();
                    thread::sleep(Duration::from_millis(500));
                }
            }
       });

       return veh_return;
   }

}



