use specs_derive::Component;
use specs::prelude::*;

//定义位置坐标
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

//定义渲染组件
#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: rltk::RGB,
    pub bg: rltk::RGB,
}

//定义玩家
#[derive(Component, Debug)]
pub struct Player {}

//定义视线距离
#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

//让怪物思考
#[derive(Component, Debug)]
pub struct Monster {}