use crate::*;
use bevy::math::vec2;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default, Debug)]
pub struct Acceleration(pub Vec2);

#[derive(Component, Default)]
pub struct SoilLayer;

#[derive(Default)]
pub struct TileNeighbor {
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    player: Player,
    collider_bundle: ColliderBundle,
    sprites: AnimBundle,
    state: PlayerState,
    grid_coord: GridCoords,
    worldly: Worldly,
}

#[derive(Default, Bundle)]
pub struct AnimBundle {
    pub sprites: SpriteSheetBundle,
    pub indices: AnimIndices,
    pub timer: AnimTimer,
}

#[derive(Component, Default, States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayerState {
    #[default]
    Normal,
    UsingTool,
}

#[derive(Component, Default)]
pub struct AnimTimer(pub Timer);

#[derive(Component, Default)]
pub struct ActionTimer(pub Timer, pub bool);

#[derive(Component, Default)]
pub struct AnimIndices {
    pub first: usize,
    pub last: usize,
    pub stopped: bool,
}

impl AnimIndices {
    pub fn spr_count(&self) -> usize {
        self.last - self.first + 1
    }

    pub fn set_row(&mut self, new_row: usize) {
        let count = self.spr_count();
        let first = count * new_row;
        if first == self.first {
            return;
        }
        self.first = first;
        self.last = first + count - 1;
    }

    pub fn row(&self) -> usize {
        self.first / self.spr_count()
    }

    pub fn offset(&self) -> usize {
        self.spr_count() * self.row()
    }
}

#[derive(Default, Component)]
pub struct Sprites(SpriteSheetBundle);

#[derive(Bundle, Default, LdtkIntCell)]
pub struct ColliderBundle {
    rigid_body: RigidBody,
    collider: Collider,
    velocity: LinearVelocity,
    acceleration: Acceleration,
    damping: LinearDamping,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(value: &EntityInstance) -> Self {
        match value.identifier.as_str() {
            "Player" => Self {
                rigid_body: RigidBody::Dynamic,
                collider: Collider::cuboid(16., 16.),
                damping: LinearDamping(0.5),
                ..default()
            },

            other => panic!("use of unregistered entity {}", other),
        }
    }
}

impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let texture_handle = asset_server.load("res/Characters/Basic Charakter Spritesheet.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, vec2(48., 48.), 4, 6, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let anim_indices = AnimIndices {
            first: 0,
            last: 3,
            stopped: false,
        };
        let anim = AnimBundle {
            sprites: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(anim_indices.first),
                ..Default::default()
            },
            indices: anim_indices,
            timer: AnimTimer(Timer::from_seconds(1. / 8., TimerMode::Repeating)),
        };
        Self {
            player: Player,
            collider_bundle: ColliderBundle::from(entity_instance),
            sprites: anim,
            state: PlayerState::default(),
            grid_coord: GridCoords::from_entity_info(entity_instance, layer_instance),
            worldly: Worldly::from_entity_info(entity_instance),
        }
    }
}
