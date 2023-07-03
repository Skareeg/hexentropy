use bevy::{prelude::*, app::AppExit};

pub mod tileset_1bit;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<GameAssets>()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest(),
        ))
        .add_state::<AppState>()
        .add_system(load_textures.in_schedule(OnEnter(AppState::Setup)))
        .add_system(check_tileset_asset.in_set(OnUpdate(AppState::Setup)))
        .add_system(setup.in_schedule(OnEnter(AppState::Run)))
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
    let tileset = atlases.add(TextureAtlas::from_grid(texture.clone(), Vec2::new(16., 16.), 49, 22, Some(Vec2::new(1., 1.)), None));
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

fn coord_ind(x: usize, y: usize, mx: usize) -> usize {
    x + (mx * y)
}

fn setup(mut commands: Commands, assets: Res<GameAssets>, atlases: Res<Assets<TextureAtlas>>) {
    println!("Spawning...");
    let atlas = atlases.get(&assets.atlas1).unwrap();
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: assets.atlas1.clone(),
            sprite: TextureAtlasSprite::new(tileset_1bit::ATLAS1_PERSON_BLANK),
            transform: Transform::from_xyz(-500., 0., 0.),
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
    println!("Ready.");
}