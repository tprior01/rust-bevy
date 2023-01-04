use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle};
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin, GridFrustumIntersect};
use bevy_polyline::prelude::*;
use std::error::Error;
use csv;
mod camera_mod;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(ui_example)
        .add_startup_system(setup)
        .add_system(camera_mod::pan_orbit_camera)
        .add_plugin(EguiPlugin)
        .add_plugin(InfiniteGridPlugin)
        .add_plugin(PolylinePlugin)
        .add_plugins(DefaultPickingPlugins)
        .run();

    // just to catch compilation errors
    let _ = App::new()
        .add_startup_system(camera_mod::spawn_camera);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {

    // spawn a cube and a light
    // commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //         transform: Transform::from_translation(Vec3::new(121231.896, 189344.672, 0.0)),
    //         ..Default::default()
    //     },
    //     PickableBundle::default(), // <- Makes the mesh pickable.
    // ));
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(121231.896, 189344.672, 0.0)),
        point_light: PointLight {
            radius: f32::MAX,
            range: f32::MAX,
            shadows_enabled: false,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(InfiniteGridBundle {
        grid: InfiniteGrid {
            fadeout_distance: f32::MAX,
            // shadow_color: None,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(121231.896, 189344.672, 0.0)),
        ..Default::default()
    });
    commands.spawn(PolylineBundle {
        polyline: polylines.add(Polyline {
            vertices: read_vector_from_csv("leftrail.csv").unwrap_or(vec![-Vec3::ONE, Vec3::ONE]),
            ..Default::default()
        }),
        material: polyline_materials.add(PolylineMaterial {
            width: 5.0,
            color: Color::RED,
            perspective: false,
            ..Default::default()
        }),
        ..Default::default()
    });
    commands.spawn(PolylineBundle {
        polyline: polylines.add(Polyline {
            vertices: read_vector_from_csv("rightrail.csv").unwrap_or(vec![-Vec3::ONE, Vec3::ONE]),
            ..Default::default()
        }),
        material: polyline_materials.add(PolylineMaterial {
            width: 5.0,
            color: Color::RED,
            perspective: false,
            ..Default::default()
        }),
        ..Default::default()
    });
    camera_mod::spawn_camera(commands);
}

fn read_vector_from_csv(path: &str) -> Result<Vec<Vec3>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut vector: Vec<Vec3> = Vec::new();
    for result in reader.deserialize() {
        let coord: Vec3 = result?;
        vector.push(coord)
        }
    Ok(vector)
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}