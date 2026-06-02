use monoxide_curves::point::Point2D;
use monoxide_script::dsl::IntoStrokeAlignment;

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

/// Alignment of stroke relative to the moving direction.
/// See [`monoxide_curves::SpiroCurve::alignment`] for details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Middle,
    Right,
}

impl IntoStrokeAlignment for Alignment {
    fn into_alignment(self) -> f64 {
        match self {
            Alignment::Left => 0.0,
            Alignment::Middle => 0.5,
            Alignment::Right => 1.0,
        }
    }
}
