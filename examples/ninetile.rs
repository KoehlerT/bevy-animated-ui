use bevy::prelude::*;
use ui_playground::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
	App::new()
        .add_plugins(DefaultPlugins)
		.add_plugins(WorldInspectorPlugin::new())
		.add_plugins(ButtonAnimationPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
	mut commands : Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	commands.spawn(Camera3dBundle::default());
	let text_style = TextStyle {
        font_size: 20.,
        ..default()
    };

	// Main Continer
	let container = commands.spawn(NodeBundle {
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
	}).id();
	
	// Make Texture Atlas
	let texture_handle = asset_server.load("glassPanel_corners.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(33.3, 33.3), 3, 3, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

	let (parent, content) = spawn_ninetile(&mut commands, texture_atlas_handle, Some(15.));
	commands.entity(container).add_child(parent);

	let text = commands.spawn(TextBundle::from_sections([
		TextSection::new("This is content!".to_string(), text_style.clone())
	])).id();
	commands.entity(content).add_child(text);
}