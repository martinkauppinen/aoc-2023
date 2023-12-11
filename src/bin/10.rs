use std::ops::{Index, IndexMut};

use itertools::Itertools;
advent_of_code::solution!(10);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn go(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                self.row -= 1;
            }
            Direction::South => {
                self.row += 1;
            }
            Direction::East => {
                self.col += 1;
            }
            Direction::West => {
                self.col -= 1;
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn invert(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Pipe {
    from: Direction,
    to: Direction,
    part_of_loop: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PipeType {
    Bend,
    Horizontal,
    Vertical,
}

impl Pipe {
    fn connects(&self, from: Direction) -> bool {
        self.from == from || self.to == from
    }

    fn outgoing(&self, from: Direction) -> Direction {
        if self.from == from {
            self.to
        } else if self.to == from {
            self.from
        } else {
            panic!("bad direction")
        }
    }

    fn swap(&mut self) {
        std::mem::swap(&mut self.from, &mut self.to);
    }

    fn is_bend(&self) -> bool {
        !self.is_horizontal() && !self.is_vertical()
    }

    fn is_horizontal(&self) -> bool {
        self.connects(Direction::West) && self.connects(Direction::East)
    }

    fn is_vertical(&self) -> bool {
        self.connects(Direction::North) && self.connects(Direction::South)
    }

    fn get_type(&self) -> PipeType {
        if self.is_bend() {
            PipeType::Bend
        } else if self.is_horizontal() {
            PipeType::Horizontal
        } else {
            PipeType::Vertical
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Pipe(Pipe),
    Ground,
    Start,
}

impl Tile {
    fn connects(&self, from: Direction) -> bool {
        match self {
            Tile::Pipe(pipe) => pipe.connects(from),
            _ => false,
        }
    }

    fn pipe(&self) -> Pipe {
        match self {
            Tile::Pipe(pipe) => *pipe,
            _ => panic!("not a pipe"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    start: Position,
    current_position: Position,
}

impl Grid {
    fn new(mut tiles: Vec<Vec<Tile>>) -> Self {
        let (row, start_row) = tiles
            .iter()
            .find_position(|row| row.contains(&Tile::Start))
            .unwrap();
        let (col, _) = start_row
            .iter()
            .find_position(|tile| matches!(tile, Tile::Start))
            .unwrap();

        let start = Position { row, col };
        let current_position = Position { row, col };

        let connects_north = tiles[row - 1][col].connects(Direction::South);
        let connects_south = tiles[row + 1][col].connects(Direction::North);
        let connects_east = tiles[row][col + 1].connects(Direction::West);
        let connects_west = tiles[row][col - 1].connects(Direction::East);

        let start_pipe = match (connects_north, connects_east, connects_south, connects_west) {
            (true, false, true, false) => Tile::Pipe(Pipe {
                from: Direction::North,
                to: Direction::South,
                part_of_loop: true,
            }),
            (false, true, false, true) => Tile::Pipe(Pipe {
                from: Direction::East,
                to: Direction::West,
                part_of_loop: true,
            }),
            (true, true, false, false) => Tile::Pipe(Pipe {
                from: Direction::North,
                to: Direction::East,
                part_of_loop: true,
            }),
            (true, false, false, true) => Tile::Pipe(Pipe {
                from: Direction::North,
                to: Direction::West,
                part_of_loop: true,
            }),
            (false, false, true, true) => Tile::Pipe(Pipe {
                from: Direction::South,
                to: Direction::West,
                part_of_loop: true,
            }),
            (false, true, true, false) => Tile::Pipe(Pipe {
                from: Direction::South,
                to: Direction::East,
                part_of_loop: true,
            }),
            _ => unreachable!(),
        };

        tiles[row][col] = start_pipe;

        Self {
            tiles,
            start,
            current_position,
        }
    }

    fn find_loop_len(&mut self) -> usize {
        let mut len = 1;
        let mut current_position = self.start;
        let mut current_tile = self.tiles[self.start];
        let mut go_direction = current_tile.pipe().from;
        current_position.go(current_tile.pipe().outgoing(go_direction));
        go_direction = current_tile.pipe().outgoing(go_direction);
        current_tile = self.tiles[current_position];

        while current_position != self.start {
            go_direction = go_direction.invert();

            let mut current_pipe = self.tiles[current_position].pipe();
            // fix orientation
            if current_pipe.from != go_direction {
                current_pipe.swap();
            }
            current_pipe.part_of_loop = true;
            self.tiles[current_position] = Tile::Pipe(current_pipe);

            len += 1;
            current_position.go(current_tile.pipe().outgoing(go_direction));
            go_direction = current_tile.pipe().outgoing(go_direction);
            current_tile = self.tiles[current_position];
        }
        len
    }
}

impl Index<Position> for Vec<Vec<Tile>> {
    type Output = Tile;

    fn index(&self, index: Position) -> &Self::Output {
        &self[index.row][index.col]
    }
}

impl IndexMut<Position> for Vec<Vec<Tile>> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self[index.row][index.col]
    }
}

fn parse_line(line: &str) -> Vec<Tile> {
    let mut tiles = vec![Tile::Ground; line.len()];
    line.chars().enumerate().for_each(|(i, c)| match c {
        'S' => tiles[i] = Tile::Start,
        '|' => {
            tiles[i] = Tile::Pipe(Pipe {
                from: Direction::North,
                to: Direction::South,
                part_of_loop: false,
            })
        }
        '-' => {
            tiles[i] = Tile::Pipe(Pipe {
                from: Direction::East,
                to: Direction::West,
                part_of_loop: false,
            })
        }
        'L' => {
            tiles[i] = Tile::Pipe(Pipe {
                from: Direction::North,
                to: Direction::East,
                part_of_loop: false,
            })
        }
        'J' => {
            tiles[i] = Tile::Pipe(Pipe {
                from: Direction::North,
                to: Direction::West,
                part_of_loop: false,
            })
        }
        '7' => {
            tiles[i] = Tile::Pipe(Pipe {
                from: Direction::South,
                to: Direction::West,
                part_of_loop: false,
            })
        }
        'F' => {
            tiles[i] = Tile::Pipe(Pipe {
                from: Direction::South,
                to: Direction::East,
                part_of_loop: false,
            })
        }
        _ => {}
    });
    tiles
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::new(input.lines().map(parse_line).collect_vec());
    Some((grid.find_loop_len() / 2) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = Grid::new(input.lines().map(parse_line).collect_vec());
    grid.find_loop_len();
    let mut nest = 0;
    for row in grid.tiles {
        let mut winding_number = 0;
        let mut bend_stack: Vec<Pipe> = Vec::new();
        for tile in row {
            match tile {
                #[allow(clippy::collapsible_if, clippy::if_same_then_else)]
                Tile::Pipe(pipe) if pipe.part_of_loop => {
                    if pipe.is_horizontal() {
                        continue;
                    }
                    if let Some(last_bend) = bend_stack.pop() {
                        match (last_bend.get_type(), pipe.get_type()) {
                            (PipeType::Bend, PipeType::Bend) => {
                                if last_bend.from == Direction::North && pipe.to == Direction::South
                                {
                                    winding_number -= 1;
                                } else if last_bend.from == Direction::South
                                    && pipe.to == Direction::North
                                {
                                    winding_number += 1;
                                } else if last_bend.to == Direction::North
                                    && pipe.from == Direction::South
                                {
                                    winding_number += 1;
                                } else if last_bend.to == Direction::South
                                    && pipe.from == Direction::North
                                {
                                    winding_number -= 1;
                                }
                            }
                            (PipeType::Bend, PipeType::Vertical) => {
                                if pipe.to == Direction::North {
                                    winding_number += 1;
                                } else {
                                    winding_number -= 1;
                                }
                                bend_stack.push(last_bend);
                            }
                            _ => {}
                        }
                    } else if pipe.is_vertical() {
                        if pipe.to == Direction::North {
                            winding_number += 1;
                        } else {
                            winding_number -= 1;
                        }
                    } else {
                        bend_stack.push(pipe);
                    }
                }
                _ => {
                    if winding_number != 0 {
                        nest += 1;
                    }
                }
            }
        }
    }
    Some(nest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_example("examples", DAY, 1));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_example("examples", DAY, 2));
        assert_eq!(result, Some(8));
    }
}
