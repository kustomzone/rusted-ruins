
use array2d::*;
use common::piece_pattern::*;
use common::objholder::EffectIdx;
use game::{Game, InfoGetter};
use game::view::ViewMap;
use game::frequent_tex::Overlay;

pub enum FogPattern {
    None,
    Fog(EffectIdx),
    PiecePattern(EffectIdx, PiecePattern),
}

pub fn view_fog(game: &Game, p: Vec2d) -> FogPattern {
    let view_map = &game.view_map;

    if view_map.get_tile_visible(p) {
        let mut piece_pattern_flags = PiecePatternFlags::new();
        for dir in &Direction::EIGHT_DIRS {
            piece_pattern_flags.set(*dir, view_map.get_tile_visible(p + dir.as_vec()));
        }

        let pp = piece_pattern_flags.to_piece_pattern(5);
        if pp == PiecePattern::SURROUNDED {
            FogPattern::None
        } else {
            FogPattern::PiecePattern(game.frequent_tex.overlay_idx(Overlay::Fog), pp)
        }
    } else {
        FogPattern::Fog(game.frequent_tex.overlay_idx(Overlay::Fog))
    }
}

pub fn all(game: &Game) -> Option<EffectIdx> {
    // If current map is indoor, don't draw night overlay
    if !game.gd.is_open_air(game.gd.get_current_mapid()) {
        return None;
    }
    
    let hour = game.gd.time.hour();
    let minute = game.gd.time.minute();
    let dawn_hour = 5;
    let dusk_hour = 18;
    assert!(dawn_hour < dusk_hour);

    if dawn_hour < hour && hour < dusk_hour { // Daytime
        None
    } else if hour == dawn_hour {
        Some(game.frequent_tex.overlay_idx(twilight(minute)))
    } else if hour == dusk_hour {
        Some(game.frequent_tex.overlay_idx(twilight(60 - minute)))
    } else { // Night
        Some(game.frequent_tex.overlay_idx(Overlay::Night))
    }
}

fn twilight(minute: u32) -> Overlay {
    if minute < 20 {
        Overlay::Twilight0
    } else if minute < 40 {
        Overlay::Twilight1
    } else {
        Overlay::Twilight2
    }
}

