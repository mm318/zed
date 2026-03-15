//! Blade rendering backend for GPUI.

#[cfg(target_os = "macos")]
mod apple_compat;
mod blade_atlas;
mod blade_context;
mod blade_renderer;

#[cfg(target_os = "macos")]
pub use apple_compat::*;
pub(crate) use blade_atlas::BladeAtlas;
pub use blade_context::*;
pub use blade_renderer::*;

pub use blade_graphics;
