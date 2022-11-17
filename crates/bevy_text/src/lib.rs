mod error;
mod font;
mod font_atlas;
mod font_atlas_set;
mod font_loader;
mod glyph_brush;
mod pipeline;
mod text;
mod text2d;

pub use error::*;
pub use font::*;
pub use font_atlas::*;
pub use font_atlas_set::*;
pub use font_loader::*;
pub use glyph_brush::*;
pub use pipeline::*;
pub use text::*;
pub use text2d::*;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        Font, HorizontalAlign, Text, Text2dBundle, TextAlignment, TextError, TextSection,
        TextStyle, VerticalAlign,
    };
}

use bevy_app::prelude::*;
use bevy_asset::AddAsset;
use bevy_ecs::{schedule::IntoSystemDescriptor, system::Resource};
use bevy_render::{camera::CameraUpdateSystem, RenderApp, RenderStage};
use bevy_sprite::SpriteSystem;
use bevy_window::ModifiesWindows;

pub struct TextPlugin {
    /// The number of font sizes allowed for each font before a warning is emitted. (See B0005)
    pub max_font_sizes: Option<usize>,
}

impl Default for TextPlugin {
    fn default() -> Self {
        Self {
            max_font_sizes: Some(16),
        }
    }
}

#[derive(Resource)]
pub struct TextLimits {
    max_font_sizes: Option<usize>,
    warned: bool,
}

/// Text is rendered for two different view projections, normal `Text2DBundle` is rendered with a
/// `BottomToTop` y axis, and UI is rendered with a `TopToBottom` y axis. This matters for text because
/// the glyph positioning is different in either layout.
pub enum YAxisOrientation {
    TopToBottom,
    BottomToTop,
}

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Font>()
            .add_asset::<FontAtlasSet>()
            .register_type::<Text>()
            .register_type::<TextSection>()
            .register_type::<Vec<TextSection>>()
            .register_type::<TextStyle>()
            .register_type::<TextAlignment>()
            .register_type::<VerticalAlign>()
            .register_type::<HorizontalAlign>()
            .init_asset_loader::<FontLoader>()
            .insert_resource(TextPipeline::default())
            .insert_resource(TextLimits {
                max_font_sizes: self.max_font_sizes,
                warned: false,
            })
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_text2d_layout
                    .after(ModifiesWindows)
                    // Potential conflict: `Assets<Image>`
                    // In practice, they run independently since `bevy_render::camera_update_system`
                    // will only ever observe its own render target, and `update_text2d_layout`
                    // will never modify a pre-existing `Image` asset.
                    .ambiguous_with(CameraUpdateSystem),
            );

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_system_to_stage(
                RenderStage::Extract,
                extract_text2d_sprite.after(SpriteSystem::ExtractSprites),
            );
        }
    }
}
