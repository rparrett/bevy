use crate::{AudioSource, Decodable};
use bevy_asset::{Asset, Handle};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::*;
use bevy_math::Vec3;

/// Defines the volume to play an audio source at.
#[derive(Clone, Copy, Debug)]
pub enum Volume {
    /// A volume level relative to the global volume.
    Relative(VolumeLevel),
    /// A volume level that ignores the global volume.
    Absolute(VolumeLevel),
}

impl Default for Volume {
    fn default() -> Self {
        Self::Relative(VolumeLevel::default())
    }
}

impl Volume {
    /// Create a new volume level relative to the global volume.
    pub fn new_relative(volume: f32) -> Self {
        Self::Relative(VolumeLevel::new(volume))
    }
    /// Create a new volume level that ignores the global volume.
    pub fn new_absolute(volume: f32) -> Self {
        Self::Absolute(VolumeLevel::new(volume))
    }
}

/// A volume level equivalent to a non-negative float.
#[derive(Clone, Copy, Deref, DerefMut, Debug)]
pub struct VolumeLevel(pub(crate) f32);

impl Default for VolumeLevel {
    fn default() -> Self {
        Self(1.0)
    }
}

impl VolumeLevel {
    /// Create a new volume level.
    pub fn new(volume: f32) -> Self {
        debug_assert!(volume >= 0.0);
        Self(volume)
    }
    /// Get the value of the volume level.
    pub fn get(&self) -> f32 {
        self.0
    }
}

/// The way Bevy manages the sound playback.
#[derive(Debug, Clone, Copy)]
pub enum PlaybackMode {
    /// Play the sound once. Do nothing when it ends.
    Once,
    /// Repeat the sound forever.
    Loop,
    /// Despawn the entity when the sound finishes playing.
    Despawn,
    /// Remove the audio components from the entity, when the sound finishes playing.
    Remove,
}

/// Initial settings to be used when audio starts playing.
/// If you would like to control the audio while it is playing, query for the
/// [`AudioSink`][crate::AudioSink] or [`SpatialAudioSink`][crate::SpatialAudioSink]
/// components. Changes to this component will *not* be applied to already-playing audio.
#[derive(Component, Clone, Copy, Debug)]
pub struct PlaybackSettings {
    /// The desired playback behavior.
    pub mode: PlaybackMode,
    /// Volume to play at.
    pub volume: Volume,
    /// Speed to play at.
    pub speed: f32,
    /// Create the sink in paused state.
    /// Useful for "deferred playback", if you want to prepare
    /// the entity, but hear the sound later.
    pub paused: bool,
    /// Enables spatial audio for this source.
    ///
    /// See also: [`SpatialListener`].
    ///
    /// Note: Bevy does not currently support HRTF or any other high-quality 3D sound rendering
    /// features. Spatial audio is implemented via simple left-right stereo panning.
    pub spatial: bool,
}

impl Default for PlaybackSettings {
    fn default() -> Self {
        // TODO: what should the default be: ONCE/DESPAWN/REMOVE?
        Self::ONCE
    }
}

impl PlaybackSettings {
    /// Will play the associated audio source once.
    pub const ONCE: PlaybackSettings = PlaybackSettings {
        mode: PlaybackMode::Once,
        volume: Volume::Relative(VolumeLevel(1.0)),
        speed: 1.0,
        paused: false,
        spatial: false,
    };

    /// Will play the associated audio source in a loop.
    pub const LOOP: PlaybackSettings = PlaybackSettings {
        mode: PlaybackMode::Loop,
        volume: Volume::Relative(VolumeLevel(1.0)),
        speed: 1.0,
        paused: false,
        spatial: false,
    };

    /// Will play the associated audio source once and despawn the entity afterwards.
    pub const DESPAWN: PlaybackSettings = PlaybackSettings {
        mode: PlaybackMode::Despawn,
        volume: Volume::Relative(VolumeLevel(1.0)),
        speed: 1.0,
        paused: false,
        spatial: false,
    };

    /// Will play the associated audio source once and remove the audio components afterwards.
    pub const REMOVE: PlaybackSettings = PlaybackSettings {
        mode: PlaybackMode::Remove,
        volume: Volume::Relative(VolumeLevel(1.0)),
        speed: 1.0,
        paused: false,
        spatial: false,
    };

    /// Helper to start in a paused state.
    pub const fn paused(mut self) -> Self {
        self.paused = true;
        self
    }

    /// Helper to set the volume from start of playback.
    pub const fn with_volume(mut self, volume: Volume) -> Self {
        self.volume = volume;
        self
    }

    /// Helper to set the speed from start of playback.
    pub const fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Helper to enable or disable spatial audio.
    pub const fn with_spatial(mut self, spatial: bool) -> Self {
        self.spatial = spatial;
        self
    }
}

/// Settings for the listener for a spatial audio source.
///

#[derive(Component, Clone, Debug)]
pub struct SpatialListener {
    /// Left ear position relative to the `GlobalTransform`.
    pub left_ear_offset: Vec3,
    /// Right ear position relative to the `GlobalTransform`.
    pub right_ear_offset: Vec3,
}

impl Default for SpatialListener {
    fn default() -> Self {
        Self::new(4.)
    }
}

impl SpatialListener {
    /// Configure spatial audio coming from the `emitter` position and heard by a `listener`.
    ///
    /// `gap` is the distance between the left and right "ears" of the listener. Ears are
    /// positioned on the x axis.
    pub fn new(gap: f32) -> Self {
        SpatialListener {
            left_ear_offset: Vec3::X * gap / -2.0,
            right_ear_offset: Vec3::X * gap / 2.0,
        }
    }
}

/// Use this [`Resource`] to control the global volume of all audio with a [`Volume::Relative`] volume.
///
/// Note: changing this value will not affect already playing audio.
#[derive(Resource, Default, Clone, Copy)]
pub struct GlobalVolume {
    /// The global volume of all audio.
    pub volume: VolumeLevel,
}

impl GlobalVolume {
    /// Create a new [`GlobalVolume`] with the given volume.
    pub fn new(volume: f32) -> Self {
        Self {
            volume: VolumeLevel::new(volume),
        }
    }
}

/// The scale factor applied to the positions of audio sources and listeners for
/// spatial audio.
///
/// You may need to adjust this scale to fit your world's units.
///
/// Default is `Vec3::ONE`.
#[derive(Resource, Clone, Copy)]
pub struct SpatialScale(pub Vec3);

impl SpatialScale {
    /// Create a new `SpatialScale` with the same value for all 3 dimensions.
    pub fn new(scale: f32) -> Self {
        Self(Vec3::splat(scale))
    }
    /// Create a new `SpatialScale` with the same value for `x` and `y`, and `0.0`
    /// for `z`.
    pub fn new_2d(scale: f32) -> Self {
        Self(Vec3::new(scale, scale, 0.0))
    }
}

impl Default for SpatialScale {
    fn default() -> Self {
        Self(Vec3::ONE)
    }
}

/// Bundle for playing a standard bevy audio asset
pub type AudioBundle = AudioSourceBundle<AudioSource>;

/// Bundle for playing a sound.
///
/// Insert this bundle onto an entity to trigger a sound source to begin playing.
///
/// If the handle refers to an unavailable asset (such as if it has not finished loading yet),
/// the audio will not begin playing immediately. The audio will play when the asset is ready.
///
/// When Bevy begins the audio playback, an [`AudioSink`][crate::AudioSink] component will be
/// added to the entity. You can use that component to control the audio settings during playback.
#[derive(Bundle)]
pub struct AudioSourceBundle<Source = AudioSource>
where
    Source: Asset + Decodable,
{
    /// Asset containing the audio data to play.
    pub source: Handle<Source>,
    /// Initial settings that the audio starts playing with.
    /// If you would like to control the audio while it is playing,
    /// query for the [`AudioSink`][crate::AudioSink] component.
    /// Changes to this component will *not* be applied to already-playing audio.
    pub settings: PlaybackSettings,
}

impl<T: Decodable + Asset> Default for AudioSourceBundle<T> {
    fn default() -> Self {
        Self {
            source: Default::default(),
            settings: Default::default(),
        }
    }
}
