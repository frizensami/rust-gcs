mod vehicle;
mod vehiclemanager;

extern crate mavlink;
use vehicle::Vehicle;
use vehiclemanager::VehicleManager;

fn main() {
    println!("Hello, world!");
    let v1: Vehicle = Vehicle::new("tcp:127.0.0.1:5760".to_owned());
    let v2: Vehicle = Vehicle::new("tcp:127.0.0.1:5770".to_owned());

    let mut vm = VehicleManager::new();

    let v1_vm = vm.add_and_start_vehicle(v1);
    let v2_vm = vm.add_and_start_vehicle(v2);

    loop {
        //println!("Main: v1_vm try_recv");
        match v1_vm.lock().unwrap().try_recv_message() {
            Some(msg) => println!("From 5760 - {:?}", msg),
            None => println!("No message from 5760")
        }

        //println!("Main: v2_vm try_recv");
        match v2_vm.lock().unwrap().try_recv_message() {
            Some(msg) => println!("From 5770 - {:?}", msg),
            None => println!("No message from 5770")
        }
    }



}
