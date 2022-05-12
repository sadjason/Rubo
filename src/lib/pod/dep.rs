use std::borrow::Borrow;
use std::collections::HashMap;
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;
use anyhow;
use super::lockfile::Lockfile;
use crate::lib::pod::lockfile::PodItem;

pub(crate) fn print_deps<P: AsRef<Path>>(
    path: P,
    target: &str,
    max_depth: usize
) -> anyhow::Result<()> {
    travel_and_print(path, target, max_depth, TravelMode::Children)
}

pub(crate) fn print_reserve_deps<P: AsRef<Path>>(
    path: P,
    target: &str,
    max_depth: usize
) -> anyhow::Result<()> {
    travel_and_print(path, target, max_depth, TravelMode::Parents)
}

fn travel_and_print<P: AsRef<Path>>(
    path: P,
    target: &str,
    max_depth: usize,
    mode: TravelMode
) -> anyhow::Result<()> {
    let lockfile = Lockfile::from_file(path)?;
    let pods = lockfile.pods()?;

    let mut result = TravelResult::new(RefCell::new(HashMap::new()));
    let travel = Travel::new(mode,&pods);
    travel.collect(
        target.to_string(),
        "".to_string(),
        &result
    );
    let r = Rc::borrow(&mut result);
    let mut r = RefCell::borrow_mut(r);
    r.remove(&target.to_string());
    let mut chains = r.values()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>();
    chains.sort();
    printer::print_pretty_chains(chains.into_iter(), max_depth);
    Ok(())
}

struct Travel<'a> {
    mode: TravelMode,
    source: &'a HashMap<String, PodItem>,
}

enum TravelMode { Parents, Children }
type TravelResult = Rc<RefCell<HashMap<String, String>>>;

impl<'a> Travel<'a> {
    fn new(mode: TravelMode, source: &'a HashMap<String, PodItem>) -> Self {
        Self { mode, source }
    }

    fn collect(&self, target: String, chain: String, result: &TravelResult) {
        {
            let r = Rc::borrow(result);
            let r = RefCell::borrow(r);
            if r.contains_key(&target) {
                return
            }
        }
        {
            let mut rt = result.borrow_mut();
            rt.insert(target.clone(), chain.clone());
        }
        let pod = self.source.get(&target).unwrap();
        let iter =
            match self.mode {
                TravelMode::Parents => { &pod.parents },
                TravelMode::Children => { &pod.children }
            };
        for c in iter {
            let chain = format!("{}:{}", chain, c);
            self.collect(c.clone(), chain, result);
        }
    }
}

mod printer {
    use ansi_term::{Colour, Style};

    pub(super) fn print_pretty_chains<'a, T>(chains: T, max_depth: usize)
        where T: IntoIterator<Item = &'a str> {
        for chain in chains {
            let comps = chain.split(":").collect::<Vec<&str>>();
            let mut depth = comps.len();
            if depth <= 1 { continue }
            depth -= 1; // fix depth
            if depth > max_depth {
                continue
            }
            let text= comps.last().unwrap();
            print_depth_text(text, depth);
        }
    }

    const LEVEL_COLORS: [Colour; 6] = [
        Colour::Red, Colour::Green, Colour::Yellow,
        Colour::Blue, Colour::Purple, Colour::Cyan
    ];

    fn print_depth_text(text: &str, depth: usize) {
        let bullet = "•";
        let prefix = std::iter::repeat(" ").take((depth - 1) * 4).collect::<String>();
        let color = LEVEL_COLORS[(depth - 1) % LEVEL_COLORS.len()];
        println!(
            "{} {} {}",
            prefix,
            Style::from(color).bold().paint(bullet).to_string(),
            Style::from(color).paint(text).to_string()
        );
    }
}