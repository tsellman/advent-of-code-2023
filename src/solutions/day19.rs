use std::collections::HashMap;

use Decision::*;
use Rule::*;

use crate::solutions::Harness;

pub struct Day19 {}

impl Harness for Day19 {
    fn part_1(&self, input: &str, _visualise: bool) -> i64 {
        let (workflows, parts) = parse_input(input);

        // identify the accepted parts
        let accepted = parts
            .iter()
            .filter(|p| accept(&workflows, p))
            .collect::<Vec<_>>();

        // calculate the result
        accepted.iter().map(|p| p.tags.values().sum::<i64>()).sum()
    }

    fn part_2(&self, input: &str, _visualise: bool) -> i64 {
        let (workflows, _) = parse_input(input);

        let start = workflows.get("in").unwrap();
        count(&start.rules, &Bounds::new(), &workflows)
    }
}

// ----------------

fn accept(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    evaluate(workflows, part) == Accept
}

fn evaluate(workflows: &HashMap<String, Workflow>, part: &Part) -> Decision {
    let mut workflow = workflows.get("in").unwrap();
    loop {
        let decision = workflow.evaluate(part);
        match decision {
            Accept | Reject => return decision,
            Execute(name) => workflow = workflows.get(&name).unwrap(),
        }
    }
}

// -------------------------------------------------------------------------------------------------
// model

type Tag = char;

struct Part {
    tags: HashMap<Tag, i64>,
}

impl Part {
    fn tag(&self, t: &Tag) -> i64 {
        *self.tags.get(t).unwrap()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Decision {
    Execute(String),
    Accept,
    Reject,
}

impl From<&str> for Decision {
    fn from(value: &str) -> Self {
        match value {
            "A" => Accept,
            "R" => Reject,
            name => Execute(name.to_owned()),
        }
    }
}

enum Rule {
    LessThan(Tag, i64, Decision),
    GreaterThan(Tag, i64, Decision),
    Decide(Decision),
}

impl Rule {
    /// Apply this rule to the given part, maybe making a decision
    fn apply(&self, part: &Part) -> Option<Decision> {
        match self {
            LessThan(t, c, d) if part.tag(t) < *c => Some(d.clone()),
            GreaterThan(t, c, d) if part.tag(t) > *c => Some(d.clone()),
            Decide(d) => Some(d.clone()),
            _ => None,
        }
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if value.contains("<") {
            let (tag, rest) = value.split_once("<").unwrap();
            let (cmp, res) = rest.split_once(":").unwrap();
            LessThan(to_tag(tag), cmp.parse().unwrap(), Decision::from(res))
        } else if value.contains(">") {
            let (tag, rest) = value.split_once(">").unwrap();
            let (cmp, res) = rest.split_once(":").unwrap();
            GreaterThan(to_tag(tag), cmp.parse().unwrap(), Decision::from(res))
        } else {
            Decide(Decision::from(value))
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    /// Run the given part through this workflow to reach a decision
    fn evaluate(&self, part: &Part) -> Decision {
        for rule in &self.rules {
            if let Some(decision) = rule.apply(part) {
                return decision;
            }
        }
        Reject
    }
}

#[derive(Debug, Clone)]
struct Bounds {
    tags: HashMap<Tag, (i64, i64)>,
}

impl Bounds {
    fn new() -> Bounds {
        Bounds { tags: HashMap::new() }
    }

    fn _lt(&mut self, tag: Tag, value: i64) {
        let (_, max) = self.tags.entry(tag).or_insert((1, 4000));
        if value < *max {
            *max = value;
        }
    }

    fn _gt(&mut self, tag: Tag, value: i64) {
        let (min, _) = self.tags.entry(tag).or_insert((1, 4000));
        if value > *min {
            *min = value;
        }
    }

    fn constrain(&mut self, rule: &Rule, invert: bool) {
        match rule {
            LessThan(t, v, _) if invert => self._gt(*t, *v),
            LessThan(t, v, _) => self._lt(*t, *v - 1),
            GreaterThan(t, v, _) if invert => self._lt(*t, *v),
            GreaterThan(t, v, _) => self._gt(*t, *v + 1),
            _ => {}
        }
    }
}

fn count(rules: &[Rule], bounds: &Bounds, workflows: &HashMap<String, Workflow>) -> i64 {
    if rules.is_empty() { return 0; }

    let rule = &rules[0];

    // treat 'true' as left
    let mut l_bounds = bounds.clone();
    l_bounds.constrain(rule, false);
    let l_count = match rule {
        Decide(Accept) => combinations(&l_bounds),
        LessThan(_, _, d) | GreaterThan(_, _, d) | Decide(d) => match d {
            Reject => 0,
            Accept => combinations(&l_bounds),
            Execute(name) => {
                let rules = &workflows.get(name).unwrap().rules;
                count(&rules, &l_bounds, &workflows)
            }
        }
    };

    // and 'false' as right
    let mut r_bounds = bounds.clone();
    r_bounds.constrain(rule, true);
    let r_count = count(&rules[1..], &r_bounds, &workflows);

    l_count + r_count
}

fn combinations(bounds: &Bounds) -> i64 {
    let mut product = 1;
    for t in ['x', 'm', 'a', 's'] {
        let (min, max) = bounds.tags.get(&t).unwrap_or(&(1, 4000));
        product *= max - min + 1;
    }
    product
}

// -------------------------------------------------------------------------------------------------
// parsing

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("{") {
            parts.push(parse_part(line));
        } else {
            let workflow = parse_workflow(line);
            workflows.insert(workflow.name.clone(), workflow);
        }
    }

    (workflows, parts)
}

fn parse_part(line: &str) -> Part {
    let tags = line
        .split(",")
        .map(|s| if s.starts_with("{") { &s[1..] } else { s })
        .map(|s| {
            if s.ends_with("}") {
                &s[..s.len() - 1]
            } else {
                s
            }
        })
        .map(|p| p.split_once("=").unwrap())
        .map(|(t, v)| (to_tag(t), v.parse().unwrap()))
        .collect();

    Part { tags }
}

fn parse_workflow(line: &str) -> Workflow {
    let (name, rules) = line.split_once("{").unwrap();

    let rules = rules[..rules.len() - 1]
        .split(",")
        .map(|r| Rule::from(r))
        .collect();

    Workflow {
        name: name.to_owned(),
        rules,
    }
}

fn to_tag(value: &str) -> Tag {
    value.chars().nth(0).unwrap()
}
