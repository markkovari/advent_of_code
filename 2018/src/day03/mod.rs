use std::{
    collections::{HashMap, HashSet},
    vec,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rule {
    id: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl TryFrom<&str> for Rule {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // it was GitHub Copilot who suggested this implementation, I trust it with every bit of my heart
        let mut parts = value.split(' ');
        let id = parts.next().ok_or("No id")?;
        let _ = parts.next().ok_or("No @")?;
        let mut coords = parts.next().ok_or("No coords")?.split(',');
        let x = coords
            .next()
            .ok_or("No x")?
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        let y = coords
            .next()
            .ok_or("No y")?
            .trim_end_matches(':')
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        let mut size = parts.next().ok_or("No size")?.split('x');
        let width = size
            .next()
            .ok_or("No width")?
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        let height = size
            .next()
            .ok_or("No height")?
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        Ok(Rule {
            id: id.to_owned(),
            x,
            y,
            width,
            height,
        })
    }
}

fn read_rules(is_prod: bool) -> Vec<Rule> {
    let mut content = include_str!("./example.data");
    if is_prod {
        content = include_str!("./prod.data");
    }
    content = content.trim();
    content
        .lines()
        .map(|s| Rule::try_from(s).unwrap())
        .collect::<Vec<Rule>>()
}

fn touched_fields_by(by_rules: Vec<Rule>) -> HashMap<(i32, i32), Vec<String>> {
    let mut touched_fields = HashMap::new();
    for rule in by_rules {
        for x in rule.x..rule.x + rule.width {
            for y in rule.y..rule.y + rule.height {
                let counter = touched_fields.entry((x, y)).or_insert(vec![]);
                counter.push(rule.id.clone());
            }
        }
    }
    touched_fields
}

fn get_overlapping(touched_fields: &HashMap<(i32, i32), Vec<String>>) -> usize {
    touched_fields
        .iter()
        .filter(|(_, claims)| claims.len() > 1)
        .count()
}

fn get_non_overlapping(
    touched_fields: &HashMap<(i32, i32), Vec<String>>,
    rules: &Vec<Rule>,
) -> Vec<Rule> {
    let mut non_overlapping: Vec<Rule> = vec![];
    for rule in rules {
        let mut is_overlapping = false;
        for x in rule.x..rule.x + rule.width {
            for y in rule.y..rule.y + rule.height {
                let claims = touched_fields.get(&(x, y)).unwrap();
                if claims.len() > 1 {
                    is_overlapping = true;
                    break;
                }
            }
            if is_overlapping {
                break;
            }
        }
        if !is_overlapping {
            non_overlapping.push(rule.clone());
        }
    }
    non_overlapping
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let rules = read_rules(false);
        let touched_fields = touched_fields_by(rules);
        assert_eq!(get_overlapping(&touched_fields), 4);
        let rules = read_rules(true);
        let touched_fields = touched_fields_by(rules);
        assert_eq!(get_overlapping(&touched_fields), 100595);
    }

    #[test]
    fn test_second() {
        let rules = read_rules(false);
        let touched_fields = &touched_fields_by(rules.clone());
        let non_overlapping = get_non_overlapping(touched_fields, &rules);
        assert_eq!(non_overlapping.len(), 1);
        assert_eq!(non_overlapping[0], Rule::try_from("#3 @ 5,5: 2x2").unwrap());
        let rules = read_rules(true);
        let touched_fields = &touched_fields_by(rules.clone());
        let non_overlapping = get_non_overlapping(touched_fields, &rules);
        assert_eq!(non_overlapping.len(), 1);
        assert_eq!(
            non_overlapping[0],
            Rule::try_from("#415 @ 517,356: 21x10").unwrap()
        );
    }
}
