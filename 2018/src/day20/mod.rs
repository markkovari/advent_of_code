// https://adventofcode.com/2018/day/20

// imports

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum Tokens {
    Start,
    End,
    OpenDirection(OpenDirections),
    ParenOpen,
    ParenClose,
    BranchOr,
}

fn tokenize(input_string: &str) -> Vec<Tokens> {
    return input_string
        .trim()
        .chars()
        .map(|c| match c {
            '^' => Tokens::Start,
            '$' => Tokens::End,
            '(' => Tokens::ParenOpen,
            ')' => Tokens::ParenClose,
            '|' => Tokens::BranchOr,
            _ => Tokens::OpenDirection(OpenDirections::from_char(c)),
        })
        .collect();
}

#[derive(Debug, Clone)]
struct Route(Vec<OpenDirections>);

impl Route {
    fn to_string(&self) -> String {
        let directions = &self.0;
        let directions: Vec<String> = directions.iter().map(|d| d.to_string()).collect();

        directions.join("")
    }
}

#[derive(Debug, Clone)]
enum Branches {
    CanSkip(Box<Routes>, Vec<Routes>),
    CannotSkip(Box<Routes>, Vec<Routes>),
}

impl Branches {
    fn to_string(&self) -> String {
        match self {
            Branches::CanSkip(routes, rest_routes) => {
                let rest_str: Vec<String> = rest_routes
                    .iter()
                    .map(|routes| routes.to_string())
                    .collect();

                if rest_str.is_empty() {
                    return format!("{}|", routes.to_string());
                }

                format!("{}|{}|", routes.to_string(), rest_str.join("|"))
            }
            Branches::CannotSkip(routes, rest_routes) => {
                let rest_str: Vec<String> = rest_routes
                    .iter()
                    .map(|routes| routes.to_string())
                    .collect();
                format!("{}|{}", routes.to_string(), rest_str.join("|"))
            }
        }
    }
}

#[derive(Debug, Clone)]
struct BranchGroup(Branches);

impl BranchGroup {
    fn to_string(&self) -> String {
        let branches = &self.0;
        format!("({})", branches.to_string())
    }
}

#[derive(Debug, Clone)]
enum Routes {
    Route(Route, Box<Option<Routes>>),
    Branch(BranchGroup, Box<Option<Routes>>),
}

impl Routes {
    fn to_string(&self) -> String {
        match self {
            Routes::Route(route, routes) => {
                let rest: String = if routes.is_none() {
                    "".to_string()
                } else {
                    routes.clone().unwrap().to_string()
                };
                format!("{}{}", route.to_string(), rest)
            }
            Routes::Branch(branch_group, routes) => {
                let rest: String = if routes.is_none() {
                    "".to_string()
                } else {
                    routes.clone().unwrap().to_string()
                };
                format!("{}{}", branch_group.to_string(), rest)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Directions(Routes);

impl Directions {
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        format!("^{}$", self.0.to_string())
    }
}

type TokenPosition = usize;
type ParseResult<T> = Option<(T, TokenPosition)>;

fn parse_start(tokens: &Vec<Tokens>, start_at: TokenPosition) -> ParseResult<()> {
    match tokens.get(start_at) {
        None => None,
        Some(token) => {
            if token == &Tokens::Start {
                return Some(((), start_at + 1));
            }
            None
        }
    }
}

fn parse_end(tokens: &Vec<Tokens>, start_at: TokenPosition) -> ParseResult<()> {
    match tokens.get(start_at) {
        None => None,
        Some(token) => {
            if token == &Tokens::End {
                return Some(((), start_at + 1));
            }
            None
        }
    }
}

fn parse_route(tokens: &Vec<Tokens>, start_at: TokenPosition) -> ParseResult<Route> {
    let mut current_position = start_at;
    let mut route = vec![];

    loop {
        match tokens.get(current_position) {
            None => {
                break;
            }
            Some(token) => {
                if let Tokens::OpenDirection(direction) = token {
                    route.push(direction.clone());
                    current_position += 1;
                    continue;
                }
                break;
            }
        }
    }

    if route.is_empty() {
        return None;
    }

    let next_position = start_at + route.len();
    let result = Route(route);

    Some((result, next_position))
}

fn parse_branches(tokens: &Vec<Tokens>, start_at: TokenPosition) -> ParseResult<Branches> {
    let mut current_position = start_at;

    let starting_routes: Box<Routes> = match parse_routes(tokens, current_position) {
        None => {
            return None;
        }
        Some((routes, next_position)) => {
            current_position = next_position;
            Box::new(routes)
        }
    };

    match tokens.get(current_position) {
        None => {
            return None;
        }
        Some(token) => {
            if token == &Tokens::BranchOr {
                current_position += 1;
            } else {
                return None;
            }
        }
    }

    let mut can_skip_all = true;
    let mut rest_routes: Vec<Routes> = vec![];

    loop {
        match parse_routes(tokens, current_position) {
            None => {
                break;
            }
            Some((routes, next_position)) => {
                current_position = next_position;
                rest_routes.push(routes);
            }
        }

        match tokens.get(current_position) {
            None => {
                return None;
            }
            Some(token) => {
                match token {
                    Tokens::BranchOr => {
                        current_position += 1;
                        can_skip_all = true;
                    }
                    Tokens::ParenClose => {
                        // this is a peek
                        can_skip_all = false;
                        break;
                    }
                    _ => {
                        return None;
                    }
                }
            }
        }
    }

    match tokens.get(current_position) {
        None => {
            return None;
        }
        Some(token) => match token {
            Tokens::ParenClose => {}
            _ => {
                return None;
            }
        },
    }

    let result = if can_skip_all {
        Branches::CanSkip(starting_routes, rest_routes)
    } else {
        Branches::CannotSkip(starting_routes, rest_routes)
    };

    Some((result, current_position))
}

fn parse_branch_group(tokens: &Vec<Tokens>, start_at: TokenPosition) -> ParseResult<BranchGroup> {
    let mut current_position = start_at;

    match tokens.get(current_position) {
        None => {
            return None;
        }
        Some(token) => {
            if token == &Tokens::ParenOpen {
                current_position += 1;
            } else {
                return None;
            }
        }
    }
    let branches: Branches = match parse_branches(tokens, current_position) {
        None => {
            return None;
        }
        Some((branches, next_position)) => {
            current_position = next_position;
            branches
        }
    };

    match tokens.get(current_position) {
        None => {
            return None;
        }
        Some(token) => {
            if token == &Tokens::ParenClose {
                current_position += 1;
            } else {
                return None;
            }
        }
    }

    let result = (BranchGroup(branches), current_position);
    Some(result)
}

fn parse_routes(tokens: &Vec<Tokens>, start_at: TokenPosition) -> ParseResult<Routes> {
    match parse_branch_group(tokens, start_at) {
        None => {}
        Some((branch_group, next_position)) => match parse_routes(tokens, next_position) {
            None => {
                let result = Routes::Branch(branch_group, Box::new(None));
                return Some((result, next_position));
            }
            Some((routes, next_position)) => {
                let result = Routes::Branch(branch_group, Box::new(Some(routes)));
                return Some((result, next_position));
            }
        },
    }

    match parse_route(tokens, start_at) {
        None => None,
        Some((starting_route, next_position)) => match parse_routes(tokens, next_position) {
            None => {
                let result = Routes::Route(starting_route, Box::new(None));
                Some((result, next_position))
            }
            Some((routes, next_position)) => {
                let result = Routes::Route(starting_route, Box::new(Some(routes)));
                Some((result, next_position))
            }
        },
    }
}

fn parse_input(input_string: &str) -> Directions {
    let tokenized = tokenize(input_string);
    let mut current_position = 0;

    match parse_start(&tokenized, current_position) {
        None => {
            panic!("Expect starting token: ^");
        }
        Some((_, next_position)) => {
            current_position = next_position;
        }
    }

    let routes: Routes = match parse_routes(&tokenized, current_position) {
        None => {
            panic!("No routes generated");
        }
        Some((routes, next_position)) => {
            current_position = next_position;
            routes
        }
    };

    match parse_end(&tokenized, current_position) {
        None => {
            panic!("Expect starting token: $");
        }
        Some((_, _next_position)) => {}
    }

    Directions(routes)
}

type Distance = usize;

#[derive(Debug, PartialEq, Clone)]
enum OpenDirections {
    North,
    South,
    West,
    East,
}

impl OpenDirections {
    fn to_string(&self) -> String {
        let result = match self {
            OpenDirections::North => "N",
            OpenDirections::South => "S",
            OpenDirections::West => "W",
            OpenDirections::East => "E",
        };

        result.to_string()
    }

    fn from_char(d: char) -> Self {
        match d {
            'N' => OpenDirections::North,
            'S' => OpenDirections::South,
            'W' => OpenDirections::West,
            'E' => OpenDirections::East,
            _ => {
                unreachable!();
            }
        }
    }
}

type Coordinate = (i32, i32);

trait Transitions {
    fn north(&self) -> Coordinate;
    fn south(&self) -> Coordinate;
    fn west(&self) -> Coordinate;
    fn east(&self) -> Coordinate;
}

impl Transitions for Coordinate {
    fn north(&self) -> Coordinate {
        let (x, y) = self;
        (*x, y - 1)
    }

    fn south(&self) -> Coordinate {
        let (x, y) = self;
        (*x, y + 1)
    }

    fn west(&self) -> Coordinate {
        let (x, y) = self;
        (x - 1, *y)
    }

    fn east(&self) -> Coordinate {
        let (x, y) = self;
        (x + 1, *y)
    }
}

struct Map {
    room_distance: HashMap<Coordinate, Distance>,
}

impl Map {
    fn new() -> Self {
        Map {
            room_distance: HashMap::new(),
        }
    }

    fn distance_to_farthest_room(&self) -> Distance {
        return *self.room_distance.values().into_iter().max().unwrap();
    }

    fn visit_room(
        &mut self,
        direction: OpenDirections,
        current_position: Coordinate,
    ) -> Coordinate {
        // generate new position

        let new_position = match direction {
            OpenDirections::North => current_position.north(),
            OpenDirections::South => current_position.south(),
            OpenDirections::West => current_position.west(),
            OpenDirections::East => current_position.east(),
        };

        let current_distance = self.room_distance.get(&current_position).unwrap();
        let new_distance = current_distance + 1;

        match self.room_distance.get(&new_position) {
            None => {
                self.room_distance.insert(new_position, new_distance);
            }
            Some(best_distance) => {
                if new_distance < *best_distance {
                    self.room_distance.insert(new_position, new_distance);
                }
            }
        }

        new_position
    }

    fn parse_route(&mut self, route: Route, current_position: Coordinate) -> Coordinate {
        let Route(directions) = route;

        if directions.is_empty() {
            return current_position;
        }

        let mut new_position = current_position;

        for direction in directions {
            new_position = self.visit_room(direction, new_position);
        }

        new_position
    }

    fn generate_positions(
        &mut self,
        choices: Vec<Routes>,
        current_position: Coordinate,
    ) -> HashSet<Coordinate> {
        let new_positions: HashSet<Coordinate> =
            choices
                .into_iter()
                .fold(HashSet::new(), |mut acc, routes_choice| {
                    let new_positions = self.parse_routes(routes_choice, current_position);
                    acc.extend(new_positions);
                    acc
                });

        new_positions
    }

    fn parse_branch_group(
        &mut self,
        branch_group: BranchGroup,
        more_routes: Option<Routes>,
        current_position: Coordinate,
    ) -> HashSet<Coordinate> {
        let BranchGroup(branches) = branch_group;

        let new_positions: HashSet<Coordinate> = match branches {
            Branches::CanSkip(first_choice, other_choices) => {
                let mut choices = vec![*first_choice];
                choices.extend(other_choices);
                let mut new_positions = self.generate_positions(choices, current_position);

                new_positions.insert(current_position);

                new_positions
            }
            Branches::CannotSkip(first_choice, other_choices) => {
                let mut choices = vec![*first_choice];
                choices.extend(other_choices);

                self.generate_positions(choices, current_position)
            }
        };

        match more_routes {
            None => new_positions,
            Some(more_routes) => {
                new_positions
                    .into_iter()
                    .fold(HashSet::new(), |mut acc, position: Coordinate| {
                        let new_positions = self.parse_routes(more_routes.clone(), position);
                        acc.extend(new_positions);
                        acc
                    })
            }
        }
    }

    fn parse_routes(
        &mut self,
        routes: Routes,
        current_position: Coordinate,
    ) -> HashSet<Coordinate> {
        match routes {
            Routes::Route(route, more_routes) => {
                let new_position = self.parse_route(route, current_position);
                match *more_routes {
                    None => {
                        let mut set = HashSet::new();
                        set.insert(new_position);
                        set
                    }
                    Some(more_routes) => self.parse_routes(more_routes, new_position),
                }
            }
            Routes::Branch(branch_group, more_routes) => {
                self.parse_branch_group(branch_group, *more_routes, current_position)
            }
        }
    }

    fn parse_directions(&mut self, directions: Directions) {
        let Directions(routes) = directions;

        let current_position = (0, 0);
        self.room_distance.insert(current_position, 0);

        self.parse_routes(routes, current_position);
    }
}

fn part1(content: String) -> usize {
    let directions = parse_input(content.as_str());
    let mut map = Map::new();
    map.parse_directions(directions);

    map.distance_to_farthest_room()
}

fn part2(content: String) -> usize {
    let directions = parse_input(content.as_str());
    let mut map = Map::new();
    map.parse_directions(directions);

    let num_of_rooms = map
        .room_distance
        .values()
        .into_iter()
        .filter(|x| *x >= &1000)
        .count();

    num_of_rooms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part1() {
        let text = include_str!("./example.data").to_owned();
        assert_eq!(part1(text), 10);
        let text = include_str!("./prod.data").to_owned();
        assert_eq!(part1(text), 3656);
    }

    #[test]
    // #[ignore]
    fn test_part2() {
        let text = include_str!("./example.data").to_owned();
        assert_eq!(part2(text), 0);
        let text = include_str!("./prod.data").to_owned();
        assert_eq!(part2(text), 8430);
    }
}
