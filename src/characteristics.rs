//! ## Characteristics
//! 
//! Characteristics are the most common way to interact with Bluetooth Low Energy 
//! devices. Each service contains a set of characteristics that define a set of properties
//! and handle the transmission of a subset of attributes (data). The ANCS GATT service defines
//! the following characteristics Notification Source, Control Point and Data Source. Each 
//! of which handle a different part of the Request/Response lifecycle.
//! 
pub mod control_point;
pub mod data_source;
pub mod notification_source;
