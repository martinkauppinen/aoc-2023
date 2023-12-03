advent_of_code::solution!(3);

#[derive(Debug, Default, Clone, Copy)]
struct Position {
    row: u32,
    col: u32,
}

#[derive(Debug, Default, Clone, Copy)]
struct Rectangle {
    top_left: Position,
    bottom_right: Position,
}

impl Rectangle {
    fn contains(&self, position: Position) -> bool {
        self.top_left.row <= position.row
            && self.bottom_right.row >= position.row
            && self.top_left.col <= position.col
            && self.bottom_right.col >= position.col
    }
}

#[derive(Debug, Default)]
struct Symbol {
    symbol: char,
    position: Position,
}

#[derive(Debug, Default)]
struct Number {
    number: u32,
    position: Position,
    length: u32,
}

impl Number {
    fn rect(&self) -> Rectangle {
        let top_left = Position {
            row: self.position.row.saturating_sub(1),
            col: self.position.col.saturating_sub(1),
        };
        let bottom_right = Position {
            row: self.position.row + 1,
            col: self.position.col + self.length,
        };
        Rectangle {
            top_left,
            bottom_right,
        }
    }
}

#[derive(Debug, Default)]
struct Grid {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

fn parse_line(line: &str, row_num: u32) -> (Vec<Symbol>, Vec<Number>) {
    let mut symbols = vec![];
    let mut numbers = vec![];
    let mut col_num = 0;

    while let Some(c) = line[col_num..].find(|c| c != '.') {
        let line_slice = &line[col_num..];
        let position = Position {
            row: row_num,
            col: (c + col_num) as u32,
        };

        if !line_slice.as_bytes()[c].is_ascii_digit() {
            symbols.push(Symbol {
                symbol: line_slice.as_bytes()[c] as char,
                position,
            });
            col_num += c + 1;
            continue;
        }

        let line_slice = &line_slice[c..];
        let next_non_digit = line_slice
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(line_slice.len());
        let number: u32 = line_slice[..next_non_digit].parse().unwrap();
        let number_length = number.ilog10() + 1;
        numbers.push(Number {
            number,
            position,
            length: number_length,
        });

        col_num += c + number_length as usize;
    }
    (symbols, numbers)
}

fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .fold(Grid::default(), |mut grid, (row_num, line)| {
            let (symbols, numbers) = parse_line(line, row_num as u32);
            grid.symbols.extend(symbols);
            grid.numbers.extend(numbers);
            grid
        })
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);

    Some(
        grid.numbers
            .into_iter()
            .filter(|number| {
                let rect = number.rect();
                grid.symbols
                    .iter()
                    .any(|symbol| rect.contains(symbol.position))
            })
            .map(|number| number.number)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);

    Some(
        grid.symbols
            .into_iter()
            .filter(|symbol| symbol.symbol == '*')
            .map(|gear| {
                let mut numbers = grid
                    .numbers
                    .iter()
                    .filter(|number| number.rect().contains(gear.position));

                let a = numbers.next();
                let b = numbers.next();
                let c = numbers.next();

                if let (Some(a), Some(b), None) = (a, b, c) {
                    a.number * b.number
                } else {
                    0
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
