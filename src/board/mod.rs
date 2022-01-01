//! **Board Common Syntax**
//!
//! This module defines all syntax that is shared across the footprint library and
//! printed circuit board file formats.

mod connect_pads;
pub mod footprint;
mod footprint_module;
mod layer;
mod timestamp;

pub use connect_pads::ConnectPads;
pub use footprint::Footprint;
pub use layer::Layer;
pub use timestamp::Timestamp;
