use crate::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn system_set() -> SystemSet {
    SystemSet::new().with_system(animate_player_sprite)
}

fn animate_player_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
    player: Res<player::Player>,
) {
    let (mut timer, mut sprite, texture_atlas_handle) = query
        .get_mut(player.sprite_entity.unwrap())
        .unwrap();

    timer.tick(time.delta());
    if player.move_direction == 0 {
        sprite.index = 8;
        return;
    }

    if timer.just_finished() {
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        sprite.index = (sprite.index + 1) % (texture_atlas.textures.len() - 1);
    }
}
