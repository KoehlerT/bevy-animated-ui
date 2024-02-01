use bevy::prelude::*;
use ui_playground::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
	App::new()
        .add_plugins(DefaultPlugins)
		.add_plugins(WorldInspectorPlugin::new())
		.add_plugins(NinetileButtonPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
	mut commands : Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>
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

	let parent = spawn_ninetile(&mut commands, texture_atlas_handle, Some(15.), 
	|parent|{
		parent.spawn(TextBundle::from_sections([
			TextSection::new("This is content!".to_string(), text_style.clone())
		]));
	});
	commands.entity(container).add_child(parent);

	// Make interactive button
	let interactive_button = make_interactive_button(&mut commands, &asset_server, &mut texture_atlases);
	commands.entity(container).add_child(interactive_button);
}

fn make_interactive_button(commands : &mut Commands,
	asset_server: &Res<AssetServer>,
	texture_atlases: &mut ResMut<Assets<TextureAtlas>>,) -> Entity
{
	let texture_handle = asset_server.load("glassPanel_corners_button.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(33.3, 33.3), 3, 9, None, None);
	let texture_atlas_handle = texture_atlases.add(texture_atlas);

	let button_descriptor = interactive::NinetileButton {
		texture_atlas: texture_atlas_handle.clone(),
		interaction_none: (0..9).collect::<Vec<_>>().try_into().expect("wrong size iterator"),
		interaction_hovered: Some((9..18).collect::<Vec<_>>().try_into().expect("wrong size iterator")),
		interaction_pressed: Some((18..27).collect::<Vec<_>>().try_into().expect("wrong size iterator")),
		border: None
	};

	let button = create_ninetile_button(commands, &button_descriptor, |parent| {
		parent.spawn(TextBundle::from_sections([
			TextSection::new("This is Buttoncontent!".to_string(), TextStyle {
				font_size: 20.,
				..default()
			})
		]));
	});
	
	return button;
}