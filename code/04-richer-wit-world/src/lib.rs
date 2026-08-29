#[allow(warnings)]
mod bindings;

use bindings::{Guest, Point, Shape, ShapeError};

struct Component;

fn shoelace_area(points: &[Point]) -> f64 {
    // The shoelace formula: works for any simple polygon, given its
    // vertices in order. Chosen because it needs the whole `list<point>`,
    // not just one field of it - a stand-in for "a real function that
    // actually consumes a Canonical-ABI list of records."
    let n = points.len();
    let mut sum = 0.0;
    for i in 0..n {
        let j = (i + 1) % n;
        sum += points[i].x * points[j].y;
        sum -= points[j].x * points[i].y;
    }
    (sum / 2.0).abs()
}

impl Guest for Component {
    fn area(s: Shape) -> Result<f64, ShapeError> {
        match s {
            Shape::Circle(r) if r < 0.0 => {
                Err(ShapeError::NegativeDimension(format!("radius {r} is negative")))
            }
            Shape::Circle(r) => Ok(std::f64::consts::PI * r * r),

            Shape::Rectangle((w, h)) if w < 0.0 || h < 0.0 => Err(
                ShapeError::NegativeDimension(format!("dimensions ({w}, {h})")),
            ),
            Shape::Rectangle((w, h)) => Ok(w * h),

            Shape::Polygon(points) if points.len() < 3 => {
                Err(ShapeError::TooFewPoints(points.len() as u32))
            }
            Shape::Polygon(points) => Ok(shoelace_area(&points)),
        }
    }

    fn describe(s: Shape) -> String {
        match s {
            Shape::Circle(r) => format!("a circle of radius {r}"),
            Shape::Rectangle((w, h)) => format!("a {w}x{h} rectangle"),
            Shape::Polygon(points) => format!("a {}-vertex polygon", points.len()),
        }
    }
}

bindings::export!(Component with_types_in bindings);
