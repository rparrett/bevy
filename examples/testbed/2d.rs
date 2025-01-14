//! 2d testbed
//!
//! You can switch scene by pressing the spacebar

#[cfg(feature = "bevy_ci_testing")]
use bevy::dev_tools::ci_testing::CiTestingCustomEvent;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .init_state::<Scene>()
        .add_systems(OnEnter(Scene::Shapes), shapes::setup)
        .add_systems(OnEnter(Scene::Bloom), bloom::setup)
        .add_systems(OnEnter(Scene::Text), text::setup)
        .add_systems(OnEnter(Scene::Sprite), sprite::setup)
        .add_systems(Update, switch_scene);
    app.run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
#[states(scoped_entities)]
enum Scene {
    #[default]
    Shapes,
    Bloom,
    Text,
    Sprite,
}

fn switch_scene(
    keyboard: Res<ButtonInput<KeyCode>>,
    #[cfg(feature = "bevy_ci_testing")] mut ci_events: EventReader<CiTestingCustomEvent>,
    scene: Res<State<Scene>>,
    mut next_scene: ResMut<NextState<Scene>>,
) {
    let mut should_switch = false;
    should_switch |= keyboard.just_pressed(KeyCode::Space);
    #[cfg(feature = "bevy_ci_testing")]
    {
        should_switch |= ci_events.read().any(|event| match event {
            CiTestingCustomEvent(event) => event == "switch_scene",
        });
    }
    if should_switch {
        info!("Switching scene");
        next_scene.set(match scene.get() {
            Scene::Shapes => Scene::Bloom,
            Scene::Bloom => Scene::Text,
            Scene::Text => Scene::Sprite,
            Scene::Sprite => Scene::Shapes,
        });
    }
}

mod shapes {
    use bevy::prelude::*;

    const X_EXTENT: f32 = 900.;

    pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn((Camera2d, StateScoped(super::Scene::Shapes)));

        let shapes = [
            meshes.add(Circle::new(50.0)),
            meshes.add(CircularSector::new(50.0, 1.0)),
            meshes.add(CircularSegment::new(50.0, 1.25)),
            meshes.add(Ellipse::new(25.0, 50.0)),
            meshes.add(Annulus::new(25.0, 50.0)),
            meshes.add(Capsule2d::new(25.0, 50.0)),
            meshes.add(Rhombus::new(75.0, 100.0)),
            meshes.add(Rectangle::new(50.0, 100.0)),
            meshes.add(RegularPolygon::new(50.0, 6)),
            meshes.add(Triangle2d::new(
                Vec2::Y * 50.0,
                Vec2::new(-50.0, -50.0),
                Vec2::new(50.0, -50.0),
            )),
        ];
        let num_shapes = shapes.len();

        for (i, shape) in shapes.into_iter().enumerate() {
            // Distribute colors evenly across the rainbow.
            let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

            commands.spawn((
                Mesh2d(shape),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(
                    // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    0.0,
                    0.0,
                ),
                StateScoped(super::Scene::Shapes),
            ));
        }
    }
}

mod bloom {
    use bevy::{
        core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
        prelude::*,
    };

    pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn((
            Camera2d,
            Camera {
                hdr: true,
                ..default()
            },
            Tonemapping::TonyMcMapface,
            Bloom::default(),
            StateScoped(super::Scene::Bloom),
        ));

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(100.))),
            MeshMaterial2d(materials.add(Color::srgb(7.5, 0.0, 7.5))),
            Transform::from_translation(Vec3::new(-200., 0., 0.)),
            StateScoped(super::Scene::Bloom),
        ));

        commands.spawn((
            Mesh2d(meshes.add(RegularPolygon::new(100., 6))),
            MeshMaterial2d(materials.add(Color::srgb(6.25, 9.4, 9.1))),
            Transform::from_translation(Vec3::new(200., 0., 0.)),
            StateScoped(super::Scene::Bloom),
        ));
    }
}

mod text {
    use bevy::color::palettes;
    use bevy::prelude::*;
    use bevy::sprite::Anchor;
    use bevy::text::TextBounds;

    fn example(
        commands: &mut Commands,
        dest: Vec3,
        justify: JustifyText,
        bounds: Option<TextBounds>,
    ) {
        commands.spawn((
            Sprite {
                color: palettes::css::YELLOW.into(),
                custom_size: Some(5. * Vec2::ONE),
                ..Default::default()
            },
            Transform::from_translation(dest),
        ));

        for anchor in [
            Anchor::TopLeft,
            Anchor::TopRight,
            Anchor::BottomRight,
            Anchor::BottomLeft,
        ] {
            let mut text = commands.spawn((
                Text2d::new("L R\n"),
                TextLayout::new_with_justify(justify.clone()),
                Transform::from_translation(dest + Vec3::Z),
                anchor.clone(),
                StateScoped(super::Scene::Text),
            ));
            text.with_children(|parent| {
                parent.spawn((
                    TextSpan::new(format!("{anchor:?}\n")),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(palettes::tailwind::BLUE_400.into()),
                ));
                parent.spawn((
                    TextSpan::new(format!("{justify:?}")),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(palettes::tailwind::GREEN_400.into()),
                ));
            });

            if let Some(bounds) = bounds {
                text.insert(bounds);

                commands.spawn((
                    Sprite {
                        color: palettes::tailwind::GRAY_900.into(),
                        custom_size: Some(Vec2::new(bounds.width.unwrap(), bounds.height.unwrap())),
                        anchor: anchor.clone(),
                        ..Default::default()
                    },
                    Transform::from_translation(dest - Vec3::Z),
                    StateScoped(super::Scene::Text),
                ));
            }
        }
    }

    pub fn setup(mut commands: Commands) {
        commands.spawn((Camera2d::default(), StateScoped(super::Scene::Text)));

        for (i, justify) in [
            JustifyText::Left,
            JustifyText::Right,
            JustifyText::Center,
            JustifyText::Justified,
        ]
        .into_iter()
        .enumerate()
        {
            let y = 230. - 150. * i as f32;
            example(&mut commands, -300. * Vec3::X + y * Vec3::Y, justify, None);
            example(
                &mut commands,
                300. * Vec3::X + y * Vec3::Y,
                justify,
                Some(TextBounds::new(150., 55.)),
            );
        }
    }
}

mod sprite {
    use bevy::prelude::*;

    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((Camera2d, StateScoped(super::Scene::Sprite)));
        commands.spawn((
            Sprite::from_image(asset_server.load("branding/bevy_bird_dark.png")),
            StateScoped(super::Scene::Sprite),
        ));
    }
}
