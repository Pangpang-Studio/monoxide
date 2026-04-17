use crate::Point;

#[derive(Default, Debug, Clone)]
pub struct Ctx {
    pub strands: Vec<Strand>,
    pub last_pos: Point,
}

#[derive(Debug, Clone, Copy)]
pub enum Strand {
    Line {
        start: Point,
        end: Point,
    },
    Cubic {
        start: Point,
        c1: Point,
        c2: Point,
        end: Point,
    },
}

impl super::Ctx for Ctx {
    fn move_to(&mut self, to: Point) {
        self.last_pos = to;
    }

    fn line_to(&mut self, to: Point) {
        self.strands.push(Strand::Line {
            start: self.last_pos,
            end: to,
        });
        self.last_pos = to;
    }

    fn cubic_to(&mut self, c1: Point, c2: Point, to: Point) {
        self.strands.push(Strand::Cubic {
            start: self.last_pos,
            c1,
            c2,
            end: to,
        });
        self.last_pos = to;
    }
}
