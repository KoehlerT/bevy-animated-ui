use bevy::prelude::*;
use ui_playground::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
	App::new()
        .add_plugins(DefaultPlugins)
		.add_plugins(WorldInspectorPlugin::new())
		.add_plugins(ScrollContainerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
	mut commands : Commands,
) {
	commands.spawn(Camera3dBundle::default());
	let vertical_scroll = spawn_vertical_scroll(&mut commands);
	let horizontal_scroll = spawn_horizontal_scroll(&mut commands);
	let both_scroll = spawn_both_scroll(&mut commands);

	// Main Container
	commands.spawn(NodeBundle {
		style: Style {
			width: Val::Percent(100.),
			height: Val::Percent(100.),
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,
			flex_direction: FlexDirection::Row,
			padding: UiRect::all(Val::Px(15.)),
			column_gap: Val::Px(15.),
			..default()
		},
		..default()
	})
	.add_child(vertical_scroll)
	.add_child(horizontal_scroll)
	.add_child(both_scroll);
	
}

/**
 * Spawns a horiziontal scroll container. Always spawn a 
 * "Window Node" with the ScrollContainer component
 * "Content Node" with the ScrollContent component
 * the actual content as children of the "Content Node"
 */
fn spawn_horizontal_scroll (commands: &mut Commands) -> Entity {
	let parent = commands.spawn(NodeBundle {
		style: Style {
			flex_direction: FlexDirection::Column,
			height: Val::Percent(50.),
			width: Val::Px(300.),
			overflow: Overflow::clip_x(),
			..default()
		},
		..default()
	}).insert(ScrollContainer{direction: ScrollDirection::HORIZONTAL, ..default()}).id();

	let content = commands.spawn(NodeBundle {
		style: Style {
			width: Val::Px(100. * 60.), // setting width explicitly because it does not calculate correctly...
			height: Val::Px(60.),
			flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
			..default()
		},
		background_color: Color::GRAY.into(),
		..default()
	}).insert(ScrollContent::default()).id();

	commands.entity(parent).add_child(content);

	commands.entity(content).with_children(|parent| {
		// List items
		for i in 0..60 {
			parent.spawn(TextBundle {
				style: Style {
					width: Val::Px(100.),
					..default()
				},
				text: Text::from_section(
					format!("Item {i}"),
					TextStyle {
						font: Default::default(),
						font_size: 20.,
						..default()
					},
				),
				..default()
			});
		}
	});

	return parent;
}

fn spawn_vertical_scroll (commands: &mut Commands) -> Entity {
	let parent = commands.spawn(NodeBundle {
		style: Style {
			flex_direction: FlexDirection::Column,
			height: Val::Percent(50.),
			width: Val::Px(300.),
			overflow: Overflow::clip_y(),
			..default()
		},
		..default()
	}).insert(ScrollContainer::default()).id();

	let content = commands.spawn(NodeBundle {
		style: Style {
			flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
			..default()
		},
		background_color: Color::GRAY.into(),
		..default()
	}).insert(ScrollContent::default()).id();

	commands.entity(parent).add_child(content);

	commands.entity(content).with_children(|parent| {
		// List items
		for i in 0..60 {
			parent.spawn((
				TextBundle::from_section(
					format!("Item {i}"),
					TextStyle {
						font: Default::default(),
						font_size: 20.,
						..default()
					},
				),
				Label,
			));
		}
	});

	return parent;
}

fn spawn_both_scroll (commands: &mut Commands) -> Entity {
	let parent = commands.spawn(NodeBundle {
		style: Style {
			flex_direction: FlexDirection::Column,
			height: Val::Percent(50.),
			width: Val::Px(300.),
			overflow: Overflow::clip(),
			..default()
		},
		..default()
	}).insert(ScrollContainer{direction: ScrollDirection::BOTH, ..default()}).id();

	let content = commands.spawn(NodeBundle {
		style: Style {
			width: Val::Px(100. * 10.), // setting width explicitly because it does not calculate correctly...
			height: Val::Px(50. * 10.),
			display: Display::Grid,
			grid_template_rows: RepeatedGridTrack::px(10, 50.),
			grid_template_columns: RepeatedGridTrack::px(10, 100.),
			..default()
		},
		background_color: Color::GRAY.into(),
		..default()
	}).insert(ScrollContent::default()).id();

	commands.entity(parent).add_child(content);

	commands.entity(content).with_children(|parent| {
		// List items
		for i in 0..100 {
			parent.spawn(TextBundle {
				style: Style {
					..default()
				},
				text: Text::from_section(
					format!("Item {i}"),
					TextStyle {
						font: Default::default(),
						font_size: 20.,
						..default()
					},
				),
				..default()
			});
		}
	});

	return parent;
}