use rltk::{Rltk, RGB};

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

//制作一个带有固体边界和400个随机放置的墙壁的地图
pub(crate) fn new_map() -> Vec<TileType> {
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

    for  _ in 0..400 {
        //掷骰子，采用经典的3d6类型格式:n是骰子的数量，die_type是骰子的大小。
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        if (x, y) != (40, 25) {
            map[xy_idx(x, y)] = TileType::Wall;
        }
    }

    // println!("{:?}", map);
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