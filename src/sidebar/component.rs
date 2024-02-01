use bevy::prelude::*;
use crate::prelude::{create_ninetile_button, spawn_ninetile, NinetileButton, ScrollContainer, ScrollContent};

use super::*;
use super::data::*;

pub fn spawn_sidebar(commands: &mut Commands, data: &SidebarData, style: &SidebarStyle) -> Entity 
{
	let category_buttons = spawn_category_buttons(commands, data);
	let content = spawn_content(commands, data.categories.get(0).unwrap(), style);
	commands.spawn(NodeBundle {
		style: Style {
			position_type: PositionType::Absolute,
			display: Display::Grid,
			width: Val::Px(300.),
			height: Val::Percent(100.),
			grid_template_columns: vec![GridTrack::px(250.), GridTrack::px(50.)],
			..default()
		},
		..default()
	}).insert(SlideAnimation::default())
	.add_child(content)
	.add_child(category_buttons).id()
}

#[derive(Component)]
pub struct CategroryButton(SidebarCategory);

enum ParentDimension {
	WIDTH, HEIGHT
}

#[derive(Component)]
pub struct FixToParentDimension {
	dimension: ParentDimension
}

fn spawn_category_buttons (commands: &mut Commands, data: &SidebarData) -> Entity {
	commands.spawn(NodeBundle {
		style: Style {
			display: Display::Grid,
			padding: UiRect::all(Val::Px(15.)),
			grid_template_rows: vec![RepeatedGridTrack::px(GridTrackRepetition::Count(data.categories.len().try_into().unwrap()), 50.), RepeatedGridTrack::auto(1)],
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,
			..default()
		},
		..default()
	}).with_children(|b| {
		for cat in &data.categories {
			b.spawn(ButtonBundle {
				style: Style {
					width: Val::Percent(100.),
					aspect_ratio: Some(1.),
					..default()
				},
				background_color: Color::GREEN.into(),
				image: UiImage {texture: cat.icon.clone(), ..default()},
				..default()
			})
			.insert(ButtonCropAnimation::default())
			.insert(CategroryButton(cat.clone()));
		};
	}).id()
}

#[derive(Component)]
pub struct SidebarContent(SidebarCategory);

fn spawn_content(commands: &mut Commands, category: &SidebarCategory, style: &SidebarStyle) -> Entity {
	let container = spawn_ninetile(commands, style.background_content.clone(), Some(15.), |parent|{
		parent.spawn(NodeBundle {
			style: Style {
				flex_direction: FlexDirection::Column,
				width: Val::Percent(100.),
				height: Val::Percent(100.), // This takes the height of the Scroll Container, not the parent therefore the annoying component (FixToParentDimension) below
				justify_self: JustifySelf::Stretch,
				overflow: Overflow::clip_y(),
				..default()
			},
			..default()
		}).insert(ScrollContainer::default())
		.insert(FixToParentDimension {dimension: ParentDimension::HEIGHT})
		.with_children(|parent| {
			parent.spawn(NodeBundle {
				style: Style {
					width: Val::Percent(100.),
					height: Val::Px(2000.), // Debugging
					display: Display::Grid,
					grid_auto_rows: GridTrack::px(120.),
					padding: UiRect { left: Val::Px(0.), right: Val::Px(50.), top: Val::Px(0.), bottom: Val::Px(0.) }, // Workaround
					justify_content: JustifyContent::Center,
					..default()
				},
				..default()
			}).insert(SidebarContent(category.clone()))
			.insert(style.clone())
			.insert(ScrollContent::default());
		});
	});
	return container;
}

pub fn update_content(
	mut commands: Commands,
	query: Query<(Entity,&SidebarStyle, &SidebarContent, Option<&Children>), Changed<SidebarContent>>
) {
	for (e, style, content, children) in query.iter() {
		info!("Update Content");
		match children {
			Some(children) => {
				for child in children {
					commands.entity(*child).despawn_recursive();
				}
			}
			None => {}
		}
		for item in &content.0.items {
			let comp = spawn_item(&mut commands, item, &style.background_item);
			commands.entity(e).add_child(comp);
		}
	}
}

pub fn change_category(
	button_query: Query<(&CategroryButton, &Interaction), (With<Button>, Changed<Interaction>)>,
	mut content_query: Query<(Entity, &mut SidebarContent)>
) {
	for (category_button, interaction) in button_query.iter() {
		match interaction {
			Interaction::Pressed => {
				for (_e, mut content) in content_query.iter_mut() {
					info!("Set new category");
					content.0 = category_button.0.clone();
				}
			}
			Interaction::Hovered => {}
			Interaction::None => {}
		}
	}
}

fn spawn_item(commands: &mut Commands, item: &SidebarItem, ninetilebutton: &NinetileButton) -> Entity{
	let btn = create_ninetile_button(commands, ninetilebutton, |builder| {
		// Node with top and bottom sections
		builder.spawn(NodeBundle {
			style: Style {
				display: Display::Grid,
				height: Val::Percent(100.),
				grid_template_rows: vec![GridTrack::percent(80.), GridTrack::percent(20.)],
				justify_content: JustifyContent::Center,
				padding: UiRect::all(Val::Px(5.)),
				..default()
			},
			..default()
		}).with_children(|parent| {
			// Image
			parent.spawn(ImageBundle {
				style: Style {
					height: Val::Percent(100.),
					aspect_ratio: Some(1.),
					justify_self: JustifySelf::Center,
					..default()
				},
				image: UiImage { texture: item.image.clone(), ..default() },
				..default()
			});
			// Text
			parent.spawn(TextBundle {
				style: Style {
					height: Val::Percent(100.),
					justify_self: JustifySelf::Center,
					..default()
				},
				text: Text::from_section(item.name.clone(), TextStyle { font: Default::default(), font_size: 22., color: Color::WHITE }),
				..default()
			});
		});
	});
	commands.entity(btn).insert(ButtonCropAnimation::default());

	return btn;
}

pub fn handle_fix_to_parent_dimension(
	mut q_entities: Query<(&mut Style, &FixToParentDimension, &Parent)>,
	q_parents: Query<&Node, With<Children>>,
	window: Query<&Window>
) {
	let window = window.get_single().unwrap();
	for (mut style, fix, parent) in q_entities.iter_mut() {
		match q_parents.get(**parent) {
			Ok(node) => {
				match fix.dimension {
					ParentDimension::HEIGHT => {style.height = Val::Px(window.resolution.height())},
					ParentDimension::WIDTH => {style.width = Val::Px(node.size().x)}
				}
			},
			Err(_)=>{}
		}
	}
}