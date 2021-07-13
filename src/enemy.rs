use std::f32::consts::PI;

use crate::{
	ActiveEnemies, ELaser, Enemy, Laser, Materials, PlayerOn, Speed, WinSize, MAX_ENEMIES, SCALE,
	TIME_STEP,
};
use bevy::{core::FixedTimestep, prelude::*};
use rand::{thread_rng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut bevy::prelude::AppBuilder) {
		app
			.add_system(elaser_movement.system())
			.add_system(enemy_movement.system())
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(1.0))
					.with_system(enemy_spawn.system()),
			)
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(1.0))
					.with_system(enemy_fire.system()),
			);
	}
}

fn enemy_spawn(
	mut commands: Commands,
	mut active_enemies: ResMut<ActiveEnemies>,
	win_size: Res<WinSize>,
	materials: Res<Materials>,
) {
	if active_enemies.0 < MAX_ENEMIES {
		// compute the random position
		let mut rng = thread_rng();
		let w_span = win_size.w / 2. - 100.;
		let h_span = win_size.h / 2. - 100.;
		let x = rng.gen_range(-w_span..w_span) as f32;
		let y = rng.gen_range(-h_span..h_span) as f32;

		// spawn enemy
		commands
			.spawn_bundle(SpriteBundle {
				material: materials.enemy.clone(),
				transform: Transform {
					translation: Vec3::new(x, y, 10.0),
					scale: Vec3::new(SCALE, SCALE, 1.),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Enemy)
			.insert(Speed::default());

		active_enemies.0 += 1;
	}
}

fn enemy_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Speed), With<Enemy>>) {
	let now = time.seconds_since_startup() as f32;
	let radius = (125., 100.);
	for (mut tf, speed) in query.iter_mut() {
		let x_org = tf.translation.x;
		let y_org = tf.translation.y;

		let angle = speed.0 * TIME_STEP * now % 360. / PI;
		let x_dst = radius.0 * angle.cos();
		let y_dst = radius.1 * angle.sin();

		// calculate the delta x/y and distance ratio (dr)
		let dx = x_org - x_dst;
		let dy = y_org - y_dst;
		let dd = (dx * dx + dy * dy).sqrt();
		let dr = TIME_STEP * speed.0 / dd;

		// calculate the final x/y (make sure to not overshoot)
		let x = x_org - dx * dr;
		let x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) };
		let y = y_org - dy * dr;
		let y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };
		// apply to tranformation
		tf.translation.x = x;
		tf.translation.y = y;
	}
}

fn enemy_fire(
	mut commands: Commands,
	materials: Res<Materials>,
	player_on: Res<PlayerOn>,
	enemy_query: Query<&Transform, With<Enemy>>,
) {
	if player_on.0 {
		for &tf in enemy_query.iter() {
			let x = tf.translation.x;
			let y = tf.translation.y;
			commands
				.spawn_bundle(SpriteBundle {
					material: materials.elaser.clone(),
					transform: Transform {
						translation: Vec3::new(x, y - 15., 0.),
						scale: Vec3::new(SCALE, -SCALE, 1.),
						..Default::default()
					},
					..Default::default()
				})
				.insert(ELaser)
				.insert(Laser)
				.insert(Speed::default());
		}
	}
}

fn elaser_movement(
	mut commands: Commands,
	win_size: Res<WinSize>,
	mut elaser_query: Query<(Entity, &Speed, &mut Transform), With<ELaser>>,
) {
	for (elaser_entity, speed, mut elaser_tf) in elaser_query.iter_mut() {
		let translation = &mut elaser_tf.translation;
		translation.y -= speed.0 * TIME_STEP;
		if translation.y < -win_size.h - 50. {
			commands.entity(elaser_entity).despawn();
		}
	}
}
