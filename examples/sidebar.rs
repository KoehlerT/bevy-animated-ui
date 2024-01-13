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
	// asset_server: Res<AssetServer>
) {
	commands.spawn(Camera3dBundle::default());

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
	});
	
}