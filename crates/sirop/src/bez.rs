mod strand;

pub use self::strand::{Ctx as StrandCtx, Strand};
use crate::Point;

pub trait Ctx {
    fn move_to(&mut self, to: Point);
    fn line_to(&mut self, to: Point);
    fn cubic_to(&mut self, c1: Point, c2: Point, to: Point);

    fn mark_knot(&mut self, _idx: usize) {}

    fn start(&mut self) {}
    fn end(&mut self) {}
}
