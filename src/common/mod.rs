//! **Common Syntax**
//!
//! This module defines all syntax that is shared across the symbol library,
//! footprint library, schematic, board, and work sheet file formats.

mod effects;
mod font;
mod justify;
mod point;
mod point_list;
mod position;

pub use effects::Effects;
pub use font::{Font, FontSize};
pub use justify::{Justify, JustifyHoriz, JustifyVert};
pub use point::Point;
pub use point_list::PointList;
pub use position::Position;
