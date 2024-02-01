use bevy::prelude::*;
use ui_playground::prelude::*;
use data::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
	App::new()
        .add_plugins(DefaultPlugins)
		.add_plugins(WorldInspectorPlugin::new())
		.add_plugins(SidebarAnimationPlugin)
		.add_plugins(ScrollContainerPlugin)
        .add_systems(Startup, setup)
		.add_systems(Update, toggle_sidebar)
        .run();
}

#[derive(Resource)]
struct SidebarEntity(Entity);

#[derive(Component)]
struct OpenButtonMarker;

fn setup(
	mut commands : Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	commands.spawn(Camera3dBundle::default());
	commands.spawn(ButtonBundle {
		style: Style {
			position_type: PositionType::Absolute, 
			right: Val::Px(0.),
			bottom: Val::Px(0.),
			width: Val::Px(100.),
			height: Val::Px(100.),
			..default()
		},
		background_color: Color::BLUE.into(),
		..default()
	}).insert(OpenButtonMarker);

	let data = SidebarData {
		categories: vec![
			SidebarCategory {
				icon : asset_server.load("sidebar/cloud-solid.png"),
				items: vec![
					SidebarItem {name: "Cat0Item0".into(), image: asset_server.load("sidebar/cloud-solid.png")},
					SidebarItem {name: "Cat0Item1".into(), image: asset_server.load("sidebar/cloud-solid.png")},
					SidebarItem {name: "Cat0Item2".into(), image: asset_server.load("sidebar/cloud-solid.png")},
					SidebarItem {name: "Cat0Item3".into(), image: asset_server.load("sidebar/cloud-solid.png")},
					SidebarItem {name: "Cat0Item4".into(), image: asset_server.load("sidebar/cloud-solid.png")},
					SidebarItem {name: "Cat0Item5".into(), image: asset_server.load("sidebar/cloud-solid.png")},
					SidebarItem {name: "Cat0Item6".into(), image: asset_server.load("sidebar/cloud-solid.png")}
				]
			},
			SidebarCategory {
				icon : asset_server.load("sidebar/house-solid.png"),
				items: vec![
					SidebarItem {name: "Cat1Item0".into(), image: asset_server.load("sidebar/house-solid.png")},
					SidebarItem {name: "Cat1Item1".into(), image: asset_server.load("sidebar/house-solid.png")},
					SidebarItem {name: "Cat1Item2".into(), image: asset_server.load("sidebar/house-solid.png")},
					SidebarItem {name: "Cat1Item3".into(), image: asset_server.load("sidebar/house-solid.png")},
					SidebarItem {name: "Cat1Item4".into(), image: asset_server.load("sidebar/house-solid.png")}
				]
			},
			SidebarCategory {
				icon : asset_server.load("sidebar/star-solid.png"),
				items: vec![
					SidebarItem {name: "Cat2Item0".into(), image: asset_server.load("sidebar/star-solid.png")},
					SidebarItem {name: "Cat2Item1".into(), image: asset_server.load("sidebar/star-solid.png")},
					SidebarItem {name: "Cat2Item2".into(), image: asset_server.load("sidebar/star-solid.png")},
				]
			}
		]
	};
	// style
	let texture_handle = asset_server.load("glassPanel_corners.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(33.3, 33.3), 3, 3, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

	let sidebar_style = SidebarStyle {
		background_content: texture_atlas_handle.clone(),
		scrollable: true,
		background_item: NinetileButton {
			texture_atlas: texture_atlas_handle.clone(),
			border: Some(15.),
			interaction_none: (0..9).collect::<Vec<_>>().try_into().expect("wrong size iterator"),
			interaction_hovered: None,
			interaction_pressed: None
		}
	};
	let sidebar = spawn_sidebar(&mut commands, &data, &sidebar_style);
	commands.insert_resource(SidebarEntity(sidebar));
}

fn toggle_sidebar (
	query : Query<&Interaction, (With<OpenButtonMarker>, Changed<Interaction>)>,
	mut event_writer: EventWriter<SidebarEvent>,
	mut state : Local<bool>,
	sidebar_entity: Res<SidebarEntity>
) {
	for i in query.iter() {
		match i {
			Interaction::Pressed => {
				if *state {
					event_writer.send(SidebarEvent::CLOSE(sidebar_entity.0));
					*state = false;
				} else {
					event_writer.send(SidebarEvent::OPEN(sidebar_entity.0));
					*state = true;
				}
			},
			Interaction::Hovered => {},
			Interaction::None => {}
		}
	}
}