use std::collections::{BTreeMap, HashMap, HashSet};

use std::fmt;
use std::mem;

use std::cmp;
use std::str::FromStr;

#[derive(Debug)]
enum SolutionError {
    ParseError(String),
}

fn part1(content: String) -> Result<(usize, usize), SolutionError> {
    let mut transport: Transport = content.parse().unwrap();
    if transport.carts.is_empty() {
        return Err(SolutionError::ParseError(
            "found no carts in input".to_owned(),
        ));
    }
    loop {
        let crashes = transport.step().unwrap();
        if !crashes.is_empty() {
            let c = crashes[0];
            println!("first crash at: {},{}", c.x, c.y);
            return Ok((c.x, c.y));
        }
    }
}

fn part2(content: String) -> Result<(), SolutionError> {
    let mut transport: Transport = content.parse().unwrap();
    if transport.carts.is_empty() {
        return Err(SolutionError::ParseError(
            "found no carts in input".to_owned(),
        ));
    }
    loop {
        let crashes = transport.step().unwrap();
        if !crashes.is_empty() {
            let _c = crashes[0];
            // writeln!(io::stdout(), "first crash at: {},{}", c.x, c.y)?;
            break;
        }
    }
    loop {
        transport.step().unwrap();
        let uncrashed = transport.uncrashed();
        if uncrashed.is_empty() {
            println!("mutually assured destruction");

            break;
        }
        if uncrashed.len() == 1 {
            let c = uncrashed[0];
            println!("last cart standing at: {},{}", c.x, c.y);
            break;
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum Track {
    Empty,
    Vertical,
    Horizontal,
    Intersection,
    CurveForward,
    CurveBackward,
}

impl FromStr for Track {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes().get(0) {
            None => Err("no track available in empty string".to_owned()),
            Some(&b' ') => Ok(Track::Empty),
            Some(&b'|') => Ok(Track::Vertical),
            Some(&b'-') => Ok(Track::Horizontal),
            Some(&b'+') => Ok(Track::Intersection),
            Some(&b'/') => Ok(Track::CurveForward),
            Some(&b'\\') => Ok(Track::CurveBackward),
            Some(&b) => Err(format!("unrecognized track: 0x{:X}", b)),
        }
    }
}

impl TryFrom<&str> for Track {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.as_bytes().get(0) {
            None => Err("no track available in empty string".to_owned()),
            Some(&b' ') => Ok(Track::Empty),
            Some(&b'|') => Ok(Track::Vertical),
            Some(&b'-') => Ok(Track::Horizontal),
            Some(&b'+') => Ok(Track::Intersection),
            Some(&b'/') => Ok(Track::CurveForward),
            Some(&b'\\') => Ok(Track::CurveBackward),
            Some(&b) => Err(format!("unrecognized track: 0x{:X}", b)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl TryFrom<&str> for Coordinate {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        Ok(Coordinate { x, y })
    }
}

#[derive(Debug)]
enum CoordinateError {
    CannotMove(String),
}

impl Coordinate {
    fn up(self) -> Result<Coordinate, CoordinateError> {
        if self.y == 0 {
            Err(CoordinateError::CannotMove("cannot move up".to_owned()))
        } else {
            Ok(Coordinate {
                y: self.y - 1,
                ..self
            })
        }
    }

    fn down(self) -> Result<Coordinate, CoordinateError> {
        Ok(Coordinate {
            y: self.y + 1,
            ..self
        })
    }

    fn left(self) -> Result<Coordinate, CoordinateError> {
        if self.x == 0 {
            Err(CoordinateError::CannotMove("cannot move left".to_owned()))
        } else {
            Ok(Coordinate {
                x: self.x - 1,
                ..self
            })
        }
    }

    fn right(self) -> Result<Coordinate, CoordinateError> {
        Ok(Coordinate {
            x: self.x + 1,
            ..self
        })
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Coordinate) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Coordinate) -> Option<cmp::Ordering> {
        Some((self.y, self.x).cmp(&(other.y, other.x)))
    }
}

#[derive(Debug)]

enum TransportError {}

#[derive(Clone)]
struct Transport {
    carts: BTreeMap<Coordinate, Cart>,
    grid: Grid,
}

impl Transport {
    fn new() -> Transport {
        Transport {
            carts: BTreeMap::new(),
            grid: Grid::new(),
        }
    }

    fn step(&mut self) -> Result<Vec<Coordinate>, TransportError> {
        let mut crashes = HashSet::new();
        let mut previous_carts = mem::replace(&mut self.carts, BTreeMap::new());
        for (c, cart) in previous_carts.clone() {
            if crashes.contains(&c) {
                continue;
            }

            let (next_cart, next_c) = self.grid.step(cart, c).unwrap();
            assert!(!cart.is_crashed());
            assert!(!next_cart.is_crashed());

            if previous_carts.contains_key(&next_c) || self.carts.contains_key(&next_c) {
                self.carts.remove(&next_c);
                crashes.insert(next_c);
            } else {
                assert!(!self.carts.contains_key(&next_c));
                self.carts.insert(next_c, next_cart);
            }
            previous_carts.remove(&c);
        }
        Ok(crashes.into_iter().collect())
    }

    fn uncrashed(&self) -> Vec<Coordinate> {
        self.carts
            .iter()
            .filter(|&(_, cart)| !cart.is_crashed())
            .map(|(&c, _)| c)
            .collect()
    }
}

impl FromStr for Transport {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err("expected initial transport grid to be ASCII");
        }

        let mut trans = Transport::new();
        for (y, line) in s.lines().enumerate() {
            for x in line.char_indices().map(|(i, _)| i) {
                let c = Coordinate { x, y };
                let cell = &line[x..x + 1];
                if !"<>^v".contains(cell) {
                    trans.grid.set(c, cell.parse().unwrap());
                    continue;
                }
                let cart = cell.parse().unwrap();
                trans.carts.insert(c, cart);
                trans.grid.set(c, cart.initial_track().unwrap());
            }
        }
        Ok(trans)
    }
}

impl TryFrom<&str> for Transport {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.is_ascii() {
            return Err("expected initial transport grid to be ASCII");
        }

        let mut trans = Transport::new();
        for (y, line) in value.lines().enumerate() {
            for x in line.char_indices().map(|(i, _)| i) {
                let c = Coordinate { x, y };
                let cell = &line[x..x + 1];
                if !"<>^v".contains(cell) {
                    trans.grid.set(c, cell.parse().unwrap());
                    continue;
                }
                let cart = cell.parse().unwrap();
                trans.carts.insert(c, cart);
                trans.grid.set(c, cart.initial_track().unwrap());
            }
        }
        Ok(trans)
    }
}

impl fmt::Debug for Transport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..=self.grid.max_y {
            for x in 0..=self.grid.max_x {
                let c = Coordinate { x, y };
                if let Some(&cart) = self.carts.get(&c) {
                    write!(f, "{:?}", cart)?;
                } else {
                    write!(f, "{:?}", self.grid.get(c))?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct Cart {
    intersections: usize,
    kind: CartKind,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum CartKind {
    Up,
    Down,
    Left,
    Right,
    Crashed,
}

#[derive(Debug)]

enum CartError {
    UnknownTrack(String),
}

impl Cart {
    fn initial_track(&self) -> Result<Track, CartError> {
        match self.kind {
            CartKind::Up | CartKind::Down => Ok(Track::Vertical),
            CartKind::Left | CartKind::Right => Ok(Track::Horizontal),
            CartKind::Crashed => Err(CartError::UnknownTrack(
                "unknown track for crashed cart".to_owned(),
            )),
        }
    }

    fn is_crashed(&self) -> bool {
        self.kind == CartKind::Crashed
    }

    fn direction(self, kind: CartKind) -> Cart {
        Cart { kind, ..self }
    }

    fn intersection(mut self) -> Cart {
        let which = self.intersections % 3;
        self.intersections += 1;
        match which {
            0 => self.turn_left(),
            1 => self,
            2 => self.turn_right(),
            _ => unreachable!(),
        }
    }

    fn turn_left(self) -> Cart {
        use self::CartKind::*;

        let kind = match self.kind {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
            Crashed => Crashed,
        };
        Cart { kind, ..self }
    }

    fn turn_right(self) -> Cart {
        use self::CartKind::*;

        let kind = match self.kind {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
            Crashed => Crashed,
        };
        Cart { kind, ..self }
    }
}

impl FromStr for Cart {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let kind = match value.as_bytes().get(0) {
            None => return Err("no cart available in empty string".to_owned()),
            Some(&b'^') => CartKind::Up,
            Some(&b'v') => CartKind::Down,
            Some(&b'<') => CartKind::Left,
            Some(&b'>') => CartKind::Right,
            Some(&b'X') => CartKind::Crashed,
            Some(&b) => return Err(format!("unrecognized cart: 0x{:X}", b)),
        };
        Ok(Cart {
            intersections: 0,
            kind,
        })
    }
}

impl TryFrom<&str> for Cart {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let kind = match value.as_bytes().get(0) {
            None => return Err("no cart available in empty string".to_owned()),
            Some(&b'^') => CartKind::Up,
            Some(&b'v') => CartKind::Down,
            Some(&b'<') => CartKind::Left,
            Some(&b'>') => CartKind::Right,
            Some(&b'X') => CartKind::Crashed,
            Some(&b) => return Err(format!("unrecognized cart: 0x{:X}", b).to_owned()),
        };
        Ok(Cart {
            intersections: 0,
            kind,
        })
    }
}

impl fmt::Debug for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            CartKind::Up => write!(f, "^"),
            CartKind::Down => write!(f, "v"),
            CartKind::Left => write!(f, "<"),
            CartKind::Right => write!(f, ">"),
            CartKind::Crashed => write!(f, "X"),
        }
    }
}

#[derive(Clone)]
struct Grid {
    tracks: HashMap<Coordinate, Track>,
    max_x: usize,
    max_y: usize,
}

#[derive(Debug)]
enum GridError {
    CannotStep(String),
}

impl Grid {
    fn new() -> Grid {
        Grid {
            tracks: HashMap::new(),
            max_x: 0,
            max_y: 0,
        }
    }

    fn get(&self, c: Coordinate) -> Track {
        self.tracks.get(&c).map(|&c| c).unwrap_or(Track::Empty)
    }

    fn set(&mut self, c: Coordinate, track: Track) {
        self.tracks.insert(c, track);
        self.max_x = cmp::max(self.max_x, c.x);
        self.max_y = cmp::max(self.max_y, c.y);
    }

    // /// Given a cart and its position in the grid, return the next position
    // /// for the cart.
    fn step(&self, mut cart: Cart, c: Coordinate) -> Result<(Cart, Coordinate), GridError> {
        use self::CartKind::*;
        use self::Track::*;

        let next_coord = match (cart.kind, self.get(c)) {
            (_, Empty) => {
                return Err(GridError::CannotStep(
                    "invalid transition on empty".to_owned(),
                ))
            }
            (Crashed, _) => c,
            (Up, Horizontal) => {
                return Err(GridError::CannotStep(
                    "cannot go up on horizontal".to_owned(),
                ))
            }
            (Up, _) => c.up().unwrap(),
            (Down, Horizontal) => {
                return Err(GridError::CannotStep(
                    "cannot go down on horizontal".to_owned(),
                ))
            }
            (Down, _) => c.down().unwrap(),
            (Left, Vertical) => {
                return Err(GridError::CannotStep(
                    "cannot go left on vertical".to_owned(),
                ))
            }
            (Left, _) => c.left().unwrap(),
            (Right, Vertical) => {
                return Err(GridError::CannotStep(
                    "cannot go right on vertical".to_owned(),
                ))
            }
            (Right, _) => c.right().unwrap(),
        };
        cart = match (cart.kind, self.get(next_coord)) {
            (_, Empty) => {
                return Err(GridError::CannotStep(
                    "cannot move to empty coordinate".to_owned(),
                ))
            }
            (Crashed, _) => cart,
            (Up, Vertical) => cart.direction(Up),
            (Up, Horizontal) => cart.direction(Up),
            (Up, Intersection) => cart.intersection(),
            (Up, CurveForward) => cart.direction(Right),
            (Up, CurveBackward) => cart.direction(Left),
            (Down, Vertical) => cart.direction(Down),
            (Down, Horizontal) => cart.direction(Down),
            (Down, Intersection) => cart.intersection(),
            (Down, CurveForward) => cart.direction(Left),
            (Down, CurveBackward) => cart.direction(Right),
            (Left, Vertical) => cart.direction(Left),
            (Left, Horizontal) => cart.direction(Left),
            (Left, Intersection) => cart.intersection(),
            (Left, CurveForward) => cart.direction(Down),
            (Left, CurveBackward) => cart.direction(Up),
            (Right, Vertical) => cart.direction(Right),
            (Right, Horizontal) => cart.direction(Right),
            (Right, Intersection) => cart.intersection(),
            (Right, CurveForward) => cart.direction(Up),
            (Right, CurveBackward) => cart.direction(Down),
        };
        Ok((cart, next_coord))
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                write!(f, "{:?}", self.get(Coordinate { x, y }))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Track::Empty => write!(f, " "),
            Track::Vertical => write!(f, "|"),
            Track::Horizontal => write!(f, "-"),
            Track::Intersection => write!(f, "+"),
            Track::CurveForward => write!(f, "/"),
            Track::CurveBackward => write!(f, "\\"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_first() {
        let example = include_str!("./example.data");
        let (x, y) = part1(example.to_string()).unwrap();
        assert_eq!(x, 7);
        assert_eq!(y, 3);

        let prod = include_str!("./prod.data");
        let (x, y) = part1(prod.to_string()).unwrap();
        assert_eq!(x, 50);
        assert_eq!(y, 54);
    }

    #[test]
    #[ignore]
    fn test_second() {
        let example = include_str!("./example.data");
        part2(example.to_string()).unwrap();

        let prod = include_str!("./prod.data");
        let _ = part2(prod.to_string());
    }
}
