use bevy::{prelude::*, app::AppExit, input::mouse::{MouseWheel, MouseScrollUnit}};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_rapier3d::{prelude::{RapierPhysicsPlugin, NoUserData, RigidBody, Collider, KinematicCharacterController, RapierConfiguration, Ccd, LockedAxes, TimestepMode, Damping, Velocity, Sleeping, ColliderMassProperties, ExternalImpulse, Friction, ActiveEvents}, render::RapierDebugRenderPlugin};
use character::{char_accel_movement_update};
use dungeon::gen_dungeon_floor;
use player::{player_input, player_movement, camera_track_entity};

use crate::{dungeon::{Level, GenRoom}, player::{Player, PlayerInput, PlayerInputMove, NetLocal, CameraTrackEntity}, character::CharacterMovement};

pub mod tileset_1bit;
pub mod character;

pub mod dungeon;
pub mod player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<GameAssets>()
        .insert_resource(RapierConfiguration {
            gravity: Vec3::new(0., 0., -9.8),
            ..default()
        })
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .build()
            .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin)
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default().with_physics_scale(16.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_state::<AppState>()
        .add_system(load_textures.in_schedule(OnEnter(AppState::Setup)))
        .add_system(check_tileset_asset.in_set(OnUpdate(AppState::Setup)))
        .add_system(setup.in_schedule(OnEnter(AppState::Run)))
        .add_system(camera_track_entity)
        .add_system(gen_dungeon_floor)
        .add_system(player_input)
        .add_system(player_movement)
        .add_system(zoomy)
        .add_system(char_accel_movement_update)
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

#[derive(Resource, Default)]
pub struct GameAssets {
    pub atlas1_img: Handle<Image>,
    pub atlas1: Handle<TextureAtlas>,
}

fn load_textures(mut commands: Commands, mut assets: ResMut<GameAssets>, mut atlases: ResMut<Assets<TextureAtlas>>, sets: Res<AssetServer>) {
    let texture = sets.load("tileset.png");
    let tileset = atlases.add(TextureAtlas::from_grid(texture.clone(), Vec2::new(16., 16.), 49, 22, None, None));
    assets.atlas1_img = texture;
    assets.atlas1 = tileset;
    println!("Loading...");
}

fn check_tileset_asset(mut next_state: ResMut<NextState<AppState>>, assets: Res<GameAssets>, sets: Res<AssetServer>, mut ev_app: EventWriter<AppExit>) {
    use bevy::asset::LoadState;
    match sets.get_load_state(assets.atlas1_img.clone()) {
        LoadState::Loaded => {
            println!("Loaded...");
            next_state.set(AppState::Run);
        }
        LoadState::Failed => {
            println!("Failed.");
            ev_app.send(AppExit);
        }
        LoadState::NotLoaded => {
        }
        LoadState::Unloaded => {
            println!("Unloaded.");
            ev_app.send(AppExit);
        }
        _ => {}
    }
}

fn setup(mut commands: Commands, assets: Res<GameAssets>, atlases: Res<Assets<TextureAtlas>>) {
    println!("Spawning...");
    
    // Local player.
    let player = commands.spawn((
        SpriteSheetBundle {
            texture_atlas: assets.atlas1.clone(),
            sprite: TextureAtlasSprite::new(tileset_1bit::TileSet1Bit::Human as usize),
            transform: Transform::from_xyz(50.0 * 16.0, 50.0 * 16.0, 4.),
            ..default()
        },
        Player {
            id: 0,
        },
        PlayerInput {
            movement: None,
        },
        NetLocal,
    ))
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
            LockedAxes::ROTATION_LOCKED,
            ActiveEvents::COLLISION_EVENTS,
        ))
        .id();

    let atlas = atlases.get(&assets.atlas1).unwrap();
    let mut cam = Camera2dBundle::new_with_far(100.0 * 16.0);
    cam.transform.translation.x = 50.0 * 16.0;
    cam.transform.translation.y = 50.0 * 16.0;
    cam.projection.scale = 1.0;
    commands.spawn((cam, CameraTrackEntity { ent: player }));

    // Floor.
    commands.spawn((
        Collider::cuboid(100.0 * 16.0, 100.0 * 16.0, 0.1),
        TransformBundle::from(Transform::from_xyz(50.0 * 16.0, 50.0 * 16.0, 0.0)),
    ));

    // Texture atlas outside of room.
    commands.spawn((
        SpriteBundle {
            texture: atlas.texture.clone(),
            transform: Transform::from_xyz(50.0 * 16.0, 30.0 * 16.0, 0.),
            ..default()
        },
    ));

    // Initial level setup.
    commands.spawn((
        Level::default(),
        GenRoom,
    ));
    println!("Ready.");
}

pub fn zoomy(
    mut projections: Query<&mut OrthographicProjection>,
    mut input: EventReader<MouseWheel>,
) {
    for ev in input.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                for mut projection in &mut projections {
                    if ev.y < 0.0 {
                        projection.scale /= 0.9;
                    } else {
                        projection.scale *= 0.9;
                    }
                }
            }
            _ => {}
        }
    }
}