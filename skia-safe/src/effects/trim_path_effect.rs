use crate::prelude::*;
use crate::{scalar, PathEffect};
use skia_bindings as sb;
use skia_bindings::SkPathEffect;

pub use skia_bindings::SkTrimPathEffect_Mode as Mode;
#[test]
fn test_mode_naming() {
    let _ = Mode::Inverted;
}

impl RCHandle<SkPathEffect> {
    pub fn trim(
        start_t: scalar,
        stop_t: scalar,
        mode: impl Into<Option<Mode>>,
    ) -> Option<PathEffect> {
        new(start_t, stop_t, mode)
    }
}

pub fn new(start_t: scalar, stop_t: scalar, mode: impl Into<Option<Mode>>) -> Option<PathEffect> {
    PathEffect::from_ptr(unsafe {
        sb::C_SkTrimPathEffect_Make(start_t, stop_t, mode.into().unwrap_or(Mode::Normal))
    })
}
