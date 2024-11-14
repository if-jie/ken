/*
 * @Author: 1454164915@qq.com 1454164915@qq.com
 * @Date: 2024-11-13 14:19:53
 * @LastEditors: 1454164915@qq.com 1454164915@qq.com
 * @LastEditTime: 2024-11-14 14:32:46
 * @FilePath: /ken/src/map.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use rltk::{Rltk, RGB, RandomNumberGenerator};
use std::cmp::{max,min};
use super::{Rect};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub struct Map {
    pub tiles : Vec<TileType>,
    pub rooms : Vec<Rect>,
    pub width : i32,
    pub height : i32
}

impl Map {
    pub fn xy_idx(x:i32, y: i32) -> usize{
        (y as usize * 80) + x as usize
    }

    fn apply_room_to_map(room: &Rect, map: &mut [TileType]){
        for y in room.y1+1 ..= room.y2{
            for x in room.x1 + 1 ..= room.x2 {
                map[xy_idx(x, y)] = TileType::Floor; 
            }
        }
    }

    fn apply_horizonal_tunnel(map: &mut [TileType], x1:i32,x2:i32, y:i32){
        for x in min(x1,x2) .. = max(x1, x2){
            let idx = xy_idx(x, y);
            if idx > 0 && idx < 80*50{
                map[idx as usize] = TileType::Floor;
            }
        }
    }
    
    fn apply_vertical_tunnel(map: &mut [TileType], y1:i32,y2:i32, x:i32){
        for y in min(y1,y2) .. = max(y1, y2){
            let idx = xy_idx(x, y);
            if idx > 0 && idx < 80*50{
                map[idx as usize] = TileType::Floor;
            }
        }
    }


///new  map
    pub fn new_map_rooms_and_corridors() -> Map {

        let mut map = Map { tiles: vec![TileType::Wall; 80*50], rooms: Vec::new(), width: 80, height: 50 };
        // let room1 = Rect::new(20, 15, 10, 15);
        // let room2 = Rect::new(35, 15, 10, 15);
        const  MAX_ROOMS: i32 = 30;
        const  MIN_SIZE: i32 = 6;
        const  MAX_SIZE: i32 = 10;
        
        let mut rng = RandomNumberGenerator::new();

        for i in 0..MAX_ROOMS{
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width-w-1) - 1;
            let y = rng.roll_dice(1, map.height-h-1) - 1;

            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;

            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room){ok = false}
            }

            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizonal_tunnel(prev_x,new_x,prev_y);
                        map.apply_vertical_tunnel(prev_y,new_y,new_x);
                    }else {
                        map.apply_horizonal_tunnel(prev_x,new_x,new_y);
                        map.apply_vertical_tunnel(prev_y,new_y,prev_x);

                    }
                }
                map.rooms.push(new_room);

            }
        }
        // apply_room_to_map(&room1, &mut map);
        // apply_room_to_map(&room2, &mut map);
            
        map
    }

}









/// Make a map with 400 randomly point as wall
pub fn new_map_test()->Vec<TileType>{
    let mut map = vec![TileType::Floor; 80*50];

    //make the bounder wall
    for x in 0..80{
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    //make the bounder wall
    for y in 0..50{
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // bunch of wall
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400{
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25){
            map[idx] = TileType::Wall;
        }
     }
    map
}


// ----------------------------------map----------------------------------------------

// 






pub fn draw_map(map: &[TileType], ctx: &mut Rltk){
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter(){
        match tile {
            TileType::Floor => {
                ctx.set(x,y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));

            }
            TileType::Wall =>{
                ctx.set(x,y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
 
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }

    }


}

///new map interesting s ===========================================
///
/// 
/// 
/// 
/// 
/// 







