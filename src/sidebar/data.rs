use bevy::prelude::*;

use crate::prelude::NinetileButton;

#[derive(Clone)]
pub struct SidebarItem {
	pub name: String,
	pub image: Handle<Image>
}
#[derive(Clone)]
pub struct SidebarCategory {
	pub icon: Handle<Image>,
	pub items: Vec<SidebarItem>
}
#[derive(Clone)]
pub struct SidebarData {
	pub categories: Vec<SidebarCategory>
}

#[derive(Clone, Component)]
pub struct SidebarStyle {
	pub background_content: Handle<TextureAtlas>,
	pub scrollable: bool,
	pub background_item: NinetileButton
}