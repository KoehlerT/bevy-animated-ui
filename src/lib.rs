use bevy::prelude::*;

mod plugin;

#[derive(Component)]
pub struct UiAnimation {
	pub delay: f32,
	pub duration: f32,
}

pub mod prelude {
	pub use super::plugin::*;
	pub use super::UiAnimation;
}