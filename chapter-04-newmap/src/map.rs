use rltk::{Rltk, RGB, RandomNumberGenerator};
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

//创建一些 地图和走廊
pub(crate) fn new_map_rooms_and_corridors() -> (Vec<Rect>,Vec<TileType>) {
    let mut map = vec![TileType::Wall; 80*50];

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, 80 -w -1) -1;
        let y = rng.roll_dice(1, 50 -h -1) -1;
        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }
        if ok {
            apply_room_to_map(&new_room, &mut map);
            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() -1].center();
                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                }else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }

    (rooms,map)
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

//制作房间 需要一个 起点坐标，然后 宽 和 高 可推算出 四个点的坐标
fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            map[xy_idx(x, y)] = TileType::Floor
        }
    }
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