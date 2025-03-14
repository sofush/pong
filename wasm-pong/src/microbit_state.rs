use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize, Default)]
pub struct MicrobitState {
    pub left_pressed: bool,
    pub right_pressed: bool,
}
