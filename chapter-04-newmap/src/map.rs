use rltk::{Rltk, RGB};
use crate::rect::Rect;
use std::cmp::{min, max};

#[derive(PartialEq, Copy, Clone, Debug)]
pub(crate) enum TileType {
    Wall,
    Floor,
}

//根据坐标获取 地图数组中的索引位置
pub(crate) fn xy_idx(x: i32, y: i32) -> usize {
    //一行 80格 例如 x: 3 y: 5  表示 第三行第五个
    (y as usize * 80) + x as usize
}

///制作一个带有固体边界和400个随机放置的墙壁的地图
///糟糕透了
pub(crate) fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];

    //制造上下墙
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }

    //制造左右墙
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    //随机制造一些障碍
    let mut rng = rltk::RandomNumberGenerator::new();

    for _ in 0..400 {
        //掷骰子，采用经典的3d6类型格式:n是骰子的数量，die_type是骰子的大小。
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        if (x, y) != (40, 25) {
            map[xy_idx(x, y)] = TileType::Wall;
        }
    }
    map
}

//创建一些 地图和走廊
pub(crate) fn new_map_rooms_and_corridors() -> Vec<TileType> {
    let mut map = vec![TileType::Wall; 80*50];

    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(35, 15, 10, 15);

    apply_room_to_map(&room1, &mut map);
    apply_room_to_map(&room2, &mut map);
    map
}

//绘制地图
pub(crate) fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;
    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5,0.5,0.5), RGB::from_f32(0.,0.,0.), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0.,1.0,0.), RGB::from_f32(0.,0.,0.), rltk::to_cp437('#'));
            }
        }
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

//制作房间
fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            map[xy_idx(x, y)] = TileType::Floor
        }
    }

    apply_horizontal_tunnel(map, 31,35,23);
}

//创建水平连接通道
fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2) ..= max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}

//创建垂直连接通道
fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2) ..= max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}