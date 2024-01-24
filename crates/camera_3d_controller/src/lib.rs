use bevy_denshi_ika_gen_plugin::gen_plugin;

use crate::flycam::FlyCameraControllerPlugin;

pub mod flycam;

gen_plugin! {
    pub Camera3dControllerPlugin;
    plugins(FlyCameraControllerPlugin);
}
