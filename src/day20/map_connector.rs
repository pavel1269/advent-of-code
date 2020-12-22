use super::directions::*;
use super::map_tile::*;
use std::collections::HashMap;

struct SearchSpecification {
    direction: Directions,
    save_match: fn(
        &mut HashMap<i32, HashMap<i32, usize>>,
        &mut (&mut i32, &mut i32, &mut i32, &mut i32),
        usize,
    ),
}

pub fn connect_map(map: &mut Vec<MapTile>) -> Vec<Vec<usize>> {
    let (mut min_x, mut min_y) = (0, 0);
    let (mut max_x, mut max_y) = (0, 0);

    let search_specifications = vec![
        SearchSpecification {
            direction: Directions::Right,
            save_match: update_max_x,
        },
        SearchSpecification {
            direction: Directions::Left,
            save_match: update_min_x,
        },
        SearchSpecification {
            direction: Directions::Down,
            save_match: update_max_y,
        },
        SearchSpecification {
            direction: Directions::Up,
            save_match: update_min_y,
        },
    ];

    let mut used_tile_indexes: Vec<usize> = Vec::new();
    let mut result_map: HashMap<i32, HashMap<i32, usize>> = HashMap::new();
    result_map.insert(0, HashMap::new());
    result_map.get_mut(&0).unwrap().insert(0, 0);
    used_tile_indexes.push(0);

    for spec in search_specifications {
        discover(
            map,
            &mut result_map,
            &mut used_tile_indexes,
            &mut (&mut min_x, &mut max_x, &mut min_y, &mut max_y),
            &spec,
        );
    }
    let size = (i32::abs(max_x - min_x) + 1) as usize;
    debug_assert_eq!(size, (i32::abs(max_y - min_y) + 1) as usize);
    debug_assert_eq!(map.len(), size * size);

    // println!(
    //     "min x: {}, max x: {}, min y: {}, max y {}",
    //     min_x, max_x, min_y, max_y
    // );
    // println!("{:?}", result_map);
    // print_map(map, &result_map, &min_x, &max_x, &min_y, &max_y);
    for index_x in min_x..max_x + 1 {
        if index_x == 0 {
            continue;
        }
        // println!("{:?}", result_map);
        // print_map(map, &result_map, &min_x, &max_x, &min_y, &max_y);
        // print_real_map(map, &result_map, &min_x, &max_x, &min_y, &max_y);
        // println!("scanning x: {}", index_x);
        discover_y(
            map,
            &mut result_map,
            &mut used_tile_indexes,
            &min_y,
            &max_y,
            &index_x,
            Directions::Up,
            -1,
        );
        discover_y(
            map,
            &mut result_map,
            &mut used_tile_indexes,
            &min_y,
            &max_y,
            &index_x,
            Directions::Down,
            1,
        );
    }

    // println!("{:?}", result_map);
    // print_map(map, &result_map, &min_x, &max_x, &min_y, &max_y);
    // print_real_map(map, &result_map, &min_x, &max_x, &min_y, &max_y);

    let mut result: Vec<Vec<usize>> = Vec::with_capacity(size);
    for index_y in 0..size as i32 {
        result.push(Vec::with_capacity(size));
        let row = &mut result[index_y as usize];
        for index_x in 0..size as i32 {
            row.push(result_map[&(index_y + min_y)][&(index_x + min_x)]);
        }
    }

    // println!("{:?}", result);
    return result;
}

fn update_max_x(
    result_map: &mut HashMap<i32, HashMap<i32, usize>>,
    limits: &mut (&mut i32, &mut i32, &mut i32, &mut i32),
    index: usize,
) {
    *limits.1 += 1;
    result_map.get_mut(&0).unwrap().insert(*limits.1, index);
}

fn update_min_x(
    result_map: &mut HashMap<i32, HashMap<i32, usize>>,
    limits: &mut (&mut i32, &mut i32, &mut i32, &mut i32),
    index: usize,
) {
    *limits.0 -= 1;
    result_map.get_mut(&0).unwrap().insert(*limits.0, index);
}

fn update_max_y(
    result_map: &mut HashMap<i32, HashMap<i32, usize>>,
    limits: &mut (&mut i32, &mut i32, &mut i32, &mut i32),
    index: usize,
) {
    *limits.3 += 1;
    result_map.insert(*limits.3, HashMap::new());
    result_map.get_mut(limits.3).unwrap().insert(0, index);
}

fn update_min_y(
    result_map: &mut HashMap<i32, HashMap<i32, usize>>,
    limits: &mut (&mut i32, &mut i32, &mut i32, &mut i32),
    index: usize,
) {
    *limits.2 -= 1;
    result_map.insert(*limits.2, HashMap::new());
    result_map.get_mut(limits.2).unwrap().insert(0, index);
}

fn discover(
    map: &mut Vec<MapTile>,
    result_map: &mut HashMap<i32, HashMap<i32, usize>>,
    used_tile_indexes: &mut Vec<usize>,
    limits: &mut (&mut i32, &mut i32, &mut i32, &mut i32),
    search_specification: &SearchSpecification,
) {
    let len = map.len();
    let mut map_tile = &map[0];
    let mut index: usize = 1;
    loop {
        while used_tile_indexes.contains(&index) {
            index += 1;
        }

        if index >= len {
            break;
        }

        let edge = map_tile.edge(search_specification.direction);
        let map_tile_next = map.get(index).unwrap();
        // println!(
        //     "checking {} ({}) / {} against {}, edge: {}",
        //     index, map_tile_next.id, len, map_tile.id, edge
        // );
        for (next_edge_index, edges_next) in map_tile_next.edges.iter().enumerate() {
            let matches_normal = edge == &edges_next.0;
            let matches_mirrored = edge == &edges_next.1;
            if matches_normal || matches_mirrored {
                // println!(
                //     "Matched {} with {}, edge index: {}, mirror: {}, edge: {}",
                //     map_tile.id, map[index].id, next_edge_index, matches_mirrored, edge
                // );
                map[index].set_map_way(
                    next_edge_index,
                    matches_mirrored,
                    search_specification.direction,
                );
                map_tile = &map[index];

                (search_specification.save_match)(result_map, limits, index);
                used_tile_indexes.push(index);
                index = 0;
                break;
            }
        }

        index += 1;
    }
}

fn discover_y(
    map: &mut Vec<MapTile>,
    result_map: &mut HashMap<i32, HashMap<i32, usize>>,
    used_tile_indexes: &mut Vec<usize>,
    min_y: &i32,
    max_y: &i32,
    index_x: &i32,
    direction: Directions,
    index_y_change: i32,
) {
    let map_index = result_map.get(&0).unwrap().get(index_x).unwrap();
    let mut map_tile = &map[*map_index];
    let len = map.len();
    let mut index: usize = 1;
    let mut index_y = index_y_change;
    loop {
        // println!(
        //     "index {}, index_x {}, index_y {}, min_y {}",
        //     index, index_x, index_y, min_y
        // );
        while used_tile_indexes.contains(&index) {
            index += 1;
        }

        if index_y < *min_y || index_y > *max_y {
            break;
        }
        if index >= len {
            println!("{:?}", result_map);
            println!("{:?}", used_tile_indexes);
            panic!();
        }

        let map_tile_next = map.get(index).unwrap();
        let edge = map_tile.edge(direction);
        // println!(
        //     "checking {} ({}) / {} against {}, edge: {}",
        //     index, map_tile_next.id, len, map_tile.id, edge
        // );
        for (next_edge_index, edges_next) in map_tile_next.edges.iter().enumerate() {
            let matches_normal = edge == &edges_next.0;
            let matches_mirrored = edge == &edges_next.1;
            // println!("edge 0: {}, edge 1: {}", &edges_next.0, &edges_next.1);
            if matches_normal || matches_mirrored {
                // println!(
                //     "Matched {} with {}, edge index: {}, mirror: {}, edge: {}",
                //     map_tile.id, map[index].id, next_edge_index, matches_mirrored, edge
                // );
                map[index].set_map_way(
                    next_edge_index,
                    matches_mirrored,
                    direction,
                );
                map_tile = &map[index];

                result_map
                    .get_mut(&index_y)
                    .unwrap()
                    .insert(*index_x, index);
                index_y += index_y_change;
                used_tile_indexes.push(index);
                index = 0;
                break;
            }
        }

        index += 1;
    }
}

#[allow(dead_code)]
fn print_map(
    map: &Vec<MapTile>,
    result_map: &HashMap<i32, HashMap<i32, usize>>,
    min_x: &i32,
    max_x: &i32,
    min_y: &i32,
    max_y: &i32,
) {
    for index_y in *min_y..*max_y + 1 {
        let row = result_map.get(&index_y).unwrap();
        for index_x in *min_x..*max_x + 1 {
            match row.get(&index_x) {
                None => print!("_ "),
                Some(index) => {
                    let tile = &map[*index];
                    print!("{} ", tile.id);
                }
            }
        }
        println!("");
    }
}

#[allow(dead_code)]
fn print_real_map(
    map: &Vec<MapTile>,
    result_map: &HashMap<i32, HashMap<i32, usize>>,
    min_x: &i32,
    max_x: &i32,
    min_y: &i32,
    max_y: &i32,
) {
    let size_int = i32::abs(max_x - min_x) + 1;
    let size: usize = map[0].map.lines().last().unwrap().len();
    let mut placeholder = String::new();
    for _ in 0..size {
        placeholder.push(' ');
    }
    for index_y in *min_y..*max_y + 1 {
        let row = result_map.get(&index_y).unwrap();
        let mut row_tiles: Vec<Option<Vec<String>>> = Vec::new();
        for index_x in *min_x..*max_x + 1 {
            match row.get(&index_x) {
                None => {
                    row_tiles.push(None);
                }
                Some(index) => {
                    let map = &map[*index].map;
                    let lines = map.lines().map(|line| line.to_string()).collect::<Vec<String>>();
                    // println!("len: {}: {}", lines.len(), map);
                    row_tiles.push(Some(lines));
                }
            }
        }
        for index_row in 0..size {
            for index_x in 0..size_int {
                match &row_tiles[index_x as usize] {
                    None => {
                        print!("{}", placeholder);
                    },
                    Some(vec) => {
                        print!("{}", vec[index_row]);
                    },
                }
                print!(" ");
            }
            println!();
        }
        println!();
    }
}
