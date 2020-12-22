#[derive(Debug)]
pub struct Pattern {
    points: Vec<(usize, usize)>,
    max_x: usize,
    max_y: usize,
}

impl Pattern {
    #[allow(dead_code)]
    fn print(&self) {
        for index_y in 0..self.max_y + 1 {
            let mut line = String::with_capacity(self.max_x + 1);
            for index_x in 0..self.max_x + 1 {
                let matching = self.points.iter().filter(|point| point.0 == index_x && point.1 == index_y).count();
                if matching > 1 {
                    panic!();
                } else if matching == 1 {
                    line.push('#');
                } else {
                    line.push(' ');
                }
            }
            println!("{}", line);
        }
    }
}

pub fn get_monster_patterns() -> Vec<Pattern> {
    let monster_string = get_monster_string();
    let monster_char_pattern = monster_string
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let pattern_width = monster_char_pattern[0].len();
    let pattern_height = monster_char_pattern.len();
    println!("width: {}, height: {}", pattern_width, pattern_height);
    let mut monster_patterns: Vec<(usize, usize)> = Vec::new();
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for index_y in 0..pattern_height {
        for index_x in 0..pattern_width {
            let char = monster_char_pattern[index_y][index_x];
            if char != '#' {
                continue;
            }

            if max_x < index_x {
                max_x = index_x;
            }
            if max_y < index_y {
                max_y = index_y;
            }

            monster_patterns.push((index_x, index_y));
        }
    }

    let pattern_count: usize = 8;
    let mut patterns: Vec<Pattern> = Vec::with_capacity(pattern_count);
    patterns.push(Pattern {
        points: monster_patterns.clone(),
        max_x: max_x,
        max_y: max_y,
    });
    patterns.push(Pattern {
        points: monster_patterns.iter().map(|point| (max_x - point.0, point.1)).collect(),
        max_x: max_x,
        max_y: max_y,
    });
    patterns.push(Pattern {
        points: monster_patterns.iter().map(|point| (point.0, max_y - point.1)).collect(),
        max_x: max_x,
        max_y: max_y,
    });
    patterns.push(Pattern {
        points: monster_patterns.iter().map(|point| (max_x - point.0, max_y - point.1)).collect(),
        max_x: max_x,
        max_y: max_y,
    });

    patterns.push(Pattern {
        points: monster_patterns.iter().map(|point| (point.1, point.0)).collect(),
        max_x: max_y,
        max_y: max_x,
    });
    patterns.push(Pattern {
        points: monster_patterns.iter().map(|point| (max_y - point.1, point.0)).collect(),
        max_x: max_y,
        max_y: max_x,
    });
    patterns.push(Pattern {
        points: monster_patterns.iter().map(|point| (point.1, max_x - point.0)).collect(),
        max_x: max_y,
        max_y: max_x,
    });
    patterns.push(Pattern {
        points: monster_patterns.iter().map(|point| (max_y - point.1, max_x - point.0)).collect(),
        max_x: max_y,
        max_y: max_x,
    });

    // println!("{:?}", patterns);
    // for pattern_index in 0..patterns.len() {
    //     println!("{}", pattern_index);
    //     patterns[pattern_index].print();
    // }

    return patterns;
}

fn get_monster_string() -> &'static str {
"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   "
}
