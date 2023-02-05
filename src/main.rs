use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_rapier2d::prelude::*;

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StarMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1000000. })),
        material: materials.add(StarMaterial {}),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_background)
        .add_startup_system(add_player)
        .add_system(move_player)
        .add_system(move_camera)
        .run();
}

fn move_player(mut query: Query<&mut ExternalImpulse, With<Player>>, keys: Res<Input<KeyCode>>) {
    let mut impulse = query.single_mut();

    let mut new_impulse = Vec2::new(0., 0.);

    if keys.pressed(KeyCode::W) {
        new_impulse.y += 1000.0;
    }
    if keys.pressed(KeyCode::A) {
        new_impulse.x -= 1000.0;
    }
    if keys.pressed(KeyCode::S) {
        new_impulse.y -= 1000.0;
    }
    if keys.pressed(KeyCode::D) {
        new_impulse.x += 1000.0;
    }

    impulse.impulse = new_impulse;
}

fn move_camera(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    camera_query.single_mut().translation = player_query.single().translation;
}

fn add_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(ShipComponent::new(&asset_server))
        .insert(Player {});
}

#[derive(Component)]
struct Player {}

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
                transform: Transform {
                    scale: Vec3::new(4., 4., 1.),
                    ..default()
                },
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

#[derive(AsBindGroup, Clone, TypeUuid)]
#[uuid = "485518f8-a23a-4e1f-83b0-4c8caa7ed319"]
struct StarMaterial {}

impl Material for StarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/stars.wgsl".into()
    }
}
