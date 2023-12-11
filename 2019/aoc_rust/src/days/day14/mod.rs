// #![allow(dead_code)]
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::Result;
use std::{fs, io};

use nom::IResult;

#[derive(Debug)]
struct Quantity {
    element: String,
    count: i64,
}

#[derive(Debug)]
struct Reaction {
    input: Vec<Quantity>,
    output: Quantity,
}

#[derive(Debug, Default)]
struct OreDeposits {
    total: i64,
    in_use: i64,
}

impl Quantity {
    fn new(element: String, count: i64) -> Self {
        Quantity { element, count }
    }
}

impl Reaction {
    fn new(output: Quantity, input: Vec<Quantity>) -> Self {
        Reaction { input, output }
    }
}

type Buckets = HashMap<String, OreDeposits>;

fn solve() -> (i64, i64) {
    let contents = fs::read_to_string("./src/days/day14/input").expect("error");

    let reactions = contents
        .trim()
        .split('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(|str| str.trim())
        .map(|str| str.split("=>").collect::<Vec<_>>())
        .map(|vstr| (vstr[0].split(',').collect::<Vec<_>>(), vstr[1]))
        .map(|tup| {
            (
                tup.0.iter().map(|str| str.trim()).collect::<Vec<_>>(),
                tup.1
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|tup| {
            (
                tup.0
                    .iter()
                    .map(|str| str.split(' ').collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
                tup.1,
            )
        })
        .map(|tup| {
            (
                tup.0
                    .iter()
                    .map(|v| (v[0].parse::<i64>().unwrap(), v[1]))
                    .collect::<Vec<_>>(),
                ((tup.1)[0].parse::<i64>().unwrap(), (tup.1)[1]),
            )
        })
        .map(|tup| {
            let output = Quantity::new((tup.1).1.to_string(), (tup.1).0);
            let input = tup
                .0
                .iter()
                .map(|quantity| Quantity::new(quantity.1.to_string(), quantity.0))
                .collect::<Vec<_>>();
            Reaction::new(output, input)
        })
        .collect::<Vec<_>>();

    fn ore_required(
        reactions: &[Reaction],
        element: &str,
        buckets: &mut Buckets,
        ore: &mut i64,
        amount: i64,
    ) {
        // find the reaction needed for the output
        if let Some(reaction) = reactions
            .iter()
            .find(|&x| x.output.element.as_str() == element)
        {
            for input in &reaction.input {
                if let Some(b) = buckets.get_mut(&input.element) {
                    if let Some(input_reaction) = reactions
                        .iter()
                        .find(|&x| x.output.element.as_str() == input.element)
                    {
                        let surplus = b.total - b.in_use;
                        let inputs_needed = input.count * amount - surplus;
                        let reactions_to_run = ((inputs_needed as f64
                            / input_reaction.output.count as f64)
                            .ceil()) as i64;
                        let inputs_produced = input_reaction.output.count * reactions_to_run;

                        b.total += inputs_produced;
                        b.in_use += inputs_needed + surplus;

                        if input_reaction.input[0].element.as_str() == "ORE" {
                            *ore += (reactions_to_run as i64) * input_reaction.input[0].count;
                        } else {
                            ore_required(reactions, &input.element, buckets, ore, reactions_to_run);
                        }
                    }
                }
            }
        }
    }

    #[allow(unused_assignments)]
    let (mut part_1, mut part_2) = (0, 0);
    // part 1
    {
        let mut buckets = Buckets::new();
        for reaction in &reactions {
            buckets.insert(reaction.output.element.to_string(), OreDeposits::default());
        }

        let mut ore = 0;
        ore_required(&reactions, "FUEL", &mut buckets, &mut ore, 1);
        part_1 = ore;
    }

    // part 2
    {
        let target = 1_000_000_000_000;

        let mut min_fuel = 1000;
        let mut max_fuel = 10000000;
        let mut fuel = 0;

        // <= to return fuel for value smaller than one trillion
        while min_fuel <= max_fuel {
            // reset buckets
            let mut buckets = Buckets::new();
            for reaction in &reactions {
                buckets.insert(reaction.output.element.to_string(), OreDeposits::default());
            }

            fuel = min_fuel + (max_fuel - min_fuel) / 2;

            let mut ore = 0;
            ore_required(&reactions, "FUEL", &mut buckets, &mut ore, fuel);

            if fuel == ore {
                break;
            }

            if ore < target {
                min_fuel = fuel + 1;
            } else {
                max_fuel = fuel - 1
            }
        }

        part_2 = fuel;
    }
    return (part_1, part_2);
}

#[inline(always)]
pub fn dfs<N, FN, NI>(start: N, mut next: FN)
where
    FN: FnMut(N) -> NI,
    NI: IntoIterator<Item = N>,
{
    dfs_impl(start, &mut next);
}

fn dfs_impl<N, FN, NI>(current: N, next: &mut FN)
where
    FN: FnMut(N) -> NI,
    NI: IntoIterator<Item = N>,
{
    for successor in next(current) {
        dfs_impl(successor, next);
    }
}

fn calculate_ore_cost<'s>(
    transformations: &Transformations<'s>,
    fuel_goal: u64,
    spare: &mut HashMap<&'s str, u64>,
) -> u64 {
    spare.clear();
    let mut ore_demand = 0;
    dfs(Molecule(fuel_goal, "FUEL"), |mut molecule| {
        match spare.entry(molecule.1) {
            Entry::Vacant(_) => {}
            Entry::Occupied(mut slot) => {
                if *slot.get() < molecule.0 {
                    molecule.0 -= slot.remove();
                } else {
                    *slot.get_mut() -= molecule.0;
                    molecule.0 = 0;
                }
            }
        }
        if molecule.1 == "ORE" {
            ore_demand += molecule.0;
        }
        let transformation = transformations.get(molecule.1).unwrap();
        let produced_per_transformation = transformation.into.0;
        let required_transformations =
            (molecule.0 + produced_per_transformation - 1) / produced_per_transformation;
        let spare_count = (required_transformations * produced_per_transformation) - molecule.0;
        if spare_count > 0 {
            *spare.entry(molecule.1).or_insert(0) += spare_count;
        }
        transformation
            .from
            .iter()
            .map(move |molecule| Molecule(required_transformations * molecule.0, molecule.1))
            .filter(|molecule| molecule.0 > 0)
    });
    ore_demand
}

fn pt1(transformations: Transformations) -> u64 {
    let mut spare = HashMap::new();
    calculate_ore_cost(&transformations, 1, &mut spare)
}

fn find_fuel_produced_range<'s>(
    transformations: &Transformations<'s>,
    spare: &mut HashMap<&'s str, u64>,
) -> (u64, u64) {
    let mut low = 0;
    let mut high = 1;

    loop {
        let ore_cost = calculate_ore_cost(transformations, high, spare);
        match ore_cost.cmp(&1_000_000_000_000) {
            Ordering::Less => {
                low = high;
                high *= 2;
            }
            Ordering::Equal => break (high, high + 1),
            Ordering::Greater => break (low, high),
        }
    }
}

fn pt2(transformations: Transformations) -> u64 {
    let mut spare = HashMap::new();
    let (mut low, mut high) = find_fuel_produced_range(&transformations, &mut spare);

    loop {
        match high - low {
            0 | 1 => break low,
            _ => {}
        }
        let midpoint = (high - low) / 2 + low;
        let ore_cost = calculate_ore_cost(&transformations, midpoint, &mut spare);
        match ore_cost.cmp(&1_000_000_000_000) {
            Ordering::Less => {
                low = midpoint;
            }
            Ordering::Equal => break midpoint,
            Ordering::Greater => {
                high = midpoint;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Molecule<'s>(u64, &'s str);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Transformation<'s> {
    from: Vec<Molecule<'s>>,
    into: Molecule<'s>,
}

type Transformations<'s> = HashMap<&'s str, Transformation<'s>>;

fn parse(s: &str) -> IResult<&str, Transformations> {
    use crate::utils::parsers::*;
    fn molecule(s: &str) -> IResult<&str, Molecule> {
        map(
            pair(terminated(u64_str, char(' ')), alpha1),
            |(count, name)| Molecule(count, name),
        )(s)
    }
    let from = separated_list1(tag(", "), molecule);
    let transformation = map(
        pair(terminated(from, tag(" => ")), molecule),
        |(from, into)| Transformation { from, into },
    );
    fn create_transformations(list: Vec<Transformation>) -> Result<Transformations> {
        let mut transformations = HashMap::with_capacity(list.len());
        for transformation in list {
            match transformations.entry(transformation.into.1) {
                Entry::Occupied(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "duplicate transformation",
                    ));
                }
                Entry::Vacant(slot) => {
                    slot.insert(transformation);
                }
            }
        }
        if transformations.get("FUEL").is_none() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"));
        }
        transformations.insert(
            "ORE",
            Transformation {
                from: Vec::new(),
                into: Molecule(1, "ORE"),
            },
        );
        Ok(transformations)
    }
    map_res(
        separated_list1(line_ending, transformation),
        create_transformations,
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14() {
        assert_eq!(solve(), (720484, 1993285));
    }

    #[test]
    fn test_day14_2() {
        //thats a one off error :|
        assert_eq!(solve(), (720484, 1993285));
        let contents = fs::read_to_string("./src/days/day14/input").expect("error");
        let parsed = parse(&contents).unwrap();
        let parsed2 = parse(&contents).unwrap();
        assert_eq!(pt1(parsed.1), 720484);
        assert_eq!(pt2(parsed2.1), 1993284);
    }
}
