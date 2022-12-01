use super::*;

#[test]
fn determine_map_change_search_up_down_matching() {
    let result = MapTile::determine_map_change(Directions::Down.index(), false, Directions::Up);
    assert_eq!((0, true, false), result);
}

#[test]
fn determine_map_change_search_up_down_mirrored() {
    let result = MapTile::determine_map_change(Directions::Down.index(), true, Directions::Up);
    assert_eq!((0, false, false), result);
}

#[test]
fn determine_map_change_search_up_up_matching() {
    let result = MapTile::determine_map_change(Directions::Up.index(), false, Directions::Up);
    assert_eq!((2, true, false), result);
}

#[test]
fn determine_map_change_search_up_up_mirrored() {
    let result = MapTile::determine_map_change(Directions::Up.index(), true, Directions::Up);
    assert_eq!((2, false, false), result);
}

#[test]
fn determine_map_change_search_up_right_matching() {
    let result = MapTile::determine_map_change(Directions::Right.index(), false, Directions::Up);
    assert_eq!((1, true, false), result);
}

#[test]
fn determine_map_change_search_up_right_mirrored() {
    let result = MapTile::determine_map_change(Directions::Right.index(), true, Directions::Up);
    assert_eq!((1, false, false), result);
}

#[test]
fn determine_map_change_search_up_left_matching() {
    let result = MapTile::determine_map_change(Directions::Left.index(), false, Directions::Up);
    assert_eq!((3, true, false), result);
}

#[test]
fn determine_map_change_search_up_left_mirrored() {
    let result = MapTile::determine_map_change(Directions::Left.index(), true, Directions::Up);
    assert_eq!((3, false, false), result);
}

#[test]
fn determine_map_change_search_down_down_matching() {
    let result = MapTile::determine_map_change(Directions::Down.index(), false, Directions::Down);
    assert_eq!((2, true, false), result);
}

#[test]
fn determine_map_change_search_down_down_mirrored() {
    let result = MapTile::determine_map_change(Directions::Down.index(), true, Directions::Down);
    assert_eq!((2, false, false), result);
}

#[test]
fn determine_map_change_search_down_up_matching() {
    let result = MapTile::determine_map_change(Directions::Up.index(), false, Directions::Down);
    assert_eq!((0, true, false), result);
}

#[test]
fn determine_map_change_search_down_up_mirrored() {
    let result = MapTile::determine_map_change(Directions::Up.index(), true, Directions::Down);
    assert_eq!((0, false, false), result);
}

#[test]
fn determine_map_change_search_down_right_matching() {
    let result = MapTile::determine_map_change(Directions::Right.index(), false, Directions::Down);
    assert_eq!((3, true, false), result);
}

#[test]
fn determine_map_change_search_down_right_mirrored() {
    let result = MapTile::determine_map_change(Directions::Right.index(), true, Directions::Down);
    assert_eq!((3, false, false), result);
}

#[test]
fn determine_map_change_search_down_left_matching() {
    let result = MapTile::determine_map_change(Directions::Left.index(), false, Directions::Down);
    assert_eq!((1, true, false), result);
}

#[test]
fn determine_map_change_search_down_left_mirrored() {
    let result = MapTile::determine_map_change(Directions::Left.index(), true, Directions::Down);
    assert_eq!((1, false, false), result);
}

#[test]
fn determine_map_change_search_right_down_matching() {
    let result = MapTile::determine_map_change(Directions::Down.index(), false, Directions::Right);
    assert_eq!((1, false, true), result);
}

#[test]
fn determine_map_change_search_right_down_mirrored() {
    let result = MapTile::determine_map_change(Directions::Down.index(), true, Directions::Right);
    assert_eq!((1, false, false), result);
}

#[test]
fn determine_map_change_search_right_up_matching() {
    let result = MapTile::determine_map_change(Directions::Up.index(), false, Directions::Right);
    assert_eq!((3, false, true), result);
}

#[test]
fn determine_map_change_search_right_up_mirrored() {
    let result = MapTile::determine_map_change(Directions::Up.index(), true, Directions::Right);
    assert_eq!((3, false, false), result);
}

#[test]
fn determine_map_change_search_right_right_matching() {
    let result = MapTile::determine_map_change(Directions::Right.index(), false, Directions::Right);
    assert_eq!((2, false, true), result);
}

#[test]
fn determine_map_change_search_right_right_mirrored() {
    let result = MapTile::determine_map_change(Directions::Right.index(), true, Directions::Right);
    assert_eq!((2, false, false), result);
}

#[test]
fn determine_map_change_search_right_left_matching() {
    let result = MapTile::determine_map_change(Directions::Left.index(), false, Directions::Right);
    assert_eq!((0, false, true), result);
}

#[test]
fn determine_map_change_search_right_left_mirrored() {
    let result = MapTile::determine_map_change(Directions::Left.index(), true, Directions::Right);
    assert_eq!((0, false, false), result);
}

#[test]
fn determine_map_change_search_left_down_matching() {
    let result = MapTile::determine_map_change(Directions::Down.index(), false, Directions::Left);
    assert_eq!((3, false, true), result);
}

#[test]
fn determine_map_change_search_left_down_mirrored() {
    let result = MapTile::determine_map_change(Directions::Down.index(), true, Directions::Left);
    assert_eq!((3, false, false), result);
}

#[test]
fn determine_map_change_search_left_up_matching() {
    let result = MapTile::determine_map_change(Directions::Up.index(), false, Directions::Left);
    assert_eq!((1, false, true), result);
}

#[test]
fn determine_map_change_search_left_up_mirrored() {
    let result = MapTile::determine_map_change(Directions::Up.index(), true, Directions::Left);
    assert_eq!((1, false, false), result);
}

#[test]
fn determine_map_change_search_left_right_matching() {
    let result = MapTile::determine_map_change(Directions::Right.index(), false, Directions::Left);
    assert_eq!((0, false, true), result);
}

#[test]
fn determine_map_change_search_left_right_mirrored() {
    let result = MapTile::determine_map_change(Directions::Right.index(), true, Directions::Left);
    assert_eq!((0, false, false), result);
}

#[test]
fn determine_map_change_search_left_left_matching() {
    let result = MapTile::determine_map_change(Directions::Left.index(), false, Directions::Left);
    assert_eq!((2, false, true), result);
}

#[test]
fn determine_map_change_search_left_left_mirrored() {
    let result = MapTile::determine_map_change(Directions::Left.index(), true, Directions::Left);
    assert_eq!((2, false, false), result);
}
