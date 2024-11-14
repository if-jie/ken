/*
 * @Author: 1454164915@qq.com 1454164915@qq.com
 * @Date: 2024-11-13 14:13:14
 * @LastEditors: 1454164915@qq.com 1454164915@qq.com
 * @LastEditTime: 2024-11-14 14:36:06
 * @FilePath: /ken/src/components.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use std::collections::VecDeque;

use specs::prelude::*;
use specs_derive::*;
use rltk::{RGB}

#[derive(Component)]
pub struct Position{
    x: i32,
    y: i32,
}

#[derive(Component)]
pub struct Renderable{
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
pub struct Player {}


#[derive(Component)]
pub struct Viewshed{
    pub visible_tiles : Vec<rltk::Point>,
    pub range: i32
}