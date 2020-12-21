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

pub fn connect_map(map: &mut Vec<MapTile>) {
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
    debug_assert_eq!(i32::abs(max_x - min_x), i32::abs(max_y - min_y));
    debug_assert_eq!(
        map.len() as i32,
        (i32::abs(max_x - min_x) + 1) * (i32::abs(max_y - min_y) + 1)
    );

    // discover_max_x(map, &mut result_map, &mut used_tile_indexes, &mut max_x);
    // discover_min_x(map, &mut result_map, &mut used_tile_indexes, &mut min_x);
    // discover_max_y(map, &mut result_map, &mut used_tile_indexes, &mut max_y);
    // discover_min_y(map, &mut result_map, &mut used_tile_indexes, &mut min_y);
    println!(
        "min x: {}, max x: {}, min y: {}, max y {}",
        min_x, max_x, min_y, max_y
    );
    println!("{:?}", result_map);
    print_map(map, &result_map, &min_x, &max_x, &min_y, &max_y);
    // for index_x in min_x..max_x + 1 {
    //     if index_x == 0 {
    //         continue;
    //     }
    //     println!("{:?}", result_map);
    //     print_map(map, &result_map, &min_x, &max_x, &min_y, &max_y);
    //     println!("scanning x: {}", index_x);
    //     // discover_y_down(map, &mut result_map, &mut used_tile_indexes, &max_y, &index_x);
    //     discover_y_up(
    //         map,
    //         &mut result_map,
    //         &mut used_tile_indexes,
    //         &min_y,
    //         &index_x,
    //     );
    // }

    // println!("{:?}", result_map);
    // print_map(map, &result_map, &min_x, &max_x, &min_y, &max_y);
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

        let edge = map_tile.edge(search_specification.direction, map_tile.mirrored);
        let map_tile_next = map.get(index).unwrap();
        println!(
            "checking {} ({}) / {} against {}, edge: {}",
            index, map_tile_next.id, len, map_tile.id, edge
        );
        for (next_edge_index, edges_next) in map_tile_next.edges.iter().enumerate() {
            let matches_normal = edge == &edges_next.0;
            let matches_mirrored = edge == &edges_next.1;
            if matches_normal || matches_mirrored {
                println!(
                    "Matched {} with {}, edge index: {}, mirror: {}, edge: {}",
                    map_tile.id, map[index].id, next_edge_index, matches_mirrored, edge
                );
                let mirrored = map_tile.mirrored;
                let rotated = map_tile.rotated_times;
                map[index].set_match_way(
                    mirrored,
                    rotated,
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

fn discover_max_x(
    map: &mut Vec<MapTile>,
    result_map: &mut HashMap<i32, HashMap<i32, usize>>,
    used_tile_indexes: &mut Vec<usize>,
    max_x: &mut i32,
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

        // println!("checking [{}] / {}", index, len);
        let search_direction = Directions::Right;
        let edge = map_tile.edge(search_direction, map_tile.mirrored);
        let map_tile_next = map.get(index).unwrap();
        for (next_edge_index, edges_next) in map_tile_next.edges.iter().enumerate() {
            let matches_normal = edge == &edges_next.0;
            let matches_mirrored = edge == &edges_next.1;
            if matches_normal || matches_mirrored {
                println!(
                    "Matched {} with {}, edge index: {}, mirror: {}, edge: {}",
                    map_tile.id, map[index].id, next_edge_index, matches_mirrored, edge
                );
                let mirrored = map_tile.mirrored;
                let rotated = map_tile.rotated_times;
                map[index].set_match_way(
                    mirrored,
                    rotated,
                    next_edge_index,
                    matches_mirrored,
                    search_direction,
                );
                map_tile = &map[index];

                *max_x += 1;
                result_map.get_mut(&0).unwrap().insert(*max_x, index);
                used_tile_indexes.push(index);
                index = 0;
                break;
            }
        }

        index += 1;
    }
}

fn discover_min_x(
    map: &mut Vec<MapTile>,
    result_map: &mut HashMap<i32, HashMap<i32, usize>>,
    used_tile_indexes: &mut Vec<usize>,
    min_x: &mut i32,
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

        // println!("checking [{}] / {}", index, len);
        let map_tile_next = map.get(index).unwrap();
        let search_direction = Directions::Left;
        let edge = map_tile.edge(search_direction, map_tile.mirrored);
        for (next_edge_index, edges_next) in map_tile_next.edges.iter().enumerate() {
            let matches_normal = edge == &edges_next.0;
            let matches_mirrored = edge == &edges_next.1;
            if matches_normal || matches_mirrored {
                println!(
                    "Matched {} with {}, edge index: {}, mirror: {}, edge: {}",
                    map_tile.id, map[index].id, next_edge_index, matches_mirrored, edge
                );
                let mirrored = map_tile.mirrored;
                let rotated = map_tile.rotated_times;
                map[index].set_match_way(
                    mirrored,
                    rotated,
                    next_edge_index,
                    matches_mirrored,
                    search_direction,
                );
                map_tile = &map[index];

                *min_x -= 1;
                result_map.get_mut(&0).unwrap().insert(*min_x, index);
                used_tile_indexes.push(index);
                index = 0;
                break;
            }
        }

        index += 1;
    }
}

fn discover_max_y(
    map: &mut Vec<MapTile>,
    result_map: &mut HashMap<i32, HashMap<i32, usize>>,
    used_tile_indexes: &mut Vec<usize>,
    max_y: &mut i32,
) {
    let mut map_tile = &map[0];
    let len = map.len();
    let mut index: usize = 1;
    loop {
        while used_tile_indexes.contains(&index) {
            index += 1;
        }

        if index >= len {
            break;
        }

        // println!("checking [{}] / {}", index, len);
        let map_tile_next = map.get(index).unwrap();
        let search_direction = Directions::Down;
        let edge = map_tile.edge(search_direction, map_tile.mirrored);
        for (next_edge_index, edges_next) in map_tile_next.edges.iter().enumerate() {
            let matches_normal = edge == &edges_next.0;
            let matches_mirrored = edge == &edges_next.1;
            if matches_normal || matches_mirrored {
                println!(
                    "Matched {} with {}, edge index: {}, mirror: {}, edge: {}",
                    map_tile.id, map[index].id, next_edge_index, matches_mirrored, edge
                );
                let mirrored = map_tile.mirrored;
                let rotated = map_tile.rotated_times;
                map[index].set_match_way(
                    mirrored,
                    rotated,
                    next_edge_index,
                    matches_mirrored,
                    search_direction,
                );
                map_tile = &map[index];

                *max_y += 1;
                result_map.insert(*max_y, HashMap::new());
                result_map.get_mut(max_y).unwrap().insert(0, index);
                used_tile_indexes.push(index);
                index = 0;
                break;
            }
        }

        index += 1;
    }
}

fn discover_min_y(
    map: &mut Vec<MapTile>,
    result_map: &mut HashMap<i32, HashMap<i32, usize>>,
    used_tile_indexes: &mut Vec<usize>,
    min_y: &mut i32,
) {
    let mut map_tile = &map[0];
    let len = map.len();
    let mut index: usize = 1;
    loop {
        while used_tile_indexes.contains(&index) {
            index += 1;
        }

        if index >= len {
            break;
        }

        let map_tile_next = map.get(index).unwrap();
        let search_direction = Directions::Up;
        let edge = map_tile.edge(search_direction, map_tile.mirrored);
        println!(
            "checking {} ({}) / {} against {}, edge: {}",
            index, map_tile_next.id, len, map_tile.id, edge
        );
        for (next_edge_index, edges_next) in map_tile_next.edges.iter().enumerate() {
            let matches_normal = edge == &edges_next.0;
            let matches_mirrored = edge == &edges_next.1;
            if matches_normal || matches_mirrored {
                println!(
                    "Matched {} with {}, edge index: {}, mirror: {}, edge: {}",
                    map_tile.id, map[index].id, next_edge_index, matches_mirrored, edge
                );
                let mirrored = map_tile.mirrored;
                let rotated = map_tile.rotated_times;
                map[index].set_match_way(
                    mirrored,
                    rotated,
                    next_edge_index,
                    matches_mirrored,
                    search_direction,
                );
                map_tile = &map[index];

                *min_y -= 1;
                result_map.insert(*min_y, HashMap::new());
                result_map.get_mut(min_y).unwrap().insert(0, index);
                used_tile_indexes.push(index);
                index = 0;
                break;
            }
        }

        index += 1;
    }
}

fn discover_y_down(
    map: &Vec<MapTile>,
    result_map: &mut HashMap<i32, HashMap<i32, usize>>,
    used_tile_indexes: &mut Vec<usize>,
    max_y: &i32,
    index_x: &i32,
) {
    let map_index = result_map.get(&0).unwrap().get(index_x).unwrap();
    let mut map_tile = &map[*map_index];
    let len = map.len();
    let mut index: usize = 1;
    let mut index_y = 1;
    loop {
        println!("index {}, index_y {}, max_y {}", index, index_y, max_y);
        while used_tile_indexes.contains(&index) {
            index += 1;
        }

        if index_y > *max_y {
            break;
        }
        if index >= len {
            println!("{:?}", result_map);
            panic!();
        }

        println!("checking [{}] / {}", index, len);
        let map_tile_next = map.get(index).unwrap();
        let edge = &map_tile.edges[Directions::Down.index()].0;
        for edges in map_tile_next.edges.iter() {
            if edge == &edges.0 || edge == &edges.1 {
                result_map
                    .get_mut(&index_y)
                    .unwrap()
                    .insert(*index_x, index);
                index_y += 1;
                used_tile_indexes.push(index);
                map_tile = &map[index];
                index = 0;
                break;
            }
        }

        index += 1;
    }
}

fn discover_y_up(
    map: &Vec<MapTile>,
    result_map: &mut HashMap<i32, HashMap<i32, usize>>,
    used_tile_indexes: &mut Vec<usize>,
    min_y: &i32,
    index_x: &i32,
) {
    let map_index = result_map.get(&0).unwrap().get(index_x).unwrap();
    let mut map_tile = &map[*map_index];
    let len = map.len();
    let mut index: usize = 1;
    let mut index_y = -1;
    loop {
        println!(
            "index {}, index_x {}, index_y {}, min_y {}",
            index, index_x, index_y, min_y
        );
        while used_tile_indexes.contains(&index) {
            index += 1;
        }

        if index_y < *min_y {
            break;
        }
        if index >= len {
            println!("{:?}", result_map);
            println!("{:?}", used_tile_indexes);
            panic!();
        }

        // println!("checking [{}] / {}", index, len);
        let map_tile_next = map.get(index).unwrap();
        for edge in map_tile.edges.iter() {
            let edge = &edge.0;
            for edges_next in map_tile_next.edges.iter() {
                if edge == &edges_next.0 || edge == &edges_next.1 {
                    result_map
                        .get_mut(&index_y)
                        .unwrap()
                        .insert(*index_x, index);
                    index_y -= 1;
                    used_tile_indexes.push(index);
                    map_tile = &map[index];
                    index = 0;
                    break;
                }
            }
        }

        index += 1;
    }
}

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
