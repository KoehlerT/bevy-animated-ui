use std::time::Duration;

use bevy::prelude::*;
use super::button::*;

pub mod data;
pub mod component;

pub struct SidebarAnimationPlugin;
impl Plugin for SidebarAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (react_to_sidebar, play_animation, component::update_content, component::change_category, component::handle_fix_to_parent_dimension))
			.add_plugins(ButtonAnimationPlugin)
			.add_event::<SidebarEvent>();
    }
}

#[derive(Component)]
struct SlideAnimation {
	duration: Duration,
	delay: Duration,
	goal: f32,
	start: f32,
	current: f32,
	playing: bool
}

impl Default for SlideAnimation {
    fn default() -> Self {
        Self { duration: Default::default(), delay: Default::default(), goal: Default::default(), start: Default::default(), current: Default::default(), playing: Default::default() }
    }
}

#[derive(Event)]
pub enum SidebarEvent {OPEN(Entity), CLOSE(Entity)}

fn react_to_sidebar(
	mut event_reader: EventReader<SidebarEvent>,
	mut query: Query<(Entity, &mut SlideAnimation)>
) {
	for event in event_reader.read() {
		match event {
			SidebarEvent::OPEN(entity) => {
				match query.get_mut(*entity) {
					Ok((_, mut slide_animation)) => {
						slide_animation.goal = 0.;
						slide_animation.start = slide_animation.current;
						slide_animation.delay = Duration::ZERO;
						slide_animation.duration = Duration::from_secs_f32(0.3);
						slide_animation.playing = true;
					},
					Err(_) => {}
				}
			}
			SidebarEvent::CLOSE(entity) => {
				match query.get_mut(*entity) {
					Ok((_, mut slide_animation)) => {
						slide_animation.goal = -300.;
						slide_animation.start = slide_animation.current;
						slide_animation.delay = Duration::ZERO;
						slide_animation.duration = Duration::from_secs_f32(0.3);
						slide_animation.playing = true;
					},
					Err(_) => {}
				}
			}
		}
	}
}

fn play_animation(
	mut animation_query: Query<(&mut Style, &mut SlideAnimation)>,
	time: Res<Time>
) {
	for (mut style, mut animation) in animation_query.iter_mut() {
		if animation.playing {
			animation.current = ((animation.goal - animation.start) / animation.duration.as_secs_f32()) * animation.delay.as_secs_f32() + animation.start;
			style.left = Val::Px(animation.current);
			animation.delay += time.delta();
			if animation.delay > animation.duration {
				animation.playing = false;
				style.left = Val::Px(animation.goal);
			}
		}
	}
}