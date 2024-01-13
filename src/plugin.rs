use std::time::Duration;

use bevy::prelude::*;

pub struct UiAnimationPlugin;

impl Plugin for UiAnimationPlugin {
	fn build(&self, app : &mut App) {
		app.add_systems(Update, (animate, play_animation));
	}
}

#[derive(Component)]
pub struct ButtonCropAnimation {
	duration: Duration,
	delay: Duration,
	goal: f32,
	start: f32,
	current: f32,
	playing: bool
}

impl Default for ButtonCropAnimation {
    fn default() -> Self {
        Self { 
			duration: Duration::from_secs_f32(1.), 
			delay: Duration::ZERO, 
			goal: 0., 
			start: 0., 
			current: 100., 
			playing: false
		}
    }
}

fn animate(
	mut animation_query : Query<(&Interaction, &mut ButtonCropAnimation), Changed<Interaction>>
) {
	for (interaction, mut animation) in animation_query.iter_mut() {
		if *interaction == Interaction::Pressed {
			animation.goal = 90.;
			animation.start = animation.current;
			animation.delay = Duration::ZERO;
			animation.duration = Duration::from_secs_f32(0.1);
			animation.playing = true;
		}
		else if *interaction != Interaction::Pressed {
			animation.goal = 100.;
			animation.start = animation.current;
			animation.delay = Duration::ZERO;
			animation.duration = Duration::from_secs_f32(0.1);
			animation.playing = true;
		}
	}
}

fn play_animation(
	mut animation_query: Query<(&mut Style, &mut ButtonCropAnimation)>,
	time: Res<Time>
) {
	for (mut style, mut animation) in animation_query.iter_mut() {
		if animation.playing {
			animation.current = ((animation.goal - animation.start) / animation.duration.as_secs_f32()) * animation.delay.as_secs_f32() + animation.start;
			style.width = Val::Percent(animation.current);
			style.height = Val::Percent(animation.current);
			animation.delay += time.delta();
			if animation.delay >= animation.duration {
				animation.playing = false
			}
		}
	}
}