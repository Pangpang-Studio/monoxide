use monoxide_curves::point::Point2D;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    L,
    R,
    U,
    D,
}

impl From<Dir> for Point2D {
    fn from(dir: Dir) -> Self {
        match dir {
            Dir::L => (-1., 0.),
            Dir::R => (1., 0.),
            Dir::U => (0., 1.),
            Dir::D => (0., -1.),
        }
        .into()
    }
}
