# Overview

Rust-GCS aims to be a **reliable** and **fast** ground-control station for vehicles that communicate over the MAVLink protocol

## Installation

## Architecture
`mavlink` crate: Provides a way to connect (through tcp/udpin/udpout) to a mavlink-message-sending source. `connect` returns an `io::Result<Box<MavConnection + Sync + Send>>` that represents the connection object for vehicle. <br>

`vehicle` lib: Contains the connection object that is used to send/recv messages to the vehicle.

`vehiclemanager` lib: Maintains an admin thread for each vehicle: which does connection maintainence + sendbuffer + receivebuffer.
