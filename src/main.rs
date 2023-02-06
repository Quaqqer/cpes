use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};
use bevy_rapier2d::prelude::*;

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Background;

fn setup_background(
    mut commands: Commands,
    mut materials: ResMut<Assets<StarMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(10000., 10000., 0.).into())
                .into(),
            material: materials.add(StarMaterial::default()),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        },
        Background {},
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(Material2dPlugin::<StarMaterial>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_background)
        .add_startup_system(setup_graphics)
        .add_startup_system(add_player)
        .add_system(move_player)
        .add_system(move_camera)
        .run();
}

fn move_player(mut query: Query<&mut ExternalImpulse, With<Player>>, keys: Res<Input<KeyCode>>) {
    let mut impulse = query.single_mut();

    let mut new_impulse = Vec2::new(0., 0.);

    if keys.pressed(KeyCode::W) {
        new_impulse.y += 10000.0;
    }
    if keys.pressed(KeyCode::A) {
        new_impulse.x -= 10000.0;
    }
    if keys.pressed(KeyCode::S) {
        new_impulse.y -= 10000.0;
    }
    if keys.pressed(KeyCode::D) {
        new_impulse.x += 10000.0;
    }

    impulse.impulse = new_impulse;
}

fn move_camera(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Background>, Without<Player>)>,
    mut bg_query: Query<
        (&mut Transform, &Handle<StarMaterial>),
        (With<Background>, Without<Camera2d>, Without<Player>),
    >,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>, Without<Background>)>,
    mut star_materials: ResMut<Assets<StarMaterial>>,
) {
    let translation = player_query.single().translation;
    let mut camera = camera_query.single_mut();
    let (mut bg_trans, bg_mat) = bg_query.single_mut();
    camera.translation = translation;
    bg_trans.translation.x = translation.x;
    bg_trans.translation.y = translation.y;
    let mat = star_materials.get_mut(bg_mat).unwrap();

    // Negate y translation because coordinate systems differ
    mat.player_position = Vec2::new(translation.x, -translation.y);
}

fn add_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(ShipComponent::new(&asset_server))
        .insert(Player {});
}

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct ShipComponent {
    spec: ShipComponentSpec,
    sprite: SpriteBundle,
    body: (RigidBody, Collider, ExternalImpulse, GravityScale),
}

impl ShipComponent {
    fn new(asset_server: &Res<AssetServer>) -> ShipComponent {
        ShipComponent {
            spec: ShipComponentSpec::Root,
            sprite: SpriteBundle {
                texture: asset_server.load("ship/base.png"),
                ..default()
            },
            body: (
                RigidBody::Dynamic,
                Collider::cuboid(20., 20.),
                ExternalImpulse {
                    impulse: Vec2::new(0., 0.),
                    torque_impulse: 0.,
                },
                GravityScale(0.0),
            ),
        }
    }
}

#[derive(Component)]
enum ShipComponentSpec {
    Root,
}

#[derive(AsBindGroup, Clone, TypeUuid, Default)]
#[uuid = "485518f8-a23a-4e1f-83b0-4c8caa7ed319"]
struct StarMaterial {
    #[uniform(0)]
    player_position: Vec2,
}

impl Material2d for StarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/stars.wgsl".into()
    }
}
