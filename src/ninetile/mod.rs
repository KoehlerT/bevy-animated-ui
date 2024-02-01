use bevy::prelude::*;

pub mod interactive;

pub fn spawn_ninetile(commands : &mut Commands, handle: Handle<TextureAtlas>, border : Option<f32>, child_builder: impl FnOnce(&mut ChildBuilder)) -> Entity {
	let border = border.unwrap_or(20.);
	let parent = commands.spawn(NodeBundle {
		style: Style {
			display: Display::Grid,
			grid_template_columns: vec![GridTrack::px(border), GridTrack::auto(), GridTrack::px(border)],
			grid_template_rows: vec![GridTrack::px(border), GridTrack::auto(), GridTrack::px(border)],
			margin: UiRect::all(Val::Px(border)), // Workaround because size calculation of this node excludes the border (= content child node)
			..default()
		},
		..default()
	}).id();
	let mut content = parent;
	for i in 0..9 {
		let tile = get_tile(commands, &handle, i);
		commands.entity(parent).add_child(tile);
		if i == 4 {content = tile}
	}
	commands.entity(content).with_children(child_builder);
	return parent;
}

fn get_tile(commands : &mut Commands, handle: &Handle<TextureAtlas>, index: usize) -> Entity {
	commands.spawn(AtlasImageBundle  {
		style: Style { 
			width: Val::Percent(100.),
			height: Val::Percent(100.),
			..default()
		},
		texture_atlas: handle.clone(),
		texture_atlas_image: UiTextureAtlasImage {index, ..Default::default()},
		..default()
	}).id()
}