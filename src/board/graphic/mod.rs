mod arc;
mod circle;
mod curve;
mod line;
mod poly;
mod rect;
mod text;

pub use arc::Arc;
pub use circle::Circle;
pub use curve::Curve;
pub use line::Line;
pub use poly::Polygon;
pub use rect::Rectangle;
pub use text::Text;

serde_sexpr::untagged! {
	#[derive(Clone, Debug, Eq, PartialEq)]
	pub enum GraphicItem {
		Arc(Arc),
		Circle(Circle),
		Curve(Curve),
		Line(Line),
		Poly(Polygon),
		Rect(Rectangle),
		Text(Text)
	}
}
