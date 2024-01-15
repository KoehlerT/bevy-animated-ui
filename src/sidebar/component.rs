use bevy::prelude::*;
use super::*;
use super::data::*;

pub fn spawn_sidebar(commands: &mut Commands, data: &SidebarData) -> Entity 
{
	let category_buttons = spawn_category_buttons(commands, data);
	let content = spawn_content(commands, data.categories.get(0).unwrap());
	commands.spawn(NodeBundle {
		style: Style {
			position_type: PositionType::Absolute,
			display: Display::Grid,
			width: Val::Px(300.),
			height: Val::Percent(100.),
			grid_template_columns: vec![GridTrack::auto(), GridTrack::px(50.)],
			..default()
		},
		background_color: Color::CRIMSON.into(),
		..default()
	}).insert(SlideAnimation::default())
	.add_child(content)
	.add_child(category_buttons).id()
}

#[derive(Component)]
pub struct CategroryButton(SidebarCategory);

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

fn spawn_content(commands: &mut Commands, category: &SidebarCategory) -> Entity{
	let container = commands.spawn(NodeBundle {
		style : Style {
			display: Display::Grid,
			grid_auto_rows: GridTrack::px(120.),
			justify_content: JustifyContent::Center,
			..default()
		},
		..default()
	}).insert(SidebarContent(category.clone())).id();

	return container;
}

pub fn update_content(
	mut commands: Commands,
	query: Query<(Entity, &SidebarContent, Option<&Children>), Changed<SidebarContent>>
) {
	for (e, content, children) in query.iter() {
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
			let comp = spawn_item(&mut commands, item);
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

fn spawn_item(commands: &mut Commands, item: &SidebarItem) -> Entity{
	commands.spawn(ButtonBundle {
		style: Style {
			display: Display::Grid,
			height: Val::Percent(100.),
			grid_template_rows: vec![GridTrack::percent(80.), GridTrack::percent(20.)],
			justify_content: JustifyContent::Center,
			padding: UiRect::all(Val::Px(5.)),
			..default()
		},
		background_color: Color::NONE.into(),
		..default()
	}).with_children(|builder| {
		// Image
		builder.spawn(ImageBundle {
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
		builder.spawn(TextBundle {
			style: Style {
				height: Val::Percent(100.),
				justify_self: JustifySelf::Center,
				..default()
			},
			text: Text::from_section(item.name.clone(), TextStyle { font: Default::default(), font_size: 22., color: Color::WHITE }),
			..default()
		});
	}).insert(ButtonCropAnimation::default()).id()
}