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
            let updown_movement = match warrior.player {
                Player::First => input.axis_value("0_updown"),
                Player::Second => input.axis_value("1_updown"),
            };
            let leftright_movement = match warrior.player {
                Player::First => input.axis_value("0_leftright"),
                Player::Second => input.axis_value("1_leftright"),
            };
            if let (Some(ud_mv_amount), Some(lr_mv_amount)) = (updown_movement, leftright_movement) {
                // TODO: set movement and rotatoin speed to a field of warrior
                let m_speed = 70.0;
                let ud_scaled_amount = m_speed * ud_mv_amount as f32;
                let lr_scaled_amount = m_speed * lr_mv_amount as f32;
                let movement_vector = Vector3::new(lr_scaled_amount, ud_scaled_amount, 0.0);
                transform.prepend_translation(movement_vector * time.delta_seconds());

                
                let (should_rotate, target_angle) = get_target_angle(movement_vector);
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



