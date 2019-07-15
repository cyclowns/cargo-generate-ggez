//! Basic error types.

use ggez;

#[derive(Debug)]
pub struct GgezError {
    err: ggez::GameError,
}

impl From<ggez::GameError> for GgezError {
    fn from(err: ggez::GameError) -> Self {
        Self { err }
    }
}
