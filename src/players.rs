/*
 * @Author: 1454164915@qq.com 1454164915@qq.com
 * @Date: 2024-11-13 14:13:26
 * @LastEditors: 1454164915@qq.com 1454164915@qq.com
 * @LastEditTime: 2024-11-14 18:49:59
 * @FilePath: /ken/src/players.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE;
 */
use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use super::{Position, Player, TileType, Map, State,Viewshed};
use std::cmp::{min,max};


pub fn try_move_player(delta_x: i32, delta_y:i32, ecs: &mut World){
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map: specs::shred::Fetch<'_, _> = ecs.fetch::<Map>();


    for(_player, pos,viewshed) in (&mut players, &mut positions, &mut viewsheds).join(){
        let destination = map.xy_idx(pos.x + delta_x, pos.y+delta_y);
        if map.tiles[destination] != TileType::Wall{
        pos.x = min(79, max(0, pos.x+delta_x));
        pos.y = min(49, max(0, pos.x+delta_y));
        
        viewshed.dirty = true;
        }
    }


}

pub fn player_input(gs: &mut State, ctx: &mut Rltk){
    // movement
    match ctx.key{
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4|
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),
            
            VirtualKeyCode::Right|
            VirtualKeyCode::Numpad6|
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),
            
            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8|
            VirtualKeyCode::K=> try_move_player(0, -1, &mut gs.ecs),
            
            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2|
            VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}