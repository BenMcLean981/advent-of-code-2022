use std::{cell::RefCell, collections::VecDeque, rc::Rc, str::FromStr};

#[derive(Clone)]
pub(crate) struct Monkey {
    items: VecDeque<usize>,
    pub operation: Rc<RefCell<dyn Fn(usize) -> usize>>,
    pub divisor: usize,
    true_target: usize,
    false_target: usize,
    pub num_inspections: usize,
}

impl Monkey {
    pub fn determine_target(&mut self, item: usize) -> usize {
        self.num_inspections += 1;
        match item % self.divisor == 0 {
            true => self.true_target,
            false => self.false_target,
        }
    }

    pub fn perform_operation(
        &self,
        item: usize,
        reducer: &dyn Fn(usize) -> usize,
    ) -> usize {
        let operated = (self.operation.as_ref().borrow())(item);
        return (reducer)(operated);
    }

    pub fn has_items(&self) -> bool {
        return self.items.len() > 0;
    }

    pub fn pop_item(&mut self) -> usize {
        return self.items.pop_front().unwrap();
    }

    pub fn push_item(&mut self, item: usize) {
        self.items.push_back(item);
    }
}

impl From<&Vec<String>> for Monkey {
    fn from(lines: &Vec<String>) -> Self {
        let items = get_items(&lines[1]);
        let operation = get_operation(&lines[2]);
        let divisor = get_divisor(&lines[3]);
        let true_target = get_target(&lines[4]);
        let false_target = get_target(&lines[5]);

        return Monkey {
            items,
            operation,
            divisor,
            true_target,
            false_target,
            num_inspections: 0,
        };
    }
}

fn get_items(line: &str) -> VecDeque<usize> {
    let split: Vec<&str> = line.split(" ").collect();

    let clean_nums: Vec<String> =
        split[4..].iter().map(|s| s.replace(",", "")).collect();

    return clean_nums
        .iter()
        .map(|s| usize::from_str(&s).unwrap())
        .collect();
}

fn get_operation(line: &str) -> Rc<RefCell<dyn Fn(usize) -> usize>> {
    let split: Vec<&str> = line.split(" ").collect();

    let arg = split[split.len() - 1];
    let arg = usize::from_str(arg);

    let op = split[split.len() - 2];

    // can either change by arg or a the old value
    match op {
        "*" => Rc::new(RefCell::new(move |x| x * arg.as_ref().unwrap_or(&x))),
        "+" => Rc::new(RefCell::new(move |x| x + arg.as_ref().unwrap_or(&x))),
        _ => panic!("Unsupported operation type={}.", op),
    }
}

fn get_divisor(line: &str) -> usize {
    let split: Vec<&str> = line.split(" ").collect();

    let arg = split[split.len() - 1];
    let arg = usize::from_str(arg).unwrap();

    return arg;
}

fn get_target(line: &str) -> usize {
    let split: Vec<&str> = line.split(" ").collect();
    let last = &split[split.len() - 1];

    return usize::from_str(last).unwrap();
}
