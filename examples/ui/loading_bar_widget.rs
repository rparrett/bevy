//! This example illustrates how to setup and use the LoadingBarWidget

use bevy::{
    prelude::*,
    ui::widget::{LoadingBarInner, LoadingBarWidget, LoadingBarWidgetInnerBundle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Progress::Completed(2.0))
        .add_startup_system(setup)
        .add_system(update_progress_state)
        .add_system(set_widget_progress.after(update_progress_state))
        .add_system(update_widget_text.after(set_widget_progress))
        .run();
}

#[derive(Resource, Clone, Copy, Debug)]
enum Progress {
    Loading(f32),
    Completed(f32),
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    // ui camera
    commands.spawn(Camera2dBundle::default());

    // root
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                // horizontally center children nodes
                justify_content: JustifyContent::Center,
                // vertically center children nodes
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgba(19.0 / 255.0, 21.0 / 255.0, 22.0 / 255.0, 1.0).into(),
            ..default()
        })
        .with_children(|root| {
            root.spawn(LoadingBarWidgetBundle::new(
                Size::new(Val::Percent(50.0), Val::Px(50.0)),
                Color::rgba_u8(29, 31, 33, 255).into(),
            ))
            .with_children(|outer| {
                outer.spawn(LoadingBarWidgetInnerBundle::new(
                    Color::rgba_u8(50, 104, 159, 255).into(),
                ));

                outer.spawn(TextBundle {
                    text: Text::from_section(
                        "Loading Bar",
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::rgba(197.0 / 255.0, 198.0 / 255.0, 190.0 / 255.0, 1.0),
                        },
                    ),
                    style: Style {
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                });
            });
        });
}

const LOAD_DURATION: f32 = 3.0;
const COMPLETE_DURATION: f32 = 1.5;

/// This is just a helper-system.
fn update_progress_state(mut progress: ResMut<Progress>, time: Res<Time>) {
    let elapsed_time = match *progress {
        Progress::Loading(value) => value,
        Progress::Completed(value) => value,
    } + time.delta_seconds();

    *progress = match *progress {
        Progress::Loading(_) => {
            if elapsed_time >= LOAD_DURATION {
                Progress::Completed(0.0)
            } else {
                Progress::Loading(elapsed_time)
            }
        }
        Progress::Completed(_) => {
            if elapsed_time >= COMPLETE_DURATION {
                Progress::Loading(0.0)
            } else {
                Progress::Completed(elapsed_time)
            }
        }
    };
}

/// This is responsible for updating the value of the LoadingBarWidget component.
/// This could be in response to changes in player health values, loading of assets ++.
fn set_widget_progress(mut q: Query<&mut LoadingBarWidget>, progress: Res<Progress>) {
    for mut widget in q.iter_mut() {
        let current_progress = match *progress {
            Progress::Loading(value) => map_range(value, (0., LOAD_DURATION), (0., 1.)),
            Progress::Completed(_) => 1.,
        };
        widget.set_progress(current_progress);
    }
}

fn update_widget_text(
    widgets: Query<(&LoadingBarWidget, &Children)>,
    mut q: Query<&mut Text, With<Parent>>,
) {
    for (widget, children) in widgets.iter() {
        for child in children.iter() {
            if let Ok(mut text) = q.get_mut(*child) {
                let progress = widget.get_progress();
                if progress >= 1.0 {
                    text.sections[0].value = format!("Loading complete!");
                } else {
                    text.sections[0].value = format!("Loading: {:.2}%", progress * 100.0);
                }
            }
        }
    }
}

fn map_range(value: f32, old_range: (f32, f32), new_range: (f32, f32)) -> f32 {
    (value - old_range.0) / (old_range.1 - old_range.0) * (new_range.1 - new_range.0) + new_range.0
}
