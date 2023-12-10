use p10::*;
use std::collections::HashMap;

fn solve(input: &str) -> usize {
    let mut positions = HashMap::new();
    let mut animal_position = Position {
        from: None,
        row: 0,
        col: 0,
    };
    let nrows = input.lines().count();
    let ncols = input.lines().next().unwrap().len();
    let is_valid_position = |Position { row, col, from: _ }| row < nrows && col < ncols;
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let position = Position {
                from: None,
                row,
                col,
            };
            if ch == 'S' {
                animal_position = position;
            }
            positions.insert(position, Tile::from_char(ch));
        }
    }
    let directions = [
        Direction::North,
        Direction::East,
        Direction::West,
        Direction::South,
    ];
    let mut relevant_adj_nodes = Vec::new();
    for direction in directions {
        let position_adj_to_animal = match animal_position + direction {
            Some(pos) => pos,
            _ => continue,
        };
        if !is_valid_position(position_adj_to_animal) {
            continue;
        }
        let tile = positions.get(&position_adj_to_animal).unwrap();
        if let Tile::Connector(pipe) = tile {
            if pipe.get_directions().contains(&direction.get_opposite()) {
                relevant_adj_nodes.push(position_adj_to_animal);
            }
        }
    }
    assert_eq!(relevant_adj_nodes.len(), 2);
    let mut steps = 1;
    let mut nodes = relevant_adj_nodes;
    while nodes[0] != nodes[1] {
        let mut new_nodes = Vec::new();
        for node in nodes {
            let pipe = match positions.get(&node).unwrap() {
                Tile::Connector(p) => p,
                _ => panic!("a non-pipe made its way into the current nodes"),
            };
            for direction in pipe.get_directions() {
                if direction == node.from.unwrap() {
                    continue;
                }
                let adj = match node + direction {
                    Some(pos) => pos,
                    _ => continue,
                };
                // if !is_valid_position(adj) {
                //     continue;
                // }
                // let tile = positions.get(&adj).unwrap();
                // if let Tile::Connector(pipe) = tile {
                //     if pipe.get_directions().contains(&direction.get_opposite()) {
                //         new_nodes.push(adj);
                //     }
                // }
                new_nodes.push(adj);
            }
        }
        assert_eq!(new_nodes.len(), 2);
        nodes = new_nodes;
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example1() {
        let input = "\
.....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(solve(input), 4);
    }

    #[test]
    fn example2() {
        let input = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(solve(input), 8);
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Got the answer: {}", solve(input));
}
