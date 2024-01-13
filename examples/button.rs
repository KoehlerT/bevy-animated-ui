use bevy::prelude::*;
use ui_playground::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
	App::new()
        .add_plugins(DefaultPlugins)
		.add_plugins(WorldInspectorPlugin::new())
		.add_plugins(UiAnimationPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
	mut commands : Commands,
	asset_server: Res<AssetServer>
) {
	commands.spawn(Camera3dBundle::default());
	let button = build_button(&mut commands);
	let btn2 = button_xmark(&mut commands, &asset_server);

	// Main Container
	commands.spawn(NodeBundle {
		style: Style {
			width: Val::Percent(100.),
			height: Val::Percent(100.),
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,
			flex_direction: FlexDirection::Column,
			padding: UiRect::all(Val::Px(15.)),
			row_gap: Val::Px(15.),
			..default()
		},
		..default()
	}).add_child(button)
	.add_child(btn2);
	
}

fn build_button(
	commands: &mut Commands
) -> Entity {
	commands.spawn(NodeBundle {
		style: Style {
			width: Val::Px(100.),
			height: Val::Px(50.),
			align_items: AlignItems::Center,
			justify_content: JustifyContent::Center,
			..default()
		},
		background_color : Color::BLUE.into(),
		..default()
	}).with_children(|builder| {
		builder.spawn(ButtonBundle {
			style: Style {
				width: Val::Percent(100.),
				height: Val::Percent(100.),
				..default()
			},
			background_color: Color::BEIGE.into(),
			..default()
		}).insert(ButtonCropAnimation::default());
	}).id()
}

fn button_xmark(
	commands: &mut Commands,
	asset_server: &Res<AssetServer>
) -> Entity {
	commands.spawn(NodeBundle {
		style: Style {
			width: Val::Px(100.),
			height: Val::Px(100.),
			align_items: AlignItems::Center,
			justify_content: JustifyContent::Center,
			..default()
		},
		background_color : Color::WHITE.into(),
		..default()
	}).with_children(|builder| {
		builder.spawn(ButtonBundle {
			style: Style {
				width: Val::Percent(100.),
				height: Val::Percent(100.),
				..default()
			},
			image: UiImage { texture: asset_server.load("circle-xmark-regular.png"), ..default() },
			..default()
		}).insert(ButtonCropAnimation::default());
	}).id()
}