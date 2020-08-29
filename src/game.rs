use amethyst::{
	assets::{AssetStorage, Loader, Handle},
	core::transform::Transform,
	ecs::{Component, DenseVecStorage},
	prelude::*,
	renderer::{
		Camera, ImageFormat, SpriteRender,
		SpriteSheet, SpriteSheetFormat, Texture
	},
};
use amethyst::core::math::{Vector3};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    	let world = data.world;

		let sprite_sheet_handle = load_sprite_sheet(world);

    	initialise_warriors(world, sprite_sheet_handle);
    	initialise_camera(world);

    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Player {
	First,
	Second,
}

pub struct Warrior {
	pub player: Player,
	pub velocity: Vector3<f32>,
	pub size: f32, 
}

impl Warrior {
	fn new(player: Player) -> Warrior {
		Warrior {
			player: player,
			velocity: Vector3::new(0.0, 0.0, 0.0),
			size: 50.0,
		}
	}
}

impl Component for Warrior {
	type Storage = DenseVecStorage<Self>;
}


fn initialise_camera(world: &mut World) {
	// Cover the whole arena
	let mut transform = Transform::default();
	transform.set_translation_xyz(
		ARENA_WIDTH * 0.5,
		ARENA_HEIGHT * 0.5,
		1.0
	);

	world
		.create_entity()
		.with(Camera::standard_2d(ARENA_WIDTH*1.5, ARENA_HEIGHT*1.5))
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
