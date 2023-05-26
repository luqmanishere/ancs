//! A general purpose library of common Apple Notification Control Service types
//!
//! This crate is a general purpose library for common types found when working
//! with the Apple Notification Control Service protocol. You'll find `attributes`
//! and `characteristics` that are used for interacting with Apple Notification Control Service
//! as a client. These two modules contain all relevant components specified in
//! the  Apple Notification Control Service protocol standard. This library is low-level
//! and as such it is only concerned with handling serialization and deserialization of the 
//! ANCS application protocol.
//! 
//! ## Apple Notification Control Service Protocol
//! 
//! > The purpose of the Apple Notification Control Center Service is to give Bluetooth
//! accessories (that connect to iOS devices through a Bluetooth low-energy link) a 
//! simple convenient way to access many kinds of notifications that are generated on 
//! iOS devices.
//! 
//! The ANCS protocol utilizes Bluetooth low-energy and a GATT Service, Characteristics and 
//! Attributes to handle all data transport over Bluetooth low-energy. This library allows
//! for easy serialization and deserilization of the wire data for this protocol.
//! 
pub mod attributes;
pub mod characteristics;
use uuid::{uuid, Uuid};

pub const APPLE_NOTIFICATION_CENTER_SERVICE_UUID: Uuid =
    uuid!("7905F431-B5CE-4E99-A40F-4B1E122D00D0");
