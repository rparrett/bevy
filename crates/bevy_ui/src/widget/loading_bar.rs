//! A loading bar widget.
//! Can be used for loading bars, but also health-bars, mana, those kind of things.

use bevy_ecs::{
    prelude::{Bundle, Component},
    query::With,
    system::Query,
};
use bevy_hierarchy::Children;
use bevy_log::warn;

use crate::{
    prelude::NodeBundle, AlignItems, BackgroundColor, JustifyContent, PositionType, Size, Style,
    Val,
};

#[derive(Component, Default, Clone, Debug)]
pub struct LoadingBarWidget {
    progress: f32,
}

/// Marker component for the inner box of the loading bar.
#[derive(Component, Default, Clone, Debug)]
pub struct LoadingBarInner;

impl LoadingBarWidget {
    /// Creates a new [``LoadingBarWidget`].
    pub const fn new(progress: f32) -> Self {
        LoadingBarWidget { progress }
    }
    pub fn get_progress(&self) -> f32 {
        self.progress
    }

    pub fn set_progress(&mut self, progress: f32) {
        if progress >= 0. && progress <= 1. {
            self.progress = progress;
        } else {
            warn!("Trying to set progress out of range");
        }
    }
}

#[derive(Bundle, Clone, Debug, Default)]
pub struct LoadingBarWidgetBundle {
    pub node_bundle: NodeBundle,
    pub loading_bar: LoadingBarWidget,
}

#[derive(Bundle, Clone, Debug, Default)]
pub struct LoadingBarWidgetInnerBundle {
    pub node_bundle: NodeBundle,
    pub loading_bar: LoadingBarInner,
}

impl LoadingBarWidgetBundle {
    pub fn new(size: Size, background_color: BackgroundColor) -> Self {
        Self {
            node_bundle: NodeBundle {
                background_color,
                style: Style {
                    size,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl LoadingBarWidgetInnerBundle {
    pub fn new(background_color: BackgroundColor) -> Self {
        Self {
            node_bundle: NodeBundle {
                background_color,
                style: Style {
                    size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub(crate) fn update_loading_bars(
    q: Query<(&LoadingBarWidget, &Children)>,
    mut inner: Query<&mut Style, With<LoadingBarInner>>,
) {
    for (widget, children) in q.iter() {
        let mut styles = inner.iter_many_mut(&**children);
        while let Some(mut style) = styles.fetch_next() {
            style.size = Size::new(
                Val::Percent(widget.get_progress() * 100.0),
                Val::Percent(100.0),
            );
        }
    }
}
