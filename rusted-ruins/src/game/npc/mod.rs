//! Functions for NPC's AI and actions

pub mod map_search;

use super::action;
use super::{Game, InfoGetter};
use common::gamedata::*;
use geom::*;
use rng::*;
use rules::{npc_ai::*, RULES};

pub fn process_npc_turn(game: &mut Game, cid: CharaId) {
    let chara = game.gd.chara.get(cid);
    let ai = &chara.ai;
    let ai_rule = RULES.npc_ai.get(ai.kind);

    match ai_rule.move_kind {
        MoveKind::NoMove => (),
        MoveKind::Melee => {
            if gen_range(0..3) == 0 {
                move_to_nearest_enemy(game, cid);
                return;
            }
        }
        MoveKind::Wander => {
            if gen_bool(ai_rule.walk_prob) {
                random_walk(game, cid);
            }
            return;
        }
        MoveKind::Return => {
            let initial_pos = chara.ai.initial_pos;
            let pos = game.gd.chara_pos(cid).unwrap();
            if initial_pos != pos {
                if gen_range(0..1) == 0 {
                    let dir = geom::dir_by_2pos(pos, initial_pos);
                    action::try_move(game, cid, dir);
                    return;
                }
            } else {
                if gen_bool(ai_rule.walk_prob) {
                    random_walk(game, cid);
                }
            }
        }
    }
}

/// Move npc at random
fn random_walk(game: &mut Game, cid: CharaId) {
    let dir = Direction::new(
        *[HDirection::Left, HDirection::None, HDirection::Right]
            .choose(&mut get_rng())
            .unwrap(),
        *[VDirection::Up, VDirection::None, VDirection::Down]
            .choose(&mut get_rng())
            .unwrap(),
    );
    action::try_move(game, cid, dir);
}

/// Move npc to nearest enemy
fn move_to_nearest_enemy(game: &mut Game, cid: CharaId) {
    if let Some(target) = map_search::search_nearest_enemy(&game.gd, cid) {
        if let Some(pos) = game.gd.chara_pos(cid) {
            let dir = map_search::dir_to_chara(&game.gd, target, pos);
            action::try_move(game, cid, dir);
        }
    }
}
