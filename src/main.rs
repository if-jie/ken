/*
 * @Author: 1454164915@qq.com 1454164915@qq.com
 * @Date: 2024-11-12 23:20:00
 * @LastEditors: 1454164915@qq.com 1454164915@qq.com
 * @LastEditTime: 2024-11-14 13:33:52
 * @FilePath: /ken/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

// use specs::System;
// alloc::System,

mod components;
pub use components::*;
mod map;
pub use map::*;
mod players;

use players::*;
mod rect;
pub use rect::Rect;


// impl Component for Position {

//     type Storage = VecStorage<Self>;
    
// }
// #[derive(Component)]
// struct LeftMover{}

// 存储世界
pub struct State{
    pub ecs: World
}

impl State {
    fn run_systems(&mut self){
        // let mut lw = LeftWalker{};
        // lw.run_now(&self.ecs);
        self.ecs.maintain();

    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk){
        ctx.cls();
        
        player_input(self, ctx);
        self.run_systems();
        // huizhituxiang
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (& positions, & renderables).join(){
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }    
}

// struct LeftWalker{}


// impl<'a> System<'a> for LeftWalker {
//     type SystemData = (ReadStorage<'a, LeftMover>,
//                         WriteStorage<'a, Position>);
//     fn run (&mut self, (lefty, mut pos) : Self::SystemData) {
//         for (_lefty, pos) in (&lefty, &mut pos).join(){
//             pos.x -= 1;
//             if pos.x < 0  {pos.x = 79; }
//         }
//     }
// }

fn main() -> rltk::BError{

    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
                    .with_title("like tutorial")
                    .build()?;
    let mut gs = State{
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    // gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    gs.ecs.insert(new_map_rooms_and_corridors());


    gs.ecs
        .create_entity()
        .with(Position{x:40, y:25})
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();

        // for i in 0..10{
        //     gs.ecs.create_entity()
        //           .with(Position{x: i*7, y: 20})
        //           .with(Renderable{
        //             glyph: rltk::to_cp437('*'),
        //             fg: RGB::named(rltk::RED),
        //             bg: RGB::named(rltk::BLACK),
        //           })
        //           .with(LeftMover{})
        //           .build();
        // }
    rltk::main_loop(context, gs)

    // println!("Hello, world!");
}
