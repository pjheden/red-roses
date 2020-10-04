use amethyst::{
    core::{Transform},
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
};

use std::vec::Vec;

use crate::game::{Warrior};

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Warrior>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut warriors, transforms): Self::SystemData) {
        let mut colliding_warriors: std::vec::Vec<u8> = Vec::new();
        let mut ind: u8 = 0;
        for (warrior, transform) in (&warriors, &transforms).join() {
            let warrior_x = transform.translation().x;
            let warrior_y = transform.translation().y;
            // Check if we are outside the arena
            // TODO

            for (w, t) in (&warriors, &transforms).join() {
                if warrior.player == w.player {
                    continue;
                }
                let o_warrior_x = t.translation().x;
                let o_warrior_y = t.translation().y;
                let radius = (warrior_x-o_warrior_x).powf(2.0)+ (warrior_y-o_warrior_y).powf(2.0);
                if radius < warrior.size {
                    // println!("collide {:?} {:?}", &warrior.player, &w.player);
                    colliding_warriors.push(ind);
                }

            }
            ind = ind + 1;
        }

        ind = 0;
        for warrior in (&mut warriors).join() {
            for i in 0..colliding_warriors.len() {
                if ind == colliding_warriors[i] {
                    // Set negative warrior velocity
                    warrior.velocity.x = warrior.velocity.x  * -1.0;
                    warrior.velocity.y = warrior.velocity.y  * -1.0;
                }
            }
            ind = ind + 1;
        }
    }
}
