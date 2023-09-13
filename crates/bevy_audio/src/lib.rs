//! Audio support for the game engine Bevy
//!
//! ```no_run
//! # use bevy_ecs::prelude::*;
//! # use bevy_audio::{AudioBundle, AudioPlugin, PlaybackSettings};
//! # use bevy_asset::{AssetPlugin, AssetServer};
//! # use bevy_app::{App, AppExit, NoopPluginGroup as MinimalPlugins, Startup};
//! fn main() {
//!    App::new()
//!         .add_plugins((MinimalPlugins, AssetPlugin::default(), AudioPlugin::default()))
//!         .add_systems(Startup, play_background_audio)
//!         .run();
//! }
//!
//! fn play_background_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
//!     commands.spawn(AudioBundle {
//!         source: asset_server.load("background_audio.ogg"),
//!         settings: PlaybackSettings::LOOP,
//!     });
//! }
//! ```

#![forbid(unsafe_code)]
#![allow(clippy::type_complexity)]
#![warn(missing_docs)]

mod audio;
mod audio_output;
mod audio_source;
mod pitch;
mod sinks;

#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        AudioBundle, AudioSink, AudioSinkPlayback, AudioSource, AudioSourceBundle, Decodable,
        GlobalVolume, Pitch, PitchBundle, PlaybackSettings, SpatialAudioSink, SpatialListener,
    };
}

pub use audio::*;
pub use audio_source::*;
pub use pitch::*;

pub use rodio::cpal::Sample as CpalSample;
pub use rodio::source::Source;
pub use rodio::Sample;
pub use sinks::*;

use bevy_app::prelude::*;
use bevy_asset::{Asset, AssetApp};
use bevy_ecs::prelude::*;
use bevy_transform::TransformSystem;

use audio_output::*;

/// Set for the audio playback systems, so they can share a run condition
#[derive(SystemSet, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct AudioPlaySet;

/// Adds support for audio playback to a Bevy Application
///
/// Insert an [`AudioBundle`] onto your entities to play audio.
#[derive(Default)]
pub struct AudioPlugin {
    /// The global volume for all audio entities with a [`Volume::Relative`] volume.
    pub global_volume: GlobalVolume,
    /// The scale factor applied to the positions of audio sources and listeners for
    /// spatial audio.
    pub spatial_scale: SpatialScale,
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.global_volume)
            .insert_resource(self.spatial_scale)
            .configure_sets(
                PostUpdate,
                AudioPlaySet
                    .run_if(audio_output_available)
                    .after(TransformSystem::TransformPropagate), // For spatial audio transforms
            )
            .init_resource::<AudioOutput>();

        #[cfg(any(feature = "mp3", feature = "flac", feature = "wav", feature = "vorbis"))]
        {
            app.add_audio_source::<AudioSource>();
            app.init_asset_loader::<AudioLoader>();
        }

        app.add_audio_source::<Pitch>();
    }
}

impl AddAudioSource for App {
    fn add_audio_source<T>(&mut self) -> &mut Self
    where
        T: Decodable + Asset,
        f32: rodio::cpal::FromSample<T::DecoderItem>,
    {
        self.init_asset::<T>().add_systems(
            PostUpdate,
            // TODO after transform propagation?
            play_queued_audio_system::<T>.in_set(AudioPlaySet),
        );
        self.add_systems(PostUpdate, cleanup_finished_audio::<T>.in_set(AudioPlaySet));
        self.add_systems(PostUpdate, update_emitter_positions.in_set(AudioPlaySet));
        self.add_systems(PostUpdate, update_listener_positions.in_set(AudioPlaySet));
        self
    }
}
