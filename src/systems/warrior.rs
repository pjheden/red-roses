use amethyst::{
    core::timing::Time,
    core::{Transform},
	derive::SystemDesc,
	ecs::{Join, System, SystemData, WriteStorage,
		ReadStorage, Read},
	input::{InputHandler, StringBindings},
};
use amethyst::core::math::{Vector3};
use std::f32::consts::PI;

use crate::game::{Warrior, Player};


#[derive(SystemDesc)]
pub struct WarriorSystem;

impl<'s> System<'s> for WarriorSystem {
	type SystemData = (
		WriteStorage<'s, Transform>,
		ReadStorage<'s, Warrior>,
		Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
	);

	fn run(&mut self, (mut transforms, warriors, input, time): Self::SystemData) {
        for (warrior, transform) in (&warriors, &mut transforms).join() {
            transform.prepend_translation(warrior.velocity * time.delta_seconds());
            let (should_rotate, target_angle) = get_target_angle(warrior.velocity);
                let r_speed = 10.0;
                if should_rotate {
                    let rot = transform.rotation().angle();
                    // debug rotation
                    // println!("rot: {}, target: {}", rot, target_angle-rot);
                    transform.prepend_rotation_z_axis((target_angle-rot) * r_speed * time.delta_seconds());
                }
        }
    }
}

fn get_target_angle(mv: Vector3<f32>) -> (bool, f32) {
    // Go clockwise, starting with right
    if mv.x > 0.0 && mv.y == 0.0 {
        return (true, 0.0);
    }
    if mv.x > 0.0 && mv.y < 0.0 {
        return (true, PI/4.0);
    }
    if mv.x == 0.0 && mv.y < 0.0 {
        return (true, PI/2.0);
    }
    if mv.x < 0.0 && mv.y < 0.0 {
        return (true, PI*3.0/4.0);
    }
    if mv.x < 0.0 && mv.y == 0.0 {
        return (true, PI);
    }
    // TODO fix bug with bad rotation
    if mv.x < 0.0 && mv.y > 0.0 {
        return (true, PI*3.0/4.0);
    }
    if mv.x == 0.0 && mv.y > 0.0 {
        return (true, PI/2.0);
    }
    if mv.x > 0.0 && mv.y > 0.0 {
        return (true, PI/4.0);
    }

    return (false, 0.0);
}



