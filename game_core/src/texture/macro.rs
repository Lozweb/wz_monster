#[macro_export]
macro_rules! make_weapon_texture {
    ($weapon_type:expr, $start_min:expr, $frame_size:expr) => {
        texture(
            WEAPON_LAYOUT_SIZE,
            $start_min,
            WEAPONS,
            $weapon_type,
            WEAPON_FRAME_COUNT,
            $frame_size,
            WEAPON_FRAME_PADDING,
            |layout, typ, path| WeaponTextureEntity {
                texture_atlas_layout: layout,
                animation_indices: AnimationIndices { first: 0, last: WEAPON_FRAME_COUNT - 1 },
                texture_path: path,
                weapon_texture_type: typ,
            }
        )
    };
}
#[macro_export]
macro_rules! make_player_texture {
    ($weapon_type:expr, $layout_size:expr, $path:expr, $start_min:expr, $frame_size:expr) => {
        texture(
            $layout_size,
            $start_min,
            $path,
            $weapon_type,
            PLAYER_FRAME_COUNT,
            $frame_size,
            PLAYER_FRAME_PADDING,
            |layout, typ, path| PlayerTextureEntity {
                texture_atlas_layout: layout,
                animation_indices: AnimationIndices { first: 0, last: PLAYER_FRAME_COUNT - 1 },
                texture_path: path,
                player_texture_type: typ,
            }
        )
    };
}

#[macro_export]
macro_rules! make_weapon_fx_texture {
    ($weapon_type:expr, $start_min:expr, $frame_size:expr) => {
        texture(
            WEAPON_FX_LAYOUT_SIZE,
            $start_min,
            WEAPONS_FX,
            $weapon_type,
            WEAPON_FX_FRAME_COUNT,
            $frame_size,
            WEAPON_FX_FRAME_PADDING,
            |layout, typ, path| WeaponFxTextureEntity {
                texture_atlas_layout: layout,
                animation_indices: AnimationIndices { first: 0, last: WEAPON_FX_FRAME_COUNT - 1 },
                texture_path: path,
                weapon_fx_texture_type: typ,
            }
        )
    };
}