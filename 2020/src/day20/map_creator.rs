use super::*;

pub fn create_image(input: &str) -> Vec<String> {
    let mut map = parse_input(input);
    let tile_dimension = map[0].map.lines().next().unwrap().len();
    let tile_dimension_max = tile_dimension - 1;
    let real_tile_dimension = tile_dimension - 2;
    let result_map = connect_map(&mut map);
    let map_dimension = result_map.len();
    // println!("map dim: {}, tile dim: {}", map_dimension, tile_dimension);

    let mut image_rows: Vec<String> = Vec::with_capacity(map_dimension * real_tile_dimension);
    for index_y in 0..map_dimension {
        for index_x in 0..map_dimension {
            // println!("x: {}, y: {}", index_x, index_y);
            let map_tile = &map[result_map[index_y][index_x]];
            let map_rows = map_tile.map.lines().collect::<Vec<&str>>();
            for index_map_y in 1..tile_dimension_max {
                if index_x == 0 {
                    image_rows.push(String::with_capacity(map_dimension * tile_dimension_max));
                }
                let slice = &map_rows[index_map_y][1..tile_dimension_max];
                image_rows[index_y * tile_dimension_max + index_map_y - 1 - index_y]
                    .push_str(slice);
            }
        }
    }

    // println!("{}", image_rows.join("\n"));
    return image_rows;
}
