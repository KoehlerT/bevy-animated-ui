use std::ops::Div;

use bevy::prelude::*;

pub struct ScrollContainerPlugin;

impl Plugin for ScrollContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (touch_scroll, handle_scroll_event, smooth_scrolling),
        )
        .add_event::<ScrollContainerScrollEvent>();
    }
}

pub enum ScrollDirection {
    VERTICAL,
    HORIZONTAL,
    BOTH,
}

#[derive(Component)]
pub struct ScrollContainer {
    pub direction: ScrollDirection,
    pub last_position: Option<Vec2>,
}

#[derive(Component)]
pub struct ScrollContent {
    current: Vec2,
    max: Vec2,
    velocity: Vec2,
    pub breaking: f32,
}

impl Default for ScrollContent {
    fn default() -> Self {
        Self {
            current: Default::default(),
            max: Default::default(),
            velocity: Default::default(),
            breaking: -5.,
        }
    }
}

#[derive(Event, Debug)]
struct ScrollContainerScrollEvent {
    sender: Entity,
    velocity: Vec2,
}

impl Default for ScrollContainer {
    fn default() -> Self {
        ScrollContainer {
            direction: ScrollDirection::VERTICAL,
            last_position: None,
        }
    }
}

/**
 * Gets mouse input, looks whether it is above a scroll container and sends a corresponding scroll event.
 */
fn touch_scroll(
    mut motion_evr: EventReader<CursorMoved>,
    mut container_query: Query<(Entity, &Node, &GlobalTransform, &mut ScrollContainer)>,
    mut event_writer: EventWriter<ScrollContainerScrollEvent>,
    mouse_button: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    for ev in motion_evr.read() {
        for (entity, node, transform, mut scroll_container) in container_query.iter_mut() {
            let pos = ev.position;
            let halfsize: Vec2 = node.size().div(2.);
            if pos.x > transform.affine().translation.x - halfsize.x
                && pos.x < transform.affine().translation.x + halfsize.x
                && pos.y > transform.affine().translation.y - halfsize.y
                && pos.y < transform.affine().translation.y + halfsize.y
            {
                if mouse_button.pressed(MouseButton::Left) {
                    match scroll_container.last_position {
                        Some(last_position) => {
                            let delta = pos - last_position;
                            let mut velocity = delta / time.delta_seconds();
                            match scroll_container.direction {
                                ScrollDirection::HORIZONTAL => {
                                    velocity.y = 0.;
                                }
                                ScrollDirection::VERTICAL => {
                                    velocity.x = 0.;
                                }
                                ScrollDirection::BOTH => {}
                            }

                            event_writer.send(ScrollContainerScrollEvent {
                                sender: entity,
                                velocity,
                            })
                        }
                        None => {}
                    }
                    scroll_container.last_position = Some(pos);
                } else {
                    // If mouse is not pressed, set the last_position to none, so scrolling does not clip
                    scroll_container.last_position = None;
                }
            }
        }
    }
}

/**
 * Handles scroll events by setting the correct velocity to the content
 */
fn handle_scroll_event(
    mut event_reader: EventReader<ScrollContainerScrollEvent>,
    mut content_query: Query<(&Parent, &mut ScrollContent, &Node)>,
    parent_query: Query<&Node>,
) {
    for event in event_reader.read() {
        for (parent, mut content, node) in content_query.iter_mut() {
            if event.sender == parent.get() {
                if content.velocity.distance(Vec2::ZERO) < 0.000001 {
                    content.velocity = event.velocity;
                } else {
                    content.velocity = content.velocity * 0.5 + event.velocity * 0.5;
                }
				if let Ok(parent_node) = parent_query.get(parent.get()) {
					// maximum values for the left and top properties. Window Size - Content size
					content.max = parent_node.size() - node.size();
				} else {
					warn!("Could not set contents size, because parent node was not found. Scolling may not work!");
				}
            }
        }
    }
}

/**
 * Updates the contents position based of the velocity and a breaking factor
 */
fn smooth_scrolling(mut content_query: Query<(&mut ScrollContent, &mut Style)>, time: Res<Time>) {
    for (mut content, mut style) in content_query.iter_mut() {
        // Run into constraints
        if content.current.y < content.max.y {
            content.current.y = content.max.y
        }
        if content.current.x < content.max.x {
            content.current.x = content.max.x
        }
        if content.current.y > 0. {
            content.current.y = 0.
        }
        if content.current.x > 0. {
            content.current.x = 0.
        }

        style.top = Val::Px(content.current.y);
        style.left = Val::Px(content.current.x);

        let delta = content.velocity * time.delta_seconds();
        content.current += delta;
        let delta_v = content.velocity * content.breaking * time.delta_seconds();
        content.velocity += delta_v;
    }
}
