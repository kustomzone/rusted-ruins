//! Functions to search objects in a map

use common::gamedata::*;

/// Search specified facility item.
pub fn search_facility<'a>(gd: &'a GameData, facility_type: &str) -> Option<&'a Item> {
    let il = gd.get_item_list(ItemListLocation::PLAYER);
    let mut facility_item = None;
    let mut quality = std::i8::MIN;

    for (item, _) in il.iter() {
        if let Some((f, q)) = item.obj().facility.as_ref() {
            if facility_type == f && *q > quality {
                facility_item = Some(item);
                quality = *q;
            }
        }
    }

    facility_item
}

/*
use crate::game::view::calc_visual_distance;

/// Search the nearest chara's position that has given Relationship on the current map.
pub fn search_nearest_target(
    gd: &GameData,
    center_cid: CharaId,
    rel: Relationship,
) -> Option<CharaId> {
    let map = gd.get_current_map();
    let chara = gd.chara.get(center_cid);

    let center = if let Some(center) = map.chara_pos(center_cid) {
        center
    } else {
        return None;
    };

    let mut target_cid = None;
    let mut min_distance = i32::max_value();

    for cid in map.iter_charaid() {
        if center_cid == *cid {
            continue;
        }
        if gd.chara.get(*cid).rel.relative(chara.rel) != rel {
            continue;
        }

        let pos = if let Some(pos) = map.chara_pos(*cid) {
            pos
        } else {
            continue;
        };

        let visual_distance = if let Some(visual_distance) = calc_visual_distance(map, center, pos)
        {
            visual_distance
        } else {
            continue;
        };

        if visual_distance <= chara.attr.view_range && visual_distance < min_distance {
            target_cid = Some(*cid);
            min_distance = visual_distance;
        }
    }

    target_cid
}
*/
