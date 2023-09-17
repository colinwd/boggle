use std::{fmt::Display, fmt::Debug};

use rand::seq::SliceRandom;

type Die = [char; 6];

const DICE: [Die; 16] = [
    ['A', 'A', 'E', 'E', 'G', 'N'],
    ['A', 'B', 'B', 'J', 'O', 'O'],
    ['A', 'C', 'H', 'O', 'P', 'S'],
    ['A', 'F', 'F', 'K', 'P', 'S'],
    ['A', 'O', 'O', 'T', 'T', 'W'],
    ['C', 'I', 'M', 'O', 'T', 'U'],
    ['D', 'E', 'I', 'L', 'R', 'X'],
    ['D', 'E', 'L', 'V', 'R', 'Y'],
    ['D', 'I', 'S', 'T', 'T', 'Y'],
    ['E', 'E', 'G', 'H', 'N', 'W'],
    ['E', 'E', 'I', 'N', 'S', 'U'],
    ['E', 'H', 'R', 'T', 'V', 'W'],
    ['E', 'I', 'O', 'S', 'S', 'T'],
    ['E', 'L', 'R', 'T', 'T', 'Y'],
    ['H', 'I', 'M', 'N', 'U', 'Q'], //TODO: Q -> Qu
    ['H', 'L', 'N', 'N', 'R', 'Z'],
];

#[derive(Debug)]
pub struct Board {
    pub slots: [[Cell; 4]; 4],
}

impl Board {
    pub fn new() -> Board {
        let rolled = DICE.map(|die| {
            die.roll().clone()
        });

        let slots = [
            [Cell::new(0, 0, rolled[0]), Cell::new(1, 0, rolled[1]), Cell::new(2, 0, rolled[2]), Cell::new(3, 0, rolled[3])],
            [Cell::new(0, 1, rolled[4]), Cell::new(1, 1, rolled[5]), Cell::new(2, 1, rolled[6]), Cell::new(3, 1, rolled[7])],
            [Cell::new(0, 2, rolled[8]), Cell::new(1, 2, rolled[9]), Cell::new(2, 2, rolled[10]), Cell::new(3, 2, rolled[11])],
            [Cell::new(0, 3, rolled[12]), Cell::new(1, 3, rolled[13]), Cell::new(2, 3, rolled[14]), Cell::new(3, 3, rolled[15])],
        ];

        Board { slots }
    }

    pub fn neighbors(&self, c: &Cell, previous: &Vec<&Cell>) -> Vec<&Cell> {
        let neighbors = vec![
            self.east(c),
            self.southeast(c),
            self.south(c),
            self.southwest(c),
            self.west(c),
            self.northwest(c),
            self.north(c),
            self.northeast(c)
        ];

        neighbors
            .iter()
            .filter_map(|opt| *opt)
            .filter(|cell| !previous.contains(cell))
            .collect()
    }

    fn north(&self, c: &Cell) -> Option<&Cell> {
        if c.y == 0 {
            None
        } else {
            let (x, y) = (c.x, c.y - 1);
            Some(&self.slots[y][x])
        }
    }

    fn northeast(&self, c: &Cell) -> Option<&Cell> {
        if c.y == 0 || c.x == 3 {
            None
        } else {
            let (x, y) = (c.x + 1, c.y - 1);
            Some(&self.slots[y][x])
        }
    }

    fn east(&self, c: &Cell) -> Option<&Cell> {
        if c.x == 3 {
            None
        } else {
            let (x, y) = (c.x + 1, c.y);
            Some(&self.slots[y][x])
        }
    }

    fn southeast(&self, c: &Cell) -> Option<&Cell> {
        if c.x == 3 || c.y == 3 {
            None
        } else {
            let (x, y) = (c.x + 1, c.y + 1);
            Some(&self.slots[y][x])
        }
    }

    fn south(&self, c: &Cell) -> Option<&Cell> {
        if c.y == 3 {
            None
        } else {
            let (x, y) = (c.x, c.y + 1);
            Some(&self.slots[y][x])
        }
    }

    fn southwest(&self, c: &Cell) -> Option<&Cell> {
        if c.x == 0 || c.y == 3 {
            None
        } else {
            let (x, y) = (c.x - 1, c.y + 1);
            Some(&self.slots[y][x])
        }
    }

    fn west(&self, c: &Cell) -> Option<&Cell> {
        if c.x == 0 {
            None
        } else {
            let (x, y) = (c.x - 1, c.y);
            Some(&self.slots[y][x])
        }
    }

    fn northwest(&self, c: &Cell) -> Option<&Cell> {
        if c.x == 0 || c.y == 0 {
            None
        } else {
            let (x, y) = (c.x - 1, c.y - 1);
            Some(&self.slots[y][x])
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.slots {
            f.write_str(format!("{} {} {} {}\n", row[0].contents, row[1].contents, row[2].contents, row[3].contents).as_ref())?;
        }

        Ok(())
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub contents: char,
}

impl Cell {
    pub fn new(x: usize, y: usize, c: char) -> Cell {
        Cell { x, y, contents: c }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}] {}", self.x, self.y, self.contents)
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}] {}", self.x, self.y, self.contents)
    }
}

trait Rollable<T> {
    fn roll(&self) -> &T;
}

impl Rollable<char> for Die {
    fn roll(&self) -> &char {
        self.choose(&mut rand::thread_rng()).unwrap()
    }
}