use bevy::prelude::*;

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