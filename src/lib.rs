use std::f32::consts::FRAC_PI_4;

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit},
    prelude::*,
};

pub struct ArcballCameraPlugin;

impl Plugin for ArcballCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AccumulatedTouchMotion>()
            .init_resource::<AccumulatedTouchPinch>()
            .add_systems(Startup, update)
            .add_systems(PreUpdate, accumulate_touches)
            .add_systems(Update, (zoom, rotate, update).chain());
    }
}

#[derive(Default, Resource)]
struct AccumulatedTouchMotion {
    delta: Vec2,
}

#[derive(Default, Resource)]
struct AccumulatedTouchPinch {
    distance: f32,
}

// Based on this converted to Quat
// https://github.com/roy-t/roy-t.nl/blob/master/_posts/2010-02-21-xna-simple-arcballcamera.md
#[derive(Component, Debug)]
#[require(Transform, Camera3d)]
pub struct ArcballCamera {
    /// Whether camera zoom responds to user input
    pub zoom_enabled: bool,
    /// Whether camera orbit responds to user input
    pub orbit_enabled: bool,
    pub look_at: Vec3,
    pub distance: f32,
    rotation: Quat,
}

impl ArcballCamera {
    pub fn new(distance: f32) -> Self {
        Self {
            distance,
            ..default()
        }
    }

    /// Rotate to a position on a sphere `distance` from `look_at`
    /// that is `x` radians rotated around X axis, and `y` radians around Y axis.
    pub fn rotate_xy(&mut self, x: f32, y: f32) {
        self.rotation *= Quat::from_rotation_x(x) * Quat::from_rotation_y(y);
    }
}

impl Default for ArcballCamera {
    fn default() -> Self {
        Self {
            distance: 1.0,
            zoom_enabled: true,
            orbit_enabled: true,
            look_at: Vec3::default(),
            rotation: Quat::default(),
        }
    }
}

fn accumulate_touches(
    touches: Res<Touches>,
    mut accumulated_touch_motion: ResMut<AccumulatedTouchMotion>,
    mut accumulated_touch_pinch: ResMut<AccumulatedTouchPinch>,
) {
    let mut touches_iter = touches.iter();
    let touch_sequence = (
        touches_iter.next(),
        touches_iter.next(),
        touches_iter.next(),
    );
    match touch_sequence {
        (Some(touch), None, None) => {
            accumulated_touch_pinch.distance = 0.0;
            accumulated_touch_motion.delta = touch.delta();
        }
        (Some(touch1), Some(touch2), None) => {
            accumulated_touch_motion.delta = Vec2::ZERO;
            accumulated_touch_pinch.distance = touch1.position().distance(touch2.position())
                - touch1
                    .previous_position()
                    .distance(touch2.previous_position());
        }
        _ => {
            accumulated_touch_motion.delta = Vec2::ZERO;
            accumulated_touch_pinch.distance = 0.0;
        }
    }
}

fn zoom(
    arcball_cameras: Query<(&mut ArcballCamera, &Camera)>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    touch_pinch: Res<AccumulatedTouchPinch>,
) {
    let distance = match mouse_scroll.unit {
        MouseScrollUnit::Line => mouse_scroll.delta.y * 16.0,
        MouseScrollUnit::Pixel => mouse_scroll.delta.y,
    } + touch_pinch.distance;
    if distance == 0.0 {
        return;
    }

    for (mut arcball, camera) in arcball_cameras {
        if !arcball.zoom_enabled {
            continue;
        }
        if let Some(viewport_size) = camera.logical_viewport_size() {
            arcball.distance =
                (arcball.distance - distance / (viewport_size.length() / 2.0)).max(0.0);
        }
    }
}

fn rotate(
    arcball_cameras: Query<(&mut ArcballCamera, &Camera)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    touch_motion: Res<AccumulatedTouchMotion>,
) {
    let mouse_delta = if mouse_buttons.pressed(MouseButton::Left) {
        mouse_motion.delta
    } else {
        Vec2::ZERO
    };
    let delta = mouse_delta + touch_motion.delta;
    if delta == Vec2::ZERO {
        return;
    }
    for (mut arcball, camera) in arcball_cameras {
        if !arcball.orbit_enabled {
            continue;
        }

        if let Some(viewport_size) = camera.logical_viewport_size() {
            let viewport_size = viewport_size / 2.0;
            let horizontal_angle = (-delta.x / viewport_size.x) * FRAC_PI_4;
            let vertical_angle = (-delta.y / viewport_size.y) * FRAC_PI_4;
            arcball.rotate_xy(vertical_angle, horizontal_angle);
        }
    }
}

fn update(arcball_transforms: Query<(&ArcballCamera, &mut Transform), Changed<ArcballCamera>>) {
    for (arcball, mut transform) in arcball_transforms {
        // Calculate position based on quaternion orientation
        let forward = arcball.rotation * (Vec3::Z * arcball.distance);

        transform.translation = arcball.look_at + forward;

        // Set rotation so we look at the target point
        let up = arcball.rotation * Vec3::Y;
        transform.look_at(arcball.look_at, up);
    }
}
