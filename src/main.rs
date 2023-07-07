use bevy::{prelude::*, app::AppExit, input::mouse::{MouseWheel, MouseScrollUnit}, utils::HashMap, render::{RenderPlugin, settings::{WgpuSettings, Backends}}};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_rapier3d::{prelude::{RapierPhysicsPlugin, NoUserData, RigidBody, Collider, KinematicCharacterController, RapierConfiguration, Ccd, LockedAxes, Damping, Velocity, Sleeping, ColliderMassProperties, ExternalImpulse, Friction, ActiveEvents}, render::RapierDebugRenderPlugin};
use character::{char_accel_movement_update, CharacterPlugin};
use dungeon::LvlPlugin;
use player::{player_movement, player_input_aim, player_input_move};


use crate::{dungeon::{CmdLvlInit}, player::{Player, PlayerInput, NetLocal}, character::CharacterMovement};

pub mod tileset_1bit;
pub mod character;

pub mod dungeon;
pub mod player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<GameAssets>()
        .insert_resource(RapierConfiguration {
            gravity: Vec3::new(0., -9.8, 0.),
            ..default()
        })
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(RenderPlugin {
                wgpu_settings: WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..default()
                },
            })
            .build()
            .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin)
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default().with_physics_scale(0.5))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(CharacterPlugin)
        .add_plugin(LvlPlugin)
        .add_state::<AppState>()
        .add_system(load_assets.in_schedule(OnEnter(AppState::Setup)))
        .add_system(check_assets.in_set(OnUpdate(AppState::Setup)))
        .add_system(setup.in_schedule(OnEnter(AppState::Run)))
        .add_system(player_input_move)
        .add_system(player_input_aim)
        .add_system(player_movement)
        .run();
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Setup,
    Run,
}

impl States for AppState {
    type Iter = std::array::IntoIter<AppState, 2>;
    fn variants() -> Self::Iter {
        [AppState::Setup, AppState::Run].into_iter()
    }
}

#[derive(Clone)]
pub struct CombinedMesh {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl CombinedMesh {
    pub fn get_mesh(&self) -> Handle<Mesh> {
        self.mesh.clone()
    }
    pub fn get_material(&self) -> Handle<StandardMaterial> {
        self.material.clone()
    }
}

#[derive(Resource, Default)]
pub struct GameAssets {
    pub meshes: HashMap<String, CombinedMesh>,
}

fn load_assets(
    mut commands: Commands,
    mut assets: ResMut<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sets: Res<AssetServer>
) {
    // Load the player.
    assets.meshes.insert("Player".to_owned(),CombinedMesh {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 0.5,
            depth: 1.75,
            ..default()
        })),
        material: materials.add(Color::GREEN.into()),
    });
    println!("Loading...");
}

fn check_assets(mut next_state: ResMut<NextState<AppState>>, assets: Res<GameAssets>, sets: Res<AssetServer>, mut ev_app: EventWriter<AppExit>) {
    use bevy::asset::LoadState;
    // match sets.get_load_state(assets.atlas1_img.clone()) {
    //     LoadState::Loaded => {
    //         println!("Loaded...");
    //         next_state.set(AppState::Run);
    //     }
    //     LoadState::Failed => {
    //         println!("Failed.");
    //         ev_app.send(AppExit);
    //     }
    //     LoadState::NotLoaded => {
    //     }
    //     LoadState::Unloaded => {
    //         println!("Unloaded.");
    //         ev_app.send(AppExit);
    //     }
    //     _ => {}
    // }
    println!("Loaded...");
    next_state.set(AppState::Run);
}

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    println!("Spawning...");

    let start_pos = Vec3::new(16.0, 16.0, 4.0);

    let player_mesh = assets.meshes["Player"].clone();
    
    // Local player.
    commands.spawn((
        Player {
            id: 0,
        },
        PlayerInput {
            movement: None,
            aiming: None,
        },
        NetLocal,
    ))
        // Mesh.
        .insert(
            PbrBundle {
                mesh: player_mesh.get_mesh(),
                material: player_mesh.get_material(),
                transform: Transform::from_translation(start_pos),
                ..default()
            }
        )
        // Physics.
        .insert((
            RigidBody::KinematicPositionBased,
            KinematicCharacterController::default(),
            CharacterMovement {
                acceleration: 250.0,
                dampening: 250.0,
                max_speed: 3.0,
                min_threshold: 0.05,
                ..default()
            },
            Collider::cuboid(0.75, 0.75, 1.75),
            ColliderMassProperties::Mass(100.0),
            Friction::coefficient(0.8),
            Velocity::default(),
            ExternalImpulse::default(),
            Damping {
                linear_damping: 1.0,
                angular_damping: 0.0,
            },
            Sleeping::disabled(),
            Ccd::enabled(),
            LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Y,
            ActiveEvents::COLLISION_EVENTS,
        ))
        // Camera.
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.65, 0.0),
                ..default()
            });
        });

    // Ambient lighting for all.
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

    // Floor.
    commands.spawn((
        Collider::cuboid(100.0 * 16.0, 0.1, 100.0 * 16.0),
        TransformBundle::from(Transform::from_xyz(50.0 * 16.0, 0.0, 50.0 * 16.0)),
    ));

    // Initial level setup.
    commands.spawn(
        CmdLvlInit {
            seed: 0,
        }
    );
    println!("Ready.");
}
