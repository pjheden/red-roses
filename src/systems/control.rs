use amethyst::{
	derive::SystemDesc,
	ecs::{Join, System, SystemData, WriteStorage, Read},
	input::{InputHandler, StringBindings},
};
use amethyst::core::math::{Vector3};

use crate::game::{Warrior, Player};


#[derive(SystemDesc)]
pub struct ControlSystem;

impl<'s> System<'s> for ControlSystem {
	type SystemData = (
		WriteStorage<'s, Warrior>,
		Read<'s, InputHandler<StringBindings>>,
	);

	fn run(&mut self, (mut warriors, input): Self::SystemData) {
        for warrior in (&mut warriors).join() {
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
                let ud_scaled_amount = warrior.movement_speed * ud_mv_amount as f32;
                let lr_scaled_amount = warrior.movement_speed * lr_mv_amount as f32;
                warrior.velocity = Vector3::new(lr_scaled_amount, ud_scaled_amount, 0.0);
            }

        }
    }
}
