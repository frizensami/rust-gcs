mod vehicle;
mod vehiclemanager;

extern crate mavlink;
use vehicle::Vehicle;
use vehiclemanager::VehicleManager;

fn main() {
    println!("Hello, world!");
    let v1: Vehicle = Vehicle::new("tcp:127.0.0.1:5760".to_owned());
    let mut vm = VehicleManager::new();
    let v1_vm = vm.add_and_start_vehicle(v1);

    loop {
        match v1_vm.lock().unwrap().try_recv_message() {
            Some(msg) => println!("{:?}", msg),
            None => println!("No message")
        }
    }



}
