use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

const CAM_SPEED: f32 = 3.;
const CAM_SCROLL_SPEED: f32 = 10.;
const CAM_ROTATIONAL_SPEED: f32 = 0.1;

pub fn move_camera(
    mut cam_q: Query<&mut Transform, With<Camera3d>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut mouse_ev: MessageReader<MouseMotion>,
    mut mouse_scroll: MessageReader<MouseWheel>,
    time: Res<Time>,
) {
    let mut cam = cam_q.iter_mut().next().expect("Camera not found");

    if keyboard_input.pressed(KeyCode::KeyD) {
        let dir = cam.right();
        cam.translation += dir * time.delta_secs() * CAM_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        let dir = cam.left();
        cam.translation += dir * time.delta_secs() * CAM_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        let dir = cam.left().cross(Vec3::Y);
        cam.translation += dir * time.delta_secs() * CAM_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        let dir = cam.right().cross(Vec3::Y);
        cam.translation += dir * time.delta_secs() * CAM_SPEED;
    }

    for ev in mouse_ev.read() {
        let delta_x = ev.delta.x;
        let delta_y = ev.delta.y;
        let right = cam.right();

        if mouse_input.pressed(MouseButton::Middle) {
            cam.rotate_axis(
                Dir3::from_xyz(0., 1., 0.).unwrap(),
                -delta_x * time.delta_secs() * CAM_ROTATIONAL_SPEED,
            );
            cam.rotate_axis(right, -delta_y * time.delta_secs() * CAM_ROTATIONAL_SPEED);
        }
    }

    for ev in mouse_scroll.read() {
        let dir = cam.forward();
        cam.translation += dir * time.delta_secs() * CAM_SCROLL_SPEED * ev.y;
    }
}
