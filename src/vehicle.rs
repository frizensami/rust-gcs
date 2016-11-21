
extern crate mavlink;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use self::mavlink::combined::MavMessage;

/// Represents a mavlink-connected vehicle which will try to keep
/// the connection alive.
pub struct Vehicle {
    pub conn_string: String,
    pub conn: Option<Box<mavlink::MavConnection + Sync + Send>>,
    pub reconnect_requested: Arc<AtomicBool>
}

impl Vehicle {

    /// Creates a new Vehicle object from a connection string
    pub fn new(connection_string: String) -> Vehicle {
        Vehicle { conn_string: connection_string, conn: None, reconnect_requested: Arc::new(AtomicBool::new(false)) }
    }

    /// Attempts to send a message to the vehicle endpoint
    pub fn try_send_message(&mut self, msg: MavMessage) -> () {
        if let Some(ref conn) = self.conn {
            match conn.send(&msg) {
                Ok(_)  => { self.set_reconnect_requested(false); println!("send success"); }
                Err(_) => { self.set_reconnect_requested(true); println!("send fail"); },
            }
        }
    }

    /// Blocks until a message is received or an error is encountered
    pub fn try_recv_message(&mut self) -> Option<MavMessage> {
        if let Some(ref conn) = self.conn {
            return match conn.recv() {
                Ok(msg) => { self.set_reconnect_requested(false); return Some(msg); },
                Err(_)  => { self.set_reconnect_requested(true); println!("recv fail"); return None;  }
            }
        } else {
            return None;
        }

    }

    /// Tries once to make a (blocking) connection to a Mavlink endpoint
    /// specified by the connection string for this vehicle
    pub fn try_start_connection(&mut self) -> bool {
        println!("Trying to connect to {}", self.conn_string);
        match mavlink::connect(&self.conn_string[..]) {
            Ok(conn) => { self.conn = Some(conn); println!("connection established"); self.set_reconnect_requested(false); return true; },
            Err(_) => { self.conn = None; println!("failed to connect"); self.set_reconnect_requested(true); return false; }
        }
    }

    /// Checks the value of the atomic reconnect field
    pub fn is_reconnect_requested(&self) -> bool {
        return self.reconnect_requested.load(Ordering::Relaxed);
    }


    /// Sets the atomic reconnect requested variable to the required value
    fn set_reconnect_requested(&self, reconnect_requested: bool) {
        (*self.reconnect_requested).store(reconnect_requested, Ordering::Relaxed);
    }
}
