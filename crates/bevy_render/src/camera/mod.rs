#[allow(clippy::module_inception)]
mod camera;
mod camera_driver_node;
mod clear_color;
mod manual_texture_view;
mod projection;

pub use camera::*;
pub use camera_driver_node::*;
pub use clear_color::*;
pub use manual_texture_view::*;
pub use projection::*;

use crate::{
    extract_component::ExtractComponentPlugin, extract_resource::ExtractResourcePlugin,
    render_graph::RenderGraph, ExtractSchedule, Render, RenderApp, RenderSet,
};
use bevy_app::{App, Plugin, PostStartup, PostUpdate};
use bevy_ecs::schedule::{IntoSystemConfigs, SystemSet};

/// Label for [`camera_system`].
///
/// [`camera_system`]: crate::camera::camera_system
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct CameraUpdateSystem;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Camera>()
            .register_type::<Viewport>()
            .register_type::<Option<Viewport>>()
            .register_type::<ScalingMode>()
            .register_type::<RenderTarget>()
            .register_type::<ClearColor>()
            .register_type::<ClearColorConfig>()
            .register_type::<Projection>()
            .init_resource::<ManualTextureViews>()
            .init_resource::<ClearColor>()
            .add_systems(PostStartup, camera_system.in_set(CameraUpdateSystem))
            .add_systems(PostUpdate, camera_system.in_set(CameraUpdateSystem))
            .add_plugins((
                ExtractResourcePlugin::<ManualTextureViews>::default(),
                ExtractResourcePlugin::<ClearColor>::default(),
                ExtractComponentPlugin::<CameraMainTextureUsages>::default(),
            ));

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .init_resource::<SortedCameras>()
                .add_systems(ExtractSchedule, extract_cameras)
                .add_systems(Render, sort_cameras.in_set(RenderSet::ManageViews));
            let camera_driver_node = CameraDriverNode::new(&mut render_app.world);
            let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
            render_graph.add_node(crate::graph::CameraDriverLabel, camera_driver_node);
        }
    }
}
