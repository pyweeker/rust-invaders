use std::f32::consts::PI;

use crate::{
	ActiveEnemies, ELaser, Enemy, Laser, Materials, PlayerState, Speed, WinSize, MAX_ENEMIES,
	MAX_FORMATION_MEMBERS, SCALE, TIME_STEP,
};
use bevy::{core::FixedTimestep, prelude::*};
use rand::{thread_rng, Rng};

pub struct EnemyPlugin;

// region:    Components
#[derive(Default, Clone)]
struct Formation {
	group_id: u32,
	offset: (f32, f32),
	radius: (f32, f32),
	start: (f32, f32),
	angle: f32,
}
// endregion: Components

// region:    Resources
#[derive(Default)]
struct FormationMaker {
	group_seq: u32,
	current_formation: Option<Formation>,
	current_formation_members: u32,
}
impl FormationMaker {
	fn make(&mut self, win_size: &WinSize) -> Formation {
		match (
			&self.current_formation,
			self.current_formation_members >= MAX_FORMATION_MEMBERS,
		) {
			// if first or previous formation full, create new formation
			(None, _) | (_, true) => {
				// compute the random position
				let mut rng = thread_rng();
				let h_span = win_size.h / 2. - 100.;
				let w_spawn = win_size.w / 4.;
				let x = if rng.gen::<bool>() {
					win_size.w
				} else {
					-win_size.w
				};
				let y = rng.gen_range(-h_span..h_span) as f32;
				let start = (x, y);

				// compute offset and radius
				let offset: (f32, f32) = (rng.gen_range(-w_spawn..w_spawn), rng.gen_range(0.0..h_span));
				let radius = (rng.gen_range(80.0..150.), 100.);
				let angle: f32 = (y - offset.0).atan2(x - offset.1);

				// create new formation
				self.group_seq += 1;
				let group_id = self.group_seq;
				let formation = Formation {
					group_id,
					offset,
					radius,
					start,
					angle,
				};

				// close, set, and return
				self.current_formation = Some(formation.clone());
				self.current_formation_members = 1;
				formation
			}
			// if still within the formation count
			(Some(tmpl), false) => {
				self.current_formation_members += 1;
				tmpl.clone()
			}
		}
	}
}

// endregion: Resources

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut bevy::prelude::AppBuilder) {
		app
			.insert_resource(FormationMaker::default())
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
	mut formation_maker: ResMut<FormationMaker>,
	win_size: Res<WinSize>,
	materials: Res<Materials>,
) {
	if active_enemies.0 < MAX_ENEMIES {
		// get the formation
		let formation = formation_maker.make(&win_size);
		let (x, y) = formation.start;

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
			.insert(Speed::default())
			.insert(formation);

		active_enemies.0 += 1;
	}
}

fn enemy_movement(mut query: Query<(&mut Transform, &Speed, &mut Formation), With<Enemy>>) {
	// for each enemy
	for (mut tf, speed, mut formation) in query.iter_mut() {
		let (x_offset, y_offset) = formation.offset;
		let (x_radius, y_radius) = formation.radius;
		let max_distance = TIME_STEP * speed.0;

		// get sprite x/y
		let x_org = tf.translation.x;
		let y_org = tf.translation.y;

		// next angle
		let dir = if formation.start.0 > 0. { 1. } else { -1. };
		let angle = formation.angle + dir * speed.0 * TIME_STEP / (x_radius.min(y_radius) * PI / 2.);

		// calculate the destination x/y
		let x_dst = x_radius * angle.cos() + x_offset;
		let y_dst = y_radius * angle.sin() + y_offset;

		// calculate the delta x/y and distance ratio
		let dx = x_org - x_dst;
		let dy = y_org - y_dst;
		let distance = (dx * dx + dy * dy).sqrt();
		let distance_ratio = if distance == 0. {
			0.
		} else {
			max_distance / distance
		};

		// calculate the final x/y (make sure to not overshoot)
		let x = x_org - dx * distance_ratio;
		let x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) };
		let y = y_org - dy * distance_ratio;
		let y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };

		// start to update the formation angle when sprite is close to formation
		if distance < max_distance * speed.0 / 20. {
			formation.angle = angle;
		}

		// apply to tranformation
		tf.translation.x = x;
		tf.translation.y = y;
	}
}

fn enemy_fire(
	mut commands: Commands,
	materials: Res<Materials>,
	playser_state: Res<PlayerState>,
	enemy_query: Query<&Transform, With<Enemy>>,
) {
	if playser_state.on {
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
