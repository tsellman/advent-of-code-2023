use std::collections::{HashMap, HashSet, VecDeque};

use Module::*;

use crate::solutions::Harness;

pub struct Day20 {}

impl Harness for Day20 {
    fn part_1(&self, input: &str, visualise: bool) -> i64 {
        let mut components = parse_configuration(input, visualise);

        let monitor = HashSet::new();

        let mut total_low = 0;
        let mut total_high = 0;
        for _ in 0..1000 {
            let (low, high, _) = push_button(&mut components, &monitor, visualise);
            total_low += low;
            total_high += high;
            if visualise { println!(); }
        }

        total_low * total_high
    }

    fn part_2(&self, input: &str, visualise: bool) -> i64 {
        let mut components = parse_configuration(input, visualise);

        // find the key components to monitor
        let monitor: HashSet<String> = {
            // rx is our target component
            let rx = &components.get("rx");
            if rx.is_none() { return 0; }

            // rx gets a LOW pulse from this conjunction
            let conjunction = rx.unwrap().senders.get(0)
                .map(|n| components.get(n));
            if conjunction.is_none() { return 0; }

            // the conjunction fires LOW when all its inputs are HIGH
            conjunction.unwrap().unwrap().senders.iter().cloned().collect()
        };

        // start pushing the button
        let mut count = 0;
        let mut push_counts = HashMap::new();
        loop {
            let (_, _, detected_highs) = push_button(&mut components, &monitor, visualise);
            count += 1;

            // check if any of our monitored components sent HIGH for first time
            for name in detected_highs {
                if !push_counts.contains_key(&name as &str) {
                    push_counts.insert(name, count);
                }
            }
            // stop when we've found a count for all the relevant inputs
            if push_counts.len() == monitor.len() { break; }
        }

        // answer is first time all the tracked inputs simultaneously fire HIGH
        push_counts.values().product()
    }
}

// -------------------------------------------------------------------------------------------------

/// Push the button to trigger pulses, and update the component state
///
/// # Arguments
/// * `components` - system components (will be updated in place)
/// * `monitor` - detect HIGH messages sent from these components
fn push_button(components: &mut HashMap<String, Component>, monitor: &HashSet<String>, visualise: bool)
               -> (i64, i64, HashSet<String>)
{
    let mut messages: VecDeque<Message> = VecDeque::new();
    messages.push_back(Message { from: "button".to_owned(), to: "broadcaster".to_owned(), value: false });

    let mut low_count = 0_i64;
    let mut high_count = 0_i64;
    let mut monitored_sends = HashSet::new();

    while let Some(message) = messages.pop_front() {
        if message.value { high_count += 1; } else { low_count += 1; }
        if visualise {
            println!("{} -{}-> {}", &message.from, if message.value { "high" } else { "low" }, &message.to);
        }

        if message.value && monitor.contains(&message.from) {
            monitored_sends.insert(message.from.clone());
        }

        if let Some(component) = components.get_mut(&message.to as &str) {
            let outputs = component.receive(message);
            messages.extend(outputs);
        }
    }

    (low_count, high_count, monitored_sends)
}

// -------------------------------------------------------------------------------------------------

// true = HIGH, false = LOW
type Pulse = bool;

#[derive(Debug, Clone)]
struct Message {
    from: String,
    to: String,
    value: Pulse,
}

#[derive(Debug)]
enum Module {
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
    Broadcaster,
    Button,
    Sink,
}

impl Module {
    fn compute(&mut self, input: Message) -> Option<Pulse> {
        match self {
            // broadcaster sends whatever it received
            Broadcaster => Some(input.value),

            // button always sends a LOW pulse
            Button => Some(false),

            // flip-flop ignores HIGH pulses;
            //  LOW pulses turn it on/off and send HIGH/LOW, respectively
            FlipFlop(is_on) => match input.value {
                true => None,
                false => {
                    *is_on = !*is_on;
                    Some(*is_on)
                }
            },

            // conjunction sends LOW if all inputs are HIGH, otherwise HIGH
            Conjunction(mem) => {
                *mem.get_mut(&input.from).unwrap() = input.value;
                Some(!mem.values().all(|v| *v))
            }

            // sink can receive, but never sends
            Sink => None
        }
    }

    fn connect_input(&mut self, name: &str) {
        match self {
            Conjunction(mem) => {
                mem.insert(name.to_owned(), false);
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
struct Component {
    name: String,
    module: Module,
    senders: Vec<String>,
    receivers: Vec<String>,
}

impl Component {
    fn new(name: &str, module: Module, receivers: &str) -> Component {
        let receivers = receivers.split(",").map(|r| r.trim().to_owned()).collect();
        Component { name: name.to_owned(), module, senders: vec![], receivers }
    }

    fn receive(&mut self, message: Message) -> Vec<Message> {
        if let Some(output) = self.module.compute(message) {
            self.receivers.iter()
                .map(|r| Message { from: self.name.clone(), to: r.clone(), value: output })
                .collect()
        } else {
            vec![]
        }
    }

    fn add_senders(&mut self, senders: Vec<String>) {
        for s in &senders { self.module.connect_input(s); }
        self.senders = senders;
    }
}

// -------------------------------------------------------------------------------------------------

fn parse_configuration(config: &str, visualise: bool) -> HashMap<String, Component> {
    let mut components = HashMap::new();

    // button is implied
    components.insert("button".to_owned(), Component::new("button", Button, "broadcaster"));

    // read the components
    for line in config.lines() {
        let (mut name, receivers) = line.split_once(" -> ").unwrap();
        let module;

        if name == "broadcaster" {
            module = Broadcaster;
        } else if name.starts_with("%") {
            name = &name[1..];
            module = FlipFlop(false);
        } else if name.starts_with("&") {
            name = &name[1..];
            module = Conjunction(HashMap::new());
        } else {
            panic!("Unknown module: {}", name);
        }

        components.insert(name.to_owned(), Component::new(name, module, receivers));
    }

    // calculate the senders for each component
    let mut senders = HashMap::new();
    for component in components.values() {
        for receiver in &component.receivers {
            senders.entry(receiver.clone())
                .or_insert(vec![])
                .push(component.name.clone());
        }
    }
    for (name, senders) in senders {
        components.entry(name.clone())
            .or_insert(Component::new(&name, Sink, ""))
            .add_senders(senders);
    }

    if visualise {
        for c in &components {
            println!("{:?}", c);
        }
        println!();
    }

    components
}

// -------------------------------------------------------------------------------------------------

