use std::{collections::BTreeMap, fmt::Display, ops::Range, time::SystemTime};

use super::Pos;

#[derive(Debug, Copy, Clone)]
pub enum Edge {
    // `F`
    SouthEast,
    // `L`
    NorthEast,
    // `7`
    SouthWest,
    // `J`
    NorthWest,
    // `|`
    NorthSouth,
    // `-`
    EastWest,
}

pub type EdgeMap<A> = BTreeMap<Pos<A>, Edge>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Out,
    BottomBorder,
    In,
    TopBorder,
}

#[allow(unused_variables)]
pub trait AreaListener<A> {
    fn on_edge(&mut self, xy: Pos<A>, edge: Edge) {}
    fn on_inside(&mut self) {}
    fn on_outside(&mut self) {}
    fn on_newline(&mut self) {}
    fn on_done(&mut self) {}
}

pub struct Noop;

impl<A> AreaListener<A> for Noop {}

pub struct Drawing {
    path: String,
    content: String,
}

impl Drawing {
    pub fn new(day: usize) -> Self {
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let path = format!("res/day{}/output{}.txt", day, ts.as_micros());
        Self {
            path,
            content: String::new(),
        }
    }
}

impl<A: Display> AreaListener<A> for Drawing {
    fn on_edge(&mut self, (x, y): Pos<A>, edge: Edge) {
        eprintln!("{edge:?} {x},{y}");
        use Edge::*;
        self.content.push(match edge {
            SouthEast => '╔',
            NorthEast => '╚',
            SouthWest => '╗',
            NorthWest => '╝',
            NorthSouth => '║',
            EastWest => '═',
        });
    }

    fn on_inside(&mut self) {
        self.content.push('i');
    }

    fn on_outside(&mut self) {
        self.content.push(' ');
    }

    fn on_newline(&mut self) {
        self.content.push('\n');
    }

    fn on_done(&mut self) {
        std::fs::write(&self.path, &self.content).unwrap();
    }
}

pub fn area<A, L: AreaListener<A>>(
    xrange: Range<A>,
    yrange: Range<A>,
    border: &EdgeMap<A>,
    listener: &mut L,
) -> usize
where
    A: Copy,
    Pos<A>: Ord,
    Range<A>: Iterator<Item = A> + Clone,
{
    let mut in_fields = 0;
    for y in yrange {
        use Edge::*;
        use State::*;
        let mut state = Out;
        for x in xrange.clone() {
            if let Some(edge) = border.get(&(x, y)).copied() {
                listener.on_edge((x, y), edge);
                state = match (state, edge) {
                    (Out, SouthEast) => TopBorder,
                    (Out, NorthEast) => BottomBorder,
                    (Out, SouthWest | NorthWest | EastWest) => unreachable!(),
                    (Out, NorthSouth) => In,
                    (TopBorder, SouthEast | NorthEast | NorthSouth) => unreachable!(),
                    (TopBorder, SouthWest) => Out,
                    (TopBorder, NorthWest) => In,
                    (TopBorder, EastWest) => TopBorder,
                    (In, SouthEast) => BottomBorder,
                    (In, NorthEast) => TopBorder,
                    (In, SouthWest | NorthWest | EastWest) => unreachable!(),
                    (In, NorthSouth) => Out,
                    (BottomBorder, SouthEast | NorthEast | NorthSouth) => unreachable!(),
                    (BottomBorder, SouthWest) => In,
                    (BottomBorder, NorthWest) => Out,
                    (BottomBorder, EastWest) => BottomBorder,
                };
            } else if state == In {
                listener.on_inside();
                in_fields += 1;
            } else {
                listener.on_outside();
            }
        }
        listener.on_newline();
    }
    listener.on_done();
    in_fields
}
