use amethyst::{
	assets::{AssetStorage, Loader, Handle},
	core::transform::Transform,
	ecs::{Component, DenseVecStorage},
	prelude::*,
	renderer::{
		Camera, ImageFormat, SpriteRender,
		SpriteSheet, SpriteSheetFormat, Texture,
		palette::Srgba,
		debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
	},
};
use amethyst::core::math::{Vector3};
use crate::{ARENA_HEIGHT, ARENA_WIDTH};

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    	let world = data.world;

		let sprite_sheet_handle = load_sprite_sheet(world);

    	initialise_arena(world, sprite_sheet_handle.clone());
    	initialise_warriors(world, sprite_sheet_handle);
    	initialise_camera(world);

    }

    // fn update(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    // 	println!("update")
    // 	// Check if warrior is dead
    //         // if warrior.hp <= 0.0 {
    //         //    data.world.delete_entity(warrior).expect("Failed to delete entity. Was it already removed?");
    //         // }
    // }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Player {
	First,
	Second,
}

pub struct Warrior {
	pub player: Player,
	pub velocity: Vector3<f32>,
	pub movement_speed: f32, 
	pub rotation_speed: f32, 
	pub size: f32,
	pub hp:	f32,
}

impl Warrior {
	fn new(player: Player) -> Warrior {
		Warrior {
			player: player,
			velocity: Vector3::new(0.0, 0.0, 0.0),
			movement_speed: 50.0,
			rotation_speed: 10.0,
			size: 50.0,
			hp: 100.0,
		}
	}
}

impl Component for Warrior {
	type Storage = DenseVecStorage<Self>;
}

pub struct Arena {
	pub width: f32,
	pub height: f32,
	pub spriteindex: u8,
}

impl Arena {
	fn new() -> Arena {
		Arena {
			width: 100.0,
			height: 100.0,
			spriteindex: 2,
		}
	}
}

impl Component for Arena {
	type Storage = DenseVecStorage<Self>;
}


fn initialise_camera(world: &mut World) {
	// Cover the whole arena
	let mut transform = Transform::default();
	transform.set_translation_xyz(
		ARENA_WIDTH * 0.5,
		ARENA_HEIGHT * 0.5,
		10.0
	);

	world
		.create_entity()
		.with(Camera::standard_2d(ARENA_WIDTH*1.5, ARENA_HEIGHT*1.5)) //TODO: set to 3d camera, standard_3d
		.with(transform)
		.build();
}


fn initialise_warriors(world: &mut World,
	sprite_sheet_handle: Handle<SpriteSheet>) {
	// Assign sprites for warriors
	// 0 because warrior is the first sprite
	let first_sprite_render = SpriteRender::new(sprite_sheet_handle.clone(), 0);
	let second_sprite_render = SpriteRender::new(sprite_sheet_handle, 1);
	let mut left_transform = Transform::default();
	let mut right_transform = Transform::default();

	// Position the warriors
	let y = ARENA_HEIGHT * 0.5;
	left_transform.set_translation_xyz(ARENA_WIDTH / 10.0, y, 0.0);
	right_transform.set_translation_xyz(ARENA_WIDTH - ARENA_WIDTH / 10.0, y, 0.0);

	// Scale the textures
	let scale = 0.1;
	left_transform.set_scale(Vector3::new(scale,scale, 1.0));
	right_transform.set_scale(Vector3::new(scale,scale, 1.0));

	// left warrior
	world
		.create_entity()
		.with(first_sprite_render.clone())
		.with(Warrior::new(Player::First))
		.with(left_transform)
		.build();
	// right warrior
	world
		.create_entity()
		.with(second_sprite_render.clone())
		.with(Warrior::new(Player::Second))
		.with(right_transform)
		.build();
}

fn initialise_arena(world: &mut World,
	sprite_sheet_handle: Handle<SpriteSheet>) {
	// Assign sprites for warriors
	// 0 because warrior is the first sprite
	let arena_sprite_render = SpriteRender::new(sprite_sheet_handle.clone(), 2);
	// let stalk_sprite_render = SpriteRender::new(sprite_sheet_handle.clone(), 3);
	// let flower_sprite_render = SpriteRender::new(sprite_sheet_handle, 4);

	// let left_transform = Transform::default();
	// let right_transform = Transform::default();

	// Position the arena
	let mut arena_transform = Transform::default();
	arena_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, -1.0);
	// Scale arena_transform arena
	// TODO: This scale needs to match ARENA_WIDTH somehow..
	arena_transform.set_scale(Vector3::new(1.0, 1.0, 1.0));
	// Create arena entity
	world
		.create_entity()
		.with(arena_sprite_render.clone())
		.with(Arena::new())
		.with(arena_transform)
		.build();

	// DEBUG: draw arena square
    world.insert(DebugLines::new());
    world.insert(DebugLinesParams { line_width: 2.0 });
    let mut debug_lines_component = DebugLinesComponent::new();

	 debug_lines_component.add_rectangle_2d(
	   	[0.0, 0.0].into(),
	    [ARENA_WIDTH, ARENA_HEIGHT].into(),
	    4.0,
	    Srgba::new(1.0, 0.0, 0.2, 1.0),
	);

    world
            .create_entity()
            .with(debug_lines_component)
            .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load(
			"texture/full_spritesheet.png",
			ImageFormat::default(),
			(),
			&texture_storage,
		)
	};
	let loader = world.read_resource::<Loader>();
	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/full_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
