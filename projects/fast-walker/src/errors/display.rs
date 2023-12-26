use super::*;

impl Error for WalkError {}

impl Error for WalkErrorKind {}
impl Display for WalkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for WalkErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WalkErrorKind::IO { path, error } => {
                write!(f, "IO Error: {} - {}", path.display(), error)
            }
        }
    }
}
