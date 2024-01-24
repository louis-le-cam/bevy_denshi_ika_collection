use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    math::vec2,
    prelude::{
        Component, EulerRot, EventReader, Input, KeyCode, MouseButton, Quat, Query, Reflect, Res,
        Time, Transform, Vec2, Vec3,
    },
};
use bevy_denshi_ika_gen_plugin::gen_plugin;

gen_plugin! {
    pub(super) FlyCameraControllerPlugin;
    reflect(
        FlyCameraController,
        FlyCameraInputs,
        FlyCameraInput,
        FlyCameraAction
    );
    systems(Update)(fly_camera_controller);
}

fn fly_camera_controller(
    mut cameras: Query<(&mut FlyCameraController, &mut Transform)>,
    time: Res<Time>,
    keycodes: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut mouse_deltas: EventReader<MouseMotion>,
    mut wheel_deltas: EventReader<MouseWheel>,
) {
    let mouse_delta = mouse_deltas.read().map(|delta| delta.delta).sum::<Vec2>();

    let wheel_delta = wheel_deltas
        .read()
        .map(|wheel| vec2(wheel.x, wheel.y))
        .sum::<Vec2>();

    for (mut controller, mut transform) in &mut cameras {
        for (input, action) in controller.inputs.0.clone() {
            let mut apply_delta = true;

            let Some(sum) = input
                .into_iter()
                .map(|input| match input {
                    FlyCameraInput::KeyCode {
                        keycode,
                        first_frame,
                    } => match first_frame {
                        true if keycodes.just_pressed(keycode) => {
                            apply_delta = false;
                            Some(1.0)
                        }
                        false if keycodes.pressed(keycode) => Some(1.0),
                        _ => None,
                    },
                    FlyCameraInput::MouseButton {
                        mouse_button,
                        first_frame,
                    } => match first_frame {
                        true if buttons.just_pressed(mouse_button) => {
                            apply_delta = false;
                            Some(1.0)
                        }
                        false if buttons.pressed(mouse_button) => Some(1.0),
                        _ => None,
                    },
                    FlyCameraInput::MouseMoveX => Some(mouse_delta.x),
                    FlyCameraInput::MouseMoveY => Some(mouse_delta.y),
                    FlyCameraInput::ScrollX => Some(wheel_delta.x),
                    FlyCameraInput::ScrollY => Some(wheel_delta.y),
                })
                .product::<Option<f32>>()
            else {
                continue;
            };

            let speed = sum
                * controller.speed
                * if apply_delta {
                    time.delta_seconds()
                } else {
                    1.0
                };

            match action {
                FlyCameraAction::MoveLocal(x) => {
                    transform.translation = transform.translation + transform.rotation * x * speed
                }
                FlyCameraAction::MoveGlobal(x) => transform.translation += x * speed,
                FlyCameraAction::RotateEuler(x) => {
                    let euler = Vec3::from(transform.rotation.to_euler(EulerRot::default()));
                    let new_euler = euler + x * sum;
                    transform.rotation = Quat::from_euler(
                        EulerRot::default(),
                        new_euler.x,
                        new_euler.y,
                        new_euler.z,
                    );
                }
                FlyCameraAction::ChangeSpeed(x) => {
                    controller.speed = (controller.speed.ln() + x * sum).exp()
                }
                FlyCameraAction::SetSpeed(x) => controller.speed = x * sum,
            }
        }
    }
}

#[derive(Component, Reflect, Clone, Debug)]
pub struct FlyCameraController {
    pub speed: f32,
    pub inputs: FlyCameraInputs,
}

#[derive(Reflect, Clone, Debug)]
pub struct FlyCameraInputs(pub Vec<(Vec<FlyCameraInput>, FlyCameraAction)>);

impl Default for FlyCameraInputs {
    fn default() -> Self {
        macro_rules! inputs {
            [$([$($input:expr),* $(,)?] => $action:expr),* $(,)?] => {
                vec![
                    $((vec![$($input),*], $action)),*
                ]
            };
        }

        fn keycode(keycode: KeyCode, first_frame: bool) -> FlyCameraInput {
            FlyCameraInput::KeyCode {
                keycode,
                first_frame,
            }
        }

        fn button(mouse_button: MouseButton, first_frame: bool) -> FlyCameraInput {
            FlyCameraInput::MouseButton {
                mouse_button,
                first_frame,
            }
        }

        Self(inputs![
            [keycode(KeyCode::Z, false)] => FlyCameraAction::MoveLocal(Vec3::NEG_Z),
            [keycode(KeyCode::S, false)] => FlyCameraAction::MoveLocal(Vec3::Z),
            [keycode(KeyCode::Q, false)] => FlyCameraAction::MoveLocal(Vec3::NEG_X),
            [keycode(KeyCode::D, false)] => FlyCameraAction::MoveLocal(Vec3::X),
            [keycode(KeyCode::ControlLeft, false)] => FlyCameraAction::MoveLocal(Vec3::NEG_Y),
            [keycode(KeyCode::Space, false)] => FlyCameraAction::MoveLocal(Vec3::Y),

            [keycode(KeyCode::Z, false), keycode(KeyCode::ShiftLeft, false)] => FlyCameraAction::MoveLocal(Vec3::NEG_Z * 2.0),
            [keycode(KeyCode::S, false), keycode(KeyCode::ShiftLeft, false)] => FlyCameraAction::MoveLocal(Vec3::Z * 2.0),
            [keycode(KeyCode::Q, false), keycode(KeyCode::ShiftLeft, false)] => FlyCameraAction::MoveLocal(Vec3::NEG_X*2.0),
            [keycode(KeyCode::D, false), keycode(KeyCode::ShiftLeft, false)] => FlyCameraAction::MoveLocal(Vec3::X*2.0),
            [keycode(KeyCode::ControlLeft, false), keycode(KeyCode::ShiftLeft, false)] => FlyCameraAction::MoveLocal(Vec3::NEG_Y*2.0),
            [keycode(KeyCode::Space, false), keycode(KeyCode::ShiftLeft, false)] => FlyCameraAction::MoveLocal(Vec3::Y*2.0),

            [keycode(KeyCode::W, true, )] => FlyCameraAction::MoveLocal(Vec3::NEG_Z * 5.0),

            [FlyCameraInput::MouseMoveX, button(MouseButton::Right, false)] => FlyCameraAction::RotateEuler(-Vec3::X*0.002),
            [FlyCameraInput::MouseMoveY, button(MouseButton::Right, false)] => FlyCameraAction::RotateEuler(-Vec3::Y*0.002),

            [FlyCameraInput::ScrollY] => FlyCameraAction::ChangeSpeed(-0.05),

            [keycode(KeyCode::Key0, true)] => FlyCameraAction::SetSpeed(0.3125),
            [keycode(KeyCode::Key1, true)] => FlyCameraAction::SetSpeed(0.0625),
            [keycode(KeyCode::Key2, true)] => FlyCameraAction::SetSpeed(0.125),
            [keycode(KeyCode::Key3, true)] => FlyCameraAction::SetSpeed(0.25),
            [keycode(KeyCode::Key4, true)] => FlyCameraAction::SetSpeed(0.5),
            [keycode(KeyCode::Key5, true)] => FlyCameraAction::SetSpeed(1.0),
            [keycode(KeyCode::Key6, true)] => FlyCameraAction::SetSpeed(2.0),
            [keycode(KeyCode::Key7, true)] => FlyCameraAction::SetSpeed(4.0),
            [keycode(KeyCode::Key8, true)] => FlyCameraAction::SetSpeed(8.0),
            [keycode(KeyCode::Key9, true)] => FlyCameraAction::SetSpeed(16.0),
        ])
    }
}

#[derive(Reflect, Clone, Copy, Debug)]
pub enum FlyCameraInput {
    KeyCode {
        keycode: KeyCode,
        first_frame: bool,
    },
    MouseButton {
        mouse_button: MouseButton,
        first_frame: bool,
    },
    MouseMoveX,
    MouseMoveY,
    ScrollX,
    ScrollY,
}

#[derive(Reflect, Clone, Copy, Debug)]
pub enum FlyCameraAction {
    MoveLocal(Vec3),
    MoveGlobal(Vec3),
    RotateEuler(Vec3),
    ChangeSpeed(f32),
    SetSpeed(f32),
}
