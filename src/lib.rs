//! centered-crosshair for Horizon: the mod API wiring.
//!
//! The upstream mod is entirely render-side and has no config, so there are
//! no settings to register and no client state to set up: the whole mod is
//! the 1:1 math core in `runtime` (the quarter-pixel-quantised crosshair
//! anchor from the raw window size, the subpixel textured-quad path, and the
//! `Math.round` semantics), 10 tests green.
//!
//! The one Horizon seam: `GuiDraw::Sprite` carries i32 x/y only, so the
//! actual draw of a float-position HUD sprite has no API surface yet. The
//! draw step waits on that surface (see the README and the tracked issue on
//! horizon-rust/horizon); the core's draw model is ready and exact.
use mod_api::{RegistrationContext, Registrar};

mod runtime;

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl mod_api::Hooks for Mod {
    /// Upstream has no config: nothing to register on the settings page.
    fn register(&self, _registrar: &mut dyn Registrar, _context: &RegistrationContext) {}
}
