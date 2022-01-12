// #![allow(unused)] // silence unused warnings while learning

mod enemy;
mod player;

use std::collections::HashSet;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use enemy::EnemyPlugin;
use player::PlayerPlugin;

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const SCALE: f32 = 0.5;
//const TIME_STEP: f32 = 1. / 60.;
const TIME_STEP: f32 = 1. / 180.;
const MAX_ENEMIES: u32 = 4;
const MAX_FORMATION_MEMBERS: u32 = 2;
const PLAYER_RESPAWN_DELAY: f64 = 2.;

// region:    Resources
pub struct Materials {
	//player: Handle<ColorMaterial>,   expected struct `bevy::prelude::Image`, found struct `bevy::prelude::ColorMaterial`  Bevy 0.6
	player: Handle<Image>,
	//player_laser: Handle<ColorMaterial>,
	player_laser: Handle<Image>,
	//enemy: Handle<ColorMaterial>,
	enemy: Handle<Image>,
	//enemy_laser: Handle<ColorMaterial>,
	enemy_laser: Handle<Image>,
	explosion: Handle<TextureAtlas>,
}
struct WinSize {
	#[allow(unused)]
	w: f32,
	h: f32,
}
struct ActiveEnemies(u32);

struct PlayerState {
	on: bool,
	last_shot: f64,
}
impl Default for PlayerState {
	fn default() -> Self {
		Self {
			on: false,
			last_shot: 0.,
		}
	}
}
impl PlayerState {
	fn shot(&mut self, time: f64) {
		self.on = false;
		self.last_shot = time;
	}
	fn spawned(&mut self) {
		self.on = true;
		self.last_shot = 0.
	}
}
// endregion: Resources

// region:    Components
#[derive(Component)]
struct Laser;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerReadyFire(bool);

#[derive(Component)]
struct FromPlayer;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct FromEnemy;

#[derive(Component)]
struct Explosion;

#[derive(Component)]
struct ExplosionToSpawn(Vec3);

#[derive(Component)]
struct Speed(f32);
impl Default for Speed {
	fn default() -> Self {
		Self(500.)
	}
}
// endregion: Components

fn main() {
	//App::build()     https://bevyengine.org/learn/book/migration-guides/0.5-0.6/
	App::new()
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
		.add_system(player_laser_hit_enemy.system())
		.add_system(enemy_laser_hit_player.system())
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
		//player: materials.add(asset_server.load(PLAYER_SPRITE).into()),
		player: asset_server.load(PLAYER_SPRITE).into(),

		//player_laser: materials.add(asset_server.load(PLAYER_LASER_SPRITE).into()),   expected struct `bevy::prelude::Image`, found struct `bevy::prelude::ColorMaterial`
		player_laser: asset_server.load(PLAYER_LASER_SPRITE).into(),

		//enemy: materials.add(asset_server.load(ENEMY_SPRITE).into()),
		enemy: asset_server.load(ENEMY_SPRITE).into(),

		//enemy_laser: materials.add(asset_server.load(ENEMY_LASER_SPRITE).into()),
		enemy_laser: asset_server.load(ENEMY_LASER_SPRITE).into(),

		explosion: texture_atlases.add(texture_atlas),
	});
	commands.insert_resource(WinSize {
		w: window.width(),
		h: window.height(),
	});

	// position window
	// Commented out - when recording tutorial (place as you see fit)
	// window.set_position(IVec2::new(3870, 4830));
}

fn player_laser_hit_enemy(
	mut commands: Commands,
	laser_query: Query<(Entity, &Transform, &Sprite), (With<Laser>, With<FromPlayer>)>,
	enemy_query: Query<(Entity, &Transform, &Sprite), With<Enemy>>,
	mut active_enemies: ResMut<ActiveEnemies>,
) {
	let mut enemies_blasted: HashSet<Entity> = HashSet::new();

	for (laser_entity, laser_tf, laser_sprite) in laser_query.iter() {
		for (enemy_entity, enemy_tf, enemy_sprite) in enemy_query.iter() {

			//let laser_scale = Vec2::from(laser_tf.scale);                  the trait `From<bevy::prelude::Vec3>` is not implemented for `bevy::prelude::Vec2`
			let laser_scale = Vec2::new(laser_tf.scale.x, laser_tf.scale.y);

			//let enemy_scale = Vec2::from(enemy_tf.scale);   Break on Bevy 0.6
			//let enemy_scale = (enemy_tf.scale.x, enemy_tf.scale.y).to_vec2();      to_vec2()  method not found in `(f32, f32)`
			let enemy_scale = Vec2::new(enemy_tf.scale.x, enemy_tf.scale.y);


			let collision = collide(
				laser_tf.translation,
				//laser_sprite.size * laser_scale,
				//laser_sprite.custom_size * laser_scale,       cannot multiply `std::option::Option<bevy::prelude::Vec2>` by `bevy::prelude::Vec2`
				//laser_sprite.custom_size * Some(laser_scale),    cannot multiply `std::option::Option<bevy::prelude::Vec2>` by `std::option::Option<bevy::prelude::Vec2>`
				laser_sprite.custom_size.unwrap() * laser_scale, 
				enemy_tf.translation,
				//enemy_sprite.size * enemy_scale,
				//enemy_sprite.custom_size * enemy_scale,
				enemy_sprite.custom_size.unwrap() * enemy_scale,
			);

			if let Some(_) = collision {
				if enemies_blasted.get(&enemy_entity).is_none() {
					// remove the enemy
					commands.entity(enemy_entity).despawn();
					active_enemies.0 -= 1;

					// spawn explosion to spawn
					commands
						.spawn()
						.insert(ExplosionToSpawn(enemy_tf.translation.clone()));

					enemies_blasted.insert(enemy_entity);
				}

				// remove the laser
				commands.entity(laser_entity).despawn();
			}
		}
	}
}

fn enemy_laser_hit_player(
	mut commands: Commands,
	mut player_state: ResMut<PlayerState>,
	time: Res<Time>,
	laser_query: Query<(Entity, &Transform, &Sprite), (With<Laser>, With<FromEnemy>)>,
	player_query: Query<(Entity, &Transform, &Sprite), With<Player>>,
) {

	//if let Ok((player_entity, player_tf, player_sprite)) = player_query.single() {       expected tuple, found enum `Result`  ...   this expression has type `(bevy::prelude::Entity, &bevy::prelude::Transform, &bevy::prelude::Sprite)`
	if let Ok((player_entity, player_tf, player_sprite)) = player_query.get_single() {
		
		//let player_size = player_sprite.size * Vec2::from(player_tf.scale.abs());
		//let player_size = player_sprite.custom_size.unwrap() * Vec2::from(player_tf.scale.abs());    the trait `From<bevy::prelude::Vec3>` is not implemented for `bevy::prelude::Vec2`
		//let player_size = player_sprite.custom_size.unwrap() * Vec2::from(player_tf.scale.abs());
		let player_size = player_sprite.custom_size.unwrap() * Vec2::new(player_tf.scale.x.abs(), player_tf.scale.y.abs());


		// for each enemy laser
		for (laser_entity, laser_tf, laser_sprite) in laser_query.iter() {
			//let laser_size = laser_sprite.size * Vec2::from(laser_tf.scale.abs());
			let laser_size = laser_sprite.custom_size.unwrap() * Vec2::new(laser_tf.scale.x.abs(), laser_tf.scale.y.abs());

			// compute the collision
			let collision = collide(
				laser_tf.translation,
				laser_size,
				player_tf.translation,
				player_size,
			);
			// process collision
			if let Some(_) = collision {
				// remove the player
				commands.entity(player_entity).despawn();
				player_state.shot(time.seconds_since_startup());
				// remove the laser
				commands.entity(laser_entity).despawn();
				// spawn the ExplosionToSpawn entity
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
				
				transform: Transform {
					translation: explosion_to_spawn.0,
					..Default::default()
				},
				texture_atlas: materials.explosion.clone(),
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

			//if sprite.index == texture_atlas.textures.len() as u32 {       expected `usize`, found `u32`            no implementation for `usize == u32`
			if sprite.index == texture_atlas.textures.len() {
				commands.entity(entity).despawn();
			}
		}
	}
}
