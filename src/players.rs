/*
 * @Author: 1454164915@qq.com 1454164915@qq.com
 * @Date: 2024-11-13 14:13:26
 * @LastEditors: 1454164915@qq.com 1454164915@qq.com
 * @LastEditTime: 2024-11-13 14:26:47
 * @FilePath: /ken/src/players.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE;
 */
use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use super::{Position, Player, TileType, xy_idx, State};
use std::cmp::{min,max};


pub fn try_move_player(delta_x: i32, delta_y:i32, ecs: &mut World){
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    let map = ecs.fetch::<Vec<TileType>>();


    for(_player, pos) in (&mut players, &mut positions).join(){
        let destination = xy_idx(pos.x + delta_x, pos.y+delta_y);
        if map[destination] != TileType::Wall{
        pos.x = min(79, max(0, pos.x+delta_x));
        pos.y = min(49, max(0, pos.x+delta_y));
        }
    }


}


pub fn player_input(gs: &mut State, ctx: &mut Rltk){
    // movement
    match ctx.key{
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}