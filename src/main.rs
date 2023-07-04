use bevy::{prelude::*, app::AppExit};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_rapier3d::{prelude::{RapierPhysicsPlugin, NoUserData}, render::RapierDebugRenderPlugin};
use dungeon::gen_dungeon_floor;

use crate::dungeon::{Level, GenRoom};

pub mod tileset_1bit;
pub mod physics;

pub mod dungeon;
pub mod player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<GameAssets>()
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
        .add_system(zoomout)
        .add_system(gen_dungeon_floor)
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
    let atlas = atlases.get(&assets.atlas1).unwrap();
    let mut cam = Camera2dBundle::new_with_far(100.0 * 16.0);
    cam.transform.translation.x = 50.0 * 16.0;
    cam.transform.translation.y = 50.0 * 16.0;
    cam.projection.scale = 1.5;
    commands.spawn(cam);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: assets.atlas1.clone(),
            sprite: TextureAtlasSprite::new(tileset_1bit::TileSet1Bit::Human as usize),
            transform: Transform::from_xyz(50.0 * 16.0, 50.0 * 16.0, 0.),
            ..default()
        },
    ));
    commands.spawn((
        SpriteBundle {
            texture: atlas.texture.clone(),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
    ));
    commands.spawn((
        Level::default(),
        GenRoom,
    ));
    println!("Ready.");
}

fn zoomout(mut cams: Query<&mut OrthographicProjection, With<Camera>>, time: Res<Time>) {
    use lerp::Lerp;
    for mut cam in &mut cams {
        cam.scale = cam.scale.lerp(cam.scale / 1.0002, time.elapsed_seconds());
    }
}