use std::collections::HashMap;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let workflows = WorkFlows::new(input);
    let mut input = input.lines();
    for line in input.by_ref() {
        if line.is_empty() {
            break;
        }
    }
    let mut items = vec![];
    for line in input {
        items.push(Item::new(line));
    }

    let mut ans = 0;
    for item in items {
        ans += workflows.eval(&item, &Rule::Name("in".to_string()));
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug)]
struct WorkFlows {
    rules: HashMap<Rule, Vec<(Condition, Rule)>>,
}
impl WorkFlows {
    fn new(input: &str) -> Self {
        let mut rule_set = Self {
            rules: HashMap::new(),
        };
        for line in input.lines() {
            if line.is_empty() {
                break;
            }
            let (name, entry) = Self::get_workflow(line);
            rule_set.rules.insert(name, entry);
        }
        rule_set
    }
    fn get_workflow(line: &str) -> (Rule, Vec<(Condition, Rule)>) {
        let mut workflow = vec![];
        let (name, line) = line.split_once('{').unwrap();
        for line in line.trim_end_matches('}').split(',') {
            let entry = match line {
                _ if line.contains('<') => {
                    let (line, rule) = line.split_once(':').unwrap();
                    let (attr, val) = line.split_once('<').unwrap();
                    (
                        Condition::Lesser(attr.chars().nth(0).unwrap(), val.parse().unwrap()),
                        Rule::new(rule),
                    )
                }
                _ if line.contains('>') => {
                    let (line, rule) = line.split_once(':').unwrap();
                    let (attr, val) = line.split_once('>').unwrap();
                    (
                        Condition::Greater(attr.chars().nth(0).unwrap(), val.parse().unwrap()),
                        Rule::new(rule),
                    )
                }
                _ => (Condition::True, Rule::new(line)),
            };
            workflow.push(entry);
        }
        (Rule::Name(name.to_string()), workflow)
    }
    fn eval(&self, item: &Item, rule: &Rule) -> u32 {
        match rule {
            Rule::Accept => item.get_sum(),
            Rule::Reject => 0,
            _ => {
                for (condition, rule) in &self.rules[&rule] {
                    if condition.is_true(item) {
                        return self.eval(item, rule);
                    }
                }
                0
            }
        }
    }
    fn eval2(&self, item: &Item2, rule: &Rule) {
        match rule {
            Rule::Accept => println!("{:?}", item),
            Rule::Reject => {}
            _ => {
                for (condition, rule) in &self.rules[&rule] {
                    if condition.(item) {}
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Rule {
    Name(String),
    Accept,
    Reject,
}
impl Rule {
    fn new(name: &str) -> Self {
        match name {
            "A" => Rule::Accept,
            "R" => Rule::Reject,
            _ => Rule::Name(name.to_string()),
        }
    }
}

#[derive(Debug)]
enum Condition {
    True,
    Greater(char, u32),
    Lesser(char, u32),
}
impl Condition {
    fn is_true(&self, item: &Item) -> bool {
        match self {
            Condition::True => true,
            Condition::Greater(attr, val) => item.get(attr) > *val,
            Condition::Lesser(attr, val) => item.get(attr) < *val,
        }
    }
    fn eval_and_keep(&self, item: &Item2) -> [Item2;2] {
        match self {
            Condition::True => [item.clone(), item.clone()],
            Condition::Greater(attr, val) => {
                let mut eval = item.clone();
                item.

            }
        }

    }
}

#[derive(Debug)]
struct Item {
    attrs: HashMap<char, u32>,
}
impl Item {
    fn new(line: &str) -> Self {
        let mut item = Self {
            attrs: HashMap::new(),
        };
        for line in line.trim_matches(&['{', '}']).split(',') {
            let (attr, val) = line.split_once('=').unwrap();
            item.attrs
                .insert(attr.chars().nth(0).unwrap(), val.parse().unwrap());
        }
        item
    }
    fn get(&self, attr: &char) -> u32 {
        self.attrs[attr]
    }
    fn get_sum(&self) -> u32 {
        self.attrs.values().sum()
    }
}

#[derive(Debug, Clone)]
struct Item2 {
    attrs: HashMap<char, [u32; 2]>,
}
impl Item2 {
    fn new() -> Self {
        Self {
            attrs: HashMap::from([
                ('x', [1, 4000]),
                ('m', [1, 400]),
                ('a', [1, 4000]),
                ('s', [1, 4000]),
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_create_item() {
        let line = "{x=787,m=2655,a=1222,s=2876}";
        let item = Item::new(line);
        println!("{:?}", item);
        assert_eq!(item.get(&'x'), 787);
    }
}
