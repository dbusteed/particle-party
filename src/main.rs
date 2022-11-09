use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, Slider},
    EguiContext, EguiPlugin,
};
use bevy_prototype_lyon::prelude as ly;
use heron::prelude::*;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

const BG_COLOR: Color = Color::rgb(0.87, 0.87, 0.87);
const MIN_R: f32 = 10.0;

struct Settings {
    shape_radius: f32,
    collision_radius: f32,
    base_g: f32,
    rn: usize,
    bn: usize,
    gn: usize,
    yn: usize,
    r: bool,
    b: bool,
    g: bool,
    y: bool,
    r_r: f32,
    r_b: f32,
    r_g: f32,
    r_y: f32,
    b_r: f32,
    b_b: f32,
    b_g: f32,
    b_y: f32,
    g_r: f32,
    g_b: f32,
    g_g: f32,
    g_y: f32,
    y_r: f32,
    y_b: f32,
    y_g: f32,
    y_y: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            shape_radius: 5.0,
            collision_radius: 5.0,
            base_g: 100.0,
            rn: 100,
            bn: 100,
            gn: 100,
            yn: 100,
            r: true,
            b: true,
            g: true,
            y: true,
            r_r: 1.0,
            r_b: -0.5,
            r_g: -0.5,
            r_y: -0.5,
            b_r: -0.5,
            b_b: 1.0,
            b_g: -0.5,
            b_y: -0.5,
            g_r: -0.5,
            g_b: -0.5,
            g_g: 1.0,
            g_y: -0.5,
            y_r: -0.5,
            y_b: -0.5,
            y_g: -0.5,
            y_y: 1.0,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum PType {
    Red,
    Blue,
    Green,
    Yellow,
}

#[derive(Component)]
struct Particle(PType);

#[derive(Component)]
struct Border;

struct Init;

#[derive(PhysicsLayer)]
enum Layers {
    Particle,
    Boundary,
}

fn main() {
    App::new()
        .add_event::<Init>()
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(WindowDescriptor {
            title: "Particle Party".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ly::ShapePlugin)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Settings::default())
        .add_startup_system(init_setup)
        .add_system(init_particles)
        .add_system(calculate_acceleration)
        .add_system(update_velocity)
        .add_system(ui_box)
        .run();
}

fn init_setup(mut commands: Commands, mut init_evt: EventWriter<Init>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.insert_resource(HashMap::from([
        (PType::Red, Color::RED),
        (PType::Blue, Color::BLUE),
        (PType::Green, Color::DARK_GREEN),
        (PType::Yellow, Color::YELLOW),
    ]));

    init_evt.send(Init);
}

fn init_particles(
    mut commands: Commands,
    mut init_evt: EventReader<Init>,
    mut settings: ResMut<Settings>,
    mut windows: ResMut<Windows>,
    colors: Res<HashMap<PType, Color>>,
    particles: Query<Entity, With<Particle>>,
    borders: Query<Entity, With<Border>>,
) {
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    for _ev in init_evt.iter() {
        // clear all particles first
        for ent in particles.iter() {
            commands.entity(ent).despawn();
        }

        for ent in &borders {
            commands.entity(ent).despawn();
        }

        commands
            .spawn()
            .insert_bundle(ly::GeometryBuilder::build_as(
                &ly::shapes::Rectangle {
                    extents: Vec2::new(2., win_h),
                    origin: ly::shapes::RectangleOrigin::Center,
                },
                ly::DrawMode::Fill(ly::FillMode::color(BG_COLOR)),
                Transform::default(),
            ))
            .insert(RigidBody::Static)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(1., win_h / 2., 0.),
                border_radius: Some(0.),
            })
            .insert(
                CollisionLayers::none()
                    .with_group(Layers::Boundary)
                    .with_masks(&[Layers::Particle]),
            )
            .insert(Transform::from_xyz(-win_w / 2., 0., 0.))
            .insert(Border);

        commands
            .spawn()
            .insert_bundle(ly::GeometryBuilder::build_as(
                &ly::shapes::Rectangle {
                    extents: Vec2::new(2., win_h),
                    origin: ly::shapes::RectangleOrigin::Center,
                },
                ly::DrawMode::Fill(ly::FillMode::color(BG_COLOR)),
                Transform::default(),
            ))
            .insert(RigidBody::Static)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(1., win_h / 2., 0.),
                border_radius: Some(0.),
            })
            .insert(
                CollisionLayers::none()
                    .with_group(Layers::Boundary)
                    .with_masks(&[Layers::Particle]),
            )
            .insert(Transform::from_xyz(win_w / 2., 0., 0.))
            .insert(Border);

        commands
            .spawn()
            .insert_bundle(ly::GeometryBuilder::build_as(
                &ly::shapes::Rectangle {
                    extents: Vec2::new(win_w, 2.),
                    origin: ly::shapes::RectangleOrigin::Center,
                },
                ly::DrawMode::Fill(ly::FillMode::color(BG_COLOR)),
                Transform::default(),
            ))
            .insert(RigidBody::Static)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(win_w / 2., 1., 0.),
                border_radius: Some(0.),
            })
            .insert(
                CollisionLayers::none()
                    .with_group(Layers::Boundary)
                    .with_masks(&[Layers::Particle]),
            )
            .insert(Transform::from_xyz(0., -win_h / 2., 0.))
            .insert(Border);

        commands
            .spawn()
            .insert_bundle(ly::GeometryBuilder::build_as(
                &ly::shapes::Rectangle {
                    extents: Vec2::new(win_w, 2.),
                    origin: ly::shapes::RectangleOrigin::Center,
                },
                ly::DrawMode::Fill(ly::FillMode::color(BG_COLOR)),
                Transform::default(),
            ))
            .insert(RigidBody::Static)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(win_w / 2., 1., 0.),
                border_radius: Some(0.),
            })
            .insert(
                CollisionLayers::none()
                    .with_group(Layers::Boundary)
                    .with_masks(&[Layers::Particle]),
            )
            .insert(Transform::from_xyz(0., win_h / 2., 0.))
            .insert(Border);

        // update the bool settings for the UI sliders
        // (these are handled separate so changing the #
        // mid simulation doesn't hide settings until "Restart")
        settings.r = settings.rn > 0;
        settings.b = settings.bn > 0;
        settings.g = settings.gn > 0;
        settings.y = settings.yn > 0;

        let n_particles = vec![
            (PType::Red, settings.rn),
            (PType::Blue, settings.bn),
            (PType::Green, settings.gn),
            (PType::Yellow, settings.yn),
        ];

        for (ptype, n) in n_particles {
            for _ in 0..n {
                let x: f32 = thread_rng().gen_range((-win_w / 2. + 5.)..(win_w / 2. - 5.));
                let y: f32 = thread_rng().gen_range((-win_h / 2. + 5.)..(win_h / 2. - 5.));

                commands
                    .spawn()
                    .insert_bundle(ly::GeometryBuilder::build_as(
                        &ly::shapes::Circle {
                            radius: settings.shape_radius,
                            center: Vec2::ZERO,
                        },
                        ly::DrawMode::Fill(ly::FillMode::color(*colors.get(&ptype).unwrap())),
                        Transform::default(),
                    ))
                    .insert(RigidBody::Dynamic)
                    .insert(Velocity::default())
                    .insert(Acceleration::default())
                    .insert(CollisionShape::Sphere {
                        radius: settings.collision_radius,
                    })
                    .insert(
                        CollisionLayers::none()
                            .with_group(Layers::Particle)
                            .with_masks(&[Layers::Particle, Layers::Boundary]),
                    )
                    .insert(Transform::from_xyz(x, y, 0.))
                    .insert(Particle(ptype));
            }
        }
    }
}

fn calculate_acceleration(
    settings: Res<Settings>,
    mut query: Query<(Entity, &Transform, &mut Acceleration, &Particle)>,
) {
    let mut accel_map: HashMap<u32, Vec3> = HashMap::new();
    for (ent, trans, _, part) in query.iter() {
        let mut accel = Vec3::ZERO;
        for (ent2, trans2, _, part2) in query.iter() {
            if ent == ent2 {
                continue;
            }

            let g = match (part.0, part2.0) {
                (PType::Red, PType::Red) => settings.r_r,
                (PType::Red, PType::Blue) => settings.r_b,
                (PType::Red, PType::Green) => settings.r_g,
                (PType::Red, PType::Yellow) => settings.r_y,
                (PType::Blue, PType::Red) => settings.b_r,
                (PType::Blue, PType::Blue) => settings.b_b,
                (PType::Blue, PType::Green) => settings.b_g,
                (PType::Blue, PType::Yellow) => settings.b_y,
                (PType::Green, PType::Red) => settings.g_r,
                (PType::Green, PType::Blue) => settings.g_b,
                (PType::Green, PType::Green) => settings.g_g,
                (PType::Green, PType::Yellow) => settings.g_y,
                (PType::Yellow, PType::Red) => settings.y_r,
                (PType::Yellow, PType::Blue) => settings.y_b,
                (PType::Yellow, PType::Green) => settings.y_g,
                (PType::Yellow, PType::Yellow) => settings.y_y,
            };

            let r_vector = trans.translation - trans2.translation;
            let r_mag = r_vector.length();
            if r_mag > MIN_R {
                let accel2: f32 = g * -1.0 * settings.base_g / r_mag.powf(2.0);
                let r_vector_unit = r_vector / r_mag;
                accel += accel2 * r_vector_unit;
            }
        }

        accel_map.insert(ent.id(), accel);
    }

    for (ent, _, mut accel, _) in query.iter_mut() {
        accel.linear = *accel_map.get(&ent.id()).unwrap();
    }
}

fn update_velocity(mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut vel, acc) in query.iter_mut() {
        vel.linear += acc.linear;
    }
}

fn ui_box(
    mut settings: ResMut<Settings>,
    mut egui_context: ResMut<EguiContext>,
    mut init_evt: EventWriter<Init>,
) {
    egui::Window::new("").show(egui_context.ctx_mut(), |ui| {
        ui.add(
            Slider::new(&mut settings.base_g, 0.0..=100.0)
                .text("Base Gravity")
                .text_color(Color32::WHITE),
        );
        if settings.r {
            ui.add(
                Slider::new(&mut settings.r_r, -5.0..=5.0)
                    .text("Red -> Red")
                    .text_color(Color32::LIGHT_RED),
            );
            if settings.b {
                ui.add(
                    Slider::new(&mut settings.r_b, -5.0..=5.0)
                        .text("Red -> Blue")
                        .text_color(Color32::LIGHT_RED),
                );
            }
            if settings.g {
                ui.add(
                    Slider::new(&mut settings.r_g, -5.0..=5.0)
                        .text("Red -> Green")
                        .text_color(Color32::LIGHT_RED),
                );
            }
            if settings.y {
                ui.add(
                    Slider::new(&mut settings.r_y, -5.0..=5.0)
                        .text("Red -> Yellow")
                        .text_color(Color32::LIGHT_RED),
                );
            }
        }
        if settings.b {
            ui.add(
                Slider::new(&mut settings.b_b, -5.0..=5.0)
                    .text("Blue -> Blue")
                    .text_color(Color32::LIGHT_BLUE),
            );
            if settings.r {
                ui.add(
                    Slider::new(&mut settings.b_r, -5.0..=5.0)
                        .text("Blue -> Red")
                        .text_color(Color32::LIGHT_BLUE),
                );
            }
            if settings.g {
                ui.add(
                    Slider::new(&mut settings.b_g, -5.0..=5.0)
                        .text("Blue -> Green")
                        .text_color(Color32::LIGHT_BLUE),
                );
            }
            if settings.y {
                ui.add(
                    Slider::new(&mut settings.b_y, -5.0..=5.0)
                        .text("Blue -> Yellow")
                        .text_color(Color32::LIGHT_BLUE),
                );
            }
        }
        if settings.g {
            ui.add(
                Slider::new(&mut settings.g_g, -5.0..=5.0)
                    .text("Green -> Green")
                    .text_color(Color32::LIGHT_GREEN),
            );
            if settings.r {
                ui.add(
                    Slider::new(&mut settings.g_r, -5.0..=5.0)
                        .text("Green -> Red")
                        .text_color(Color32::LIGHT_GREEN),
                );
            }
            if settings.b {
                ui.add(
                    Slider::new(&mut settings.g_b, -5.0..=5.0)
                        .text("Green -> Blue")
                        .text_color(Color32::LIGHT_GREEN),
                );
            }
            if settings.y {
                ui.add(
                    Slider::new(&mut settings.g_y, -5.0..=5.0)
                        .text("Green -> Yellow")
                        .text_color(Color32::LIGHT_GREEN),
                );
            }
        }
        if settings.y {
            ui.add(
                Slider::new(&mut settings.y_y, -5.0..=5.0)
                    .text("Yellow -> Yellow")
                    .text_color(Color32::YELLOW),
            );
            if settings.r {
                ui.add(
                    Slider::new(&mut settings.y_r, -5.0..=5.0)
                        .text("Yellow -> Red")
                        .text_color(Color32::YELLOW),
                );
            }
            if settings.b {
                ui.add(
                    Slider::new(&mut settings.y_b, -5.0..=5.0)
                        .text("Yellow -> Blue")
                        .text_color(Color32::YELLOW),
                );
            }
            if settings.g {
                ui.add(
                    Slider::new(&mut settings.y_g, -5.0..=5.0)
                        .text("Yellow -> Green")
                        .text_color(Color32::YELLOW),
                );
            }
        }
        ui.separator();
        ui.add(
            Slider::new(&mut settings.shape_radius, 1.0..=10.0)
                .text("Shape Radius")
                .text_color(Color32::WHITE),
        );
        ui.add(
            Slider::new(&mut settings.collision_radius, 1.0..=10.0)
                .text("Collision Radius")
                .text_color(Color32::WHITE),
        );
        ui.add(
            Slider::new(&mut settings.rn, 0..=1000)
                .text("Red Count")
                .text_color(Color32::LIGHT_RED),
        );
        ui.add(
            Slider::new(&mut settings.bn, 0..=1000)
                .text("Blue Count")
                .text_color(Color32::LIGHT_BLUE),
        );
        ui.add(
            Slider::new(&mut settings.gn, 0..=1000)
                .text("Green Count")
                .text_color(Color32::LIGHT_GREEN),
        );
        ui.add(
            Slider::new(&mut settings.yn, 0..=1000)
                .text("Yellow Count")
                .text_color(Color32::YELLOW),
        );
        if ui.add(egui::Button::new("Restart")).clicked() {
            init_evt.send(Init);
        }
    });
}
