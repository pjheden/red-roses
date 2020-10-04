use amethyst::{
    core::timing::Time,
    core::{Transform},
	derive::SystemDesc,
	ecs::{Entities, Join, System, SystemData, WriteStorage,
		ReadStorage, Read},
};

use crate::game::{Warrior, Arena};


#[derive(SystemDesc)]
pub struct ArenaSystem;

impl<'s> System<'s> for ArenaSystem {
	type SystemData = (
        Entities<'s>,
		WriteStorage<'s, Transform>,
		WriteStorage<'s, Warrior>,
		ReadStorage<'s, Arena>,
        Read<'s, Time>,
	);

	fn run(&mut self, (entities, mut transforms, warriors, _arena, _time): Self::SystemData) {
		use crate::{ARENA_HEIGHT, ARENA_WIDTH};

        for (entity, _warrior, transform) in (&entities, &warriors, &mut transforms).join() {
        	//TODO: make them fall down instead of insta dying?
        	// simple falldown - don't allow any movement, maybe just rotation. Kill after x seconds
        	let warrior_position = transform.translation(); // .translation_mut()
        	println!("{:?} - {:?}", warrior_position, ARENA_WIDTH);
        	if warrior_position.x > ARENA_WIDTH {
                let _ = entities.delete(entity);
        	} else if warrior_position.x < 0.0 {
                let _ = entities.delete(entity);
        	} else if warrior_position.y < 0.0 {
                let _ = entities.delete(entity);
        	} else if warrior_position.y > ARENA_HEIGHT {
                let _ = entities.delete(entity);
        	}
        }
    }
}


