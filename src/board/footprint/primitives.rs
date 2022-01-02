use crate::{
	board::graphic::{
		Arc, Circle, Curve, GraphicItem, Line, Polygon, Rectangle, Text
	},
	mm
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Yes {
	Yes
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename = "fill")]
struct Fill(Yes);

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename = "width")]
struct Width(mm);

serde_sexpr::untagged! {
	enum Content {
		Arc(Arc),
		Circle(Circle),
		Curve(Curve),
		Line(Line),
		Poly(Polygon),
		Rect(Rectangle),
		Text(Text),

		Width(Width),
		Fill(Fill)
	}
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename = "primitives")]
struct PrimitivesDef {
	#[serde(default, rename = "")]
	content: Vec<Content>
}

impl From<Primitives> for PrimitivesDef {
	fn from(p: Primitives) -> Self {
		let mut content = Vec::with_capacity(p.items.len() + 2);
		for item in p.items {
			content.push(match item {
				GraphicItem::Arc(arc) => Content::Arc(arc),
				GraphicItem::Circle(circle) => Content::Circle(circle),
				GraphicItem::Curve(curve) => Content::Curve(curve),
				GraphicItem::Line(line) => Content::Line(line),
				GraphicItem::Poly(poly) => Content::Poly(poly),
				GraphicItem::Rect(rect) => Content::Rect(rect),
				GraphicItem::Text(text) => Content::Text(text)
			});
		}
		if let Some(w) = p.width {
			content.push(Content::Width(Width(w)));
		}
		if p.fill {
			content.push(Content::Fill(Fill(Yes::Yes)));
		}
		Self { content }
	}
}

impl From<PrimitivesDef> for Primitives {
	fn from(def: PrimitivesDef) -> Self {
		let mut items = Vec::with_capacity(def.content.len());
		let mut width = None;
		let mut fill = false;
		for c in def.content {
			match c {
				Content::Arc(arc) => items.push(GraphicItem::Arc(arc)),
				Content::Circle(circle) => items.push(GraphicItem::Circle(circle)),
				Content::Curve(curve) => items.push(GraphicItem::Curve(curve)),
				Content::Line(line) => items.push(GraphicItem::Line(line)),
				Content::Poly(poly) => items.push(GraphicItem::Poly(poly)),
				Content::Rect(rect) => items.push(GraphicItem::Rect(rect)),
				Content::Text(text) => items.push(GraphicItem::Text(text)),

				Content::Width(w) => width = Some(w.0),
				Content::Fill(Fill(Yes::Yes)) => fill = true
			};
		}
		Self { items, width, fill }
	}
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(from = "PrimitivesDef", into = "PrimitivesDef")]
pub struct Primitives {
	pub items: Vec<GraphicItem>,

	pub width: Option<mm>,

	pub fill: bool
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{common::Point, sexpr_test_case, Unit};
	use uuid::Uuid;

	sexpr_test_case! {
		name: primitives,
		input: r#"(primitives (gr_circle (center 1 1) (end 2 2) (width 0.12) (tstamp "00000000-0000-0000-0000-000000000000")))"#,
		value: Primitives {
			items: vec![GraphicItem::Circle(Circle {
				center: Point::new(1.0.mm(), 1.0.mm()),
				end: Point::new(2.0.mm(), 2.0.mm()),
				layer: None,
				width: 0.12.mm(),
				fill: None,
				tstamp: Uuid::nil()
			})],
			width: None,
			fill: false
		}
	}
}
