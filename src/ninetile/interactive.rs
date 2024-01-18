use bevy::prelude::*;

pub struct NinetileButtonPlugin;

impl Plugin for NinetileButtonPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, update_ninetile_image);
	}
}

#[derive(Clone)]
pub struct NinetileButton {
	pub texture_atlas: Handle<TextureAtlas>,
	pub interaction_none: [usize; 9],
	pub interaction_hovered: Option<[usize; 9]>,
	pub interaction_pressed: Option<[usize; 9]>,
	pub border: Option<f32>
}

#[derive(Component)]
struct NinetileButtonComponent {
	descriptor: NinetileButton,
	children: [Entity; 9]
}

pub fn create_ninetile_button(commands :&mut Commands, button_descr: &NinetileButton) -> (Entity, Entity){
	let border = button_descr.border.unwrap_or(20.);

	let button = commands.spawn(ButtonBundle {
		style: Style {
			display: Display::Grid,
			grid_template_columns: vec![GridTrack::px(border), GridTrack::auto(), GridTrack::px(border)],
			grid_template_rows: vec![GridTrack::px(border), GridTrack::auto(), GridTrack::px(border)],
			margin: UiRect::all(Val::Px(border)),
			..default()
		},
		background_color: Color::NONE.into(),
		..default()
	}).id();

	let mut images : Vec<Entity> = vec![];
	for i in 0..9 {
		let tile = get_tile(commands, &button_descr.texture_atlas, button_descr.interaction_none[i]);
		commands.entity(button).add_child(tile);
		images.push(tile);
	}

	commands.entity(button).insert(NinetileButtonComponent {
		descriptor: button_descr.clone(),
		children: [images[0], images[1],images[2],images[3],images[4], images[5],images[6],images[7],images[8]]
	});

	return (button, images[4]); //Parent, Content
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

fn update_ninetile_image(
	query: Query<(&NinetileButtonComponent, &Interaction), Changed<Interaction>>,
	mut image_query: Query<(Entity, &mut UiTextureAtlasImage)>
) {
	for (button, interaction) in query.iter() {
		match interaction {
			Interaction::None => {
				update_image(&mut image_query, &button.children, &button.descriptor.interaction_none)
			},
			Interaction::Hovered => {
				match button.descriptor.interaction_hovered {
					Some(indices) => {
						update_image(&mut image_query, &button.children, &indices);
					},
					None => {}
				}
			},
			Interaction::Pressed => {
				match button.descriptor.interaction_pressed {
					Some(indices) => {
						update_image(&mut image_query, &button.children, &indices);
					},
					None => {}
				}
			}
		}
	}
}

fn update_image(image_query: &mut Query<(Entity, &mut UiTextureAtlasImage)>, children: &[Entity;9], indices: &[usize; 9]) 
{
	for i in 0..9 {
		match image_query.get_mut(children[i]) {
			Ok((_, mut atlas_image)) => {
				atlas_image.index = indices[i];
			},
			Err(_) => {warn!("Could not update interactive Button!")}
		}
	}
}