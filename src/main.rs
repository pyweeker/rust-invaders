// #![allow(unused)] // silence unused warnings while learning

mod enemy;
mod player;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use enemy::EnemyPlugin;
use player::PlayerPlugin;

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLASER_SPRITE: &str = "laser_a_01.png";
const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ELASER_SPRITE: &str = "laser_b_01.png";
const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;
const MAX_ENEMIES: u32 = 1;

// region:    Resources
pub struct Materials {
	player: Handle<ColorMaterial>,
	plaser: Handle<ColorMaterial>,
	enemy: Handle<ColorMaterial>,
	elaser: Handle<ColorMaterial>,
	explosion: Handle<TextureAtlas>,
}
struct WinSize {
	#[allow(unused)]
	w: f32,
	h: f32,
}
struct ActiveEnemies(u32);
// endregion: Resources

// region:    Components
struct Laser;
struct Player;
struct PlayerReadyFire(bool);
struct PLaser;

struct Enemy;
struct ELaser;

struct Explosion;
struct ExplosionToSpawn(Vec3);

struct Speed(f32);
impl Default for Speed {
	fn default() -> Self {
		Self(500.)
	}
}
// endregion: Components

fn main() {
	App::build()
		.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
		.insert_resource(WindowDescriptor {
			title: "Rust Invaders!".to_string(),
			width: 598.0,
			height: 676.0,
			..Default::default()
		})
		.insert_resource(ActiveEnemies(0))
		.add_plugins(DefaultPlugins)
		.add_plugin(PlayerPlugin)
		.add_plugin(EnemyPlugin)
		.add_startup_system(setup.system())
		.add_system(plaser_hit_enemy.system())
		.add_system(elaser_hit_player.system())
		.add_system(explosion_to_spawn.system())
		.add_system(animate_explosion.system())
		.run();
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut windows: ResMut<Windows>,
) {
	let window = windows.get_primary_mut().unwrap();

	// camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	// create the main resources
	let texture_handle = asset_server.load(EXPLOSION_SHEET);
	let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 4, 4);
	commands.insert_resource(Materials {
		player: materials.add(asset_server.load(PLAYER_SPRITE).into()),
		plaser: materials.add(asset_server.load(PLASER_SPRITE).into()),
		enemy: materials.add(asset_server.load(ENEMY_SPRITE).into()),
		elaser: materials.add(asset_server.load(ELASER_SPRITE).into()),
		explosion: texture_atlases.add(texture_atlas),
	});
	commands.insert_resource(WinSize {
		w: window.width(),
		h: window.height(),
	});

	// position window
	window.set_position(IVec2::new(3870, 4830));
}

fn plaser_hit_enemy(
	mut commands: Commands,
	laser_query: Query<(Entity, &Transform, &Sprite), With<PLaser>>,
	enemy_query: Query<(Entity, &Transform, &Sprite), With<Enemy>>,
	mut active_enemies: ResMut<ActiveEnemies>,
) {
	for (laser_entity, laser_tf, laser_sprite) in laser_query.iter() {
		for (enemy_entity, enemy_tf, enemy_sprite) in enemy_query.iter() {
			let laser_scale = Vec2::from(laser_tf.scale);
			let enemy_scale = Vec2::from(enemy_tf.scale);
			let collision = collide(
				laser_tf.translation,
				laser_sprite.size * laser_scale,
				enemy_tf.translation,
				enemy_sprite.size * enemy_scale,
			);

			if let Some(_) = collision {
				// remove the enemy
				commands.entity(enemy_entity).despawn();
				active_enemies.0 -= 1;

				// remove the laser
				commands.entity(laser_entity).despawn();

				// spawn explosion to spawn
				commands
					.spawn()
					.insert(ExplosionToSpawn(enemy_tf.translation.clone()));
			}
		}
	}
}

fn elaser_hit_player(
	mut commands: Commands,
	player_query: Query<(Entity, &Transform, &Sprite), With<Player>>,
	elaser_query: Query<(Entity, &Transform, &Sprite), With<ELaser>>,
) {
	if let Ok((player_entity, player_tf, player_sprite)) = player_query.single() {
		let player_size = player_sprite.size * Vec2::from(player_tf.scale.abs());

		for (elaser_entity, elaser_tf, elaser_sprite) in elaser_query.iter() {
			let elaser_size = elaser_sprite.size * Vec2::from(elaser_tf.scale.abs());

			let collision = collide(
				elaser_tf.translation,
				elaser_size,
				player_tf.translation,
				player_size,
			);

			if let Some(_) = collision {
				// remove the player
				commands.entity(player_entity).despawn();
				// remove the laser
				commands.entity(elaser_entity).despawn();
				// spawn ExplosionToSpawn
				commands
					.spawn()
					.insert(ExplosionToSpawn(player_tf.translation.clone()));
			}
		}
	}
}

fn explosion_to_spawn(
	mut commands: Commands,
	query: Query<(Entity, &ExplosionToSpawn)>,
	materials: Res<Materials>,
) {
	for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
		commands
			.spawn_bundle(SpriteSheetBundle {
				texture_atlas: materials.explosion.clone(),
				transform: Transform {
					translation: explosion_to_spawn.0,
					..Default::default()
				},
				..Default::default()
			})
			.insert(Explosion)
			.insert(Timer::from_seconds(0.05, true));

		commands.entity(explosion_spawn_entity).despawn();
	}
}

fn animate_explosion(
	mut commands: Commands,
	time: Res<Time>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut query: Query<
		(
			Entity,
			&mut Timer,
			&mut TextureAtlasSprite,
			&Handle<TextureAtlas>,
		),
		With<Explosion>,
	>,
) {
	for (entity, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
		timer.tick(time.delta());
		if timer.finished() {
			let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
			sprite.index += 1;
			if sprite.index == texture_atlas.textures.len() as u32 {
				commands.entity(entity).despawn()
			}
		}
	}
}
