use rltk::{field_of_view, Point};
/*
 * @Author: 1454164915@qq.com 1454164915@qq.com
 * @Date: 2024-11-14 17:13:05
 * @LastEditors: 1454164915@qq.com 1454164915@qq.com
 * @LastEditTime: 2024-11-14 19:11:22
 * @FilePath: /ken/src/visibility.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use specs::prelude::*;
use super::{Viewshed, Position, Map, Player};
// use rltk::{Point};

pub struct VisibilitySystem{}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
                    WriteExpect<'a, Map>,
                    Entities<'a>,
                    WriteStorage<'a, Viewshed>,
                    WriteStorage<'a, Position>,
                    ReadStorage<'a, Player>);
    fn run(&mut self, data : Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent,viewshed,pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height );

                // If this is the player, reveal what they can see
                let _p : Option<&Player> = player.get(ent);
                if let Some(_p) = _p {
                    for t in map.visible_tiles.iter_mut() { *t = false };
                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}