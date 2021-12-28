//! **Common Syntax**
//!
//! This module defines all syntax that is shared across the symbol library,
//! footprint library, schematic, board, and work sheet file formats.

mod effects;
mod font;
mod justify;
mod paper;
mod point;
mod point_list;
mod position;
mod title_block;

pub use effects::Effects;
pub use font::{Font, FontSize};
pub use justify::{Justify, JustifyHoriz, JustifyVert};
pub use paper::{Paper, PaperSize};
pub use point::Point;
pub use point_list::PointList;
pub use position::Position;
pub use title_block::TitleBlock;
