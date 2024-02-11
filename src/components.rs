use crate::*;
use bevy::math::vec2;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default, Debug)]
pub struct Acceleration(pub Vec2);

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    player: Player,
    collider_bundle: ColliderBundle,
    sprites: Anim,
}

#[derive(Default, Bundle)]
pub struct Anim {
    pub sprites: SpriteSheetBundle,
    pub indices: AnimIndices,
    pub timer: AnimTimer,
}

#[derive(Component, Default)]
pub struct AnimTimer(pub Timer);

#[derive(Component, Default)]
pub struct AnimIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Default, Component)]
pub struct Sprites(SpriteSheetBundle);

#[derive(Bundle, Default, LdtkIntCell)]
pub struct ColliderBundle {
    rigid_body: RigidBody,
    collider: Collider,
    velocity: LinearVelocity,
    acceleration: Acceleration,
    friction: Friction,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(value: &EntityInstance) -> Self {
        match value.identifier.as_str() {
            "Player" => Self {
                rigid_body: RigidBody::Dynamic,
                collider: Collider::cuboid(16., 16.),
                friction: Friction::new(5.0),
                ..default()
            },

            other => panic!("use of unregistered entity {}", other),
        }
    }
}

impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let texture_handle = asset_server.load("res/Characters/Basic Charakter Spritesheet.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, vec2(48., 48.), 4, 4, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let anim_indices = AnimIndices { first: 0, last: 3 };
        let anim = Anim {
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
        }
    }
}
