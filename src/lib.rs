use std::f32::consts::FRAC_PI_4;

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit},
    prelude::*,
};

pub struct ArcballCameraPlugin;

impl Plugin for ArcballCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (zoom, rotate, update).chain());
    }
}

// Based on this converted to Quat
// https://github.com/roy-t/roy-t.nl/blob/master/_posts/2010-02-21-xna-simple-arcballcamera.md
#[derive(Component, Debug)]
#[require(Transform, Camera3d)]
pub struct ArcballCamera {
    /// Whether camera currently responds to user input
    pub enabled: bool,
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

    pub fn rotate_xy(&mut self, x: f32, y: f32) {
        self.rotation *= Quat::from_rotation_x(x) * Quat::from_rotation_y(y);
    }
}

impl Default for ArcballCamera {
    fn default() -> Self {
        Self {
            distance: 1.0,
            enabled: true,
            look_at: Vec3::default(),
            rotation: Quat::default(),
        }
    }
}

fn zoom(arcball_cameras: Query<&mut ArcballCamera>, mouse_scroll: Res<AccumulatedMouseScroll>) {
    for mut arcball in arcball_cameras {
        if !arcball.enabled {
            continue;
        }
        let scroll = match mouse_scroll.unit {
            MouseScrollUnit::Line => mouse_scroll.delta.y * 16.0,
            MouseScrollUnit::Pixel => mouse_scroll.delta.y,
        };
        arcball.distance += scroll / 100.0;
    }
}

fn rotate(
    arcball_cameras: Query<(&mut ArcballCamera, &Camera)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    if !mouse_buttons.pressed(MouseButton::Left) {
        return;
    }

    for (mut arcball, camera) in arcball_cameras {
        if !arcball.enabled {
            continue;
        }

        let delta = mouse_motion.delta;
        if delta != Vec2::ZERO {
            if let Some(viewport_size) = camera.logical_viewport_size() {
                let viewport_size = viewport_size / 2.0;
                let horizontal_angle = (-delta.x / viewport_size.x) * FRAC_PI_4;
                let vertical_angle = (-delta.y / viewport_size.y) * FRAC_PI_4;
                arcball.rotate_xy(vertical_angle, horizontal_angle);
            }
        }
    }
}

fn update(arcball_transforms: Query<(&ArcballCamera, &mut Transform), Changed<ArcballCamera>>) {
    for (arcball, mut transform) in arcball_transforms {
        // Calculate position based on quaternion orientation
        let forward = arcball.rotation * (Vec3::Z * -arcball.distance);

        transform.translation = arcball.look_at + forward;

        // Set rotation so we look at the target point
        let up = arcball.rotation * Vec3::Y;
        transform.look_at(arcball.look_at, up);
    }
}
