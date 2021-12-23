use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Position {
    Hallway(u8),
    RoomA(u8),
    RoomB(u8),
    RoomC(u8),
    RoomD(u8),
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let short_code = match self {
            Hallway(..) => "H",
            RoomA(..) => "A",
            RoomB(..) => "B",
            RoomC(..) => "C",
            RoomD(..) => "D",
        };
        write!(f, "{}{}", short_code, self.index())
    }
}

impl Position {
    fn index(&self) -> u8 {
        match self {
            Hallway(p) => *p,
            RoomA(p) => *p,
            RoomB(p) => *p,
            RoomC(p) => *p,
            RoomD(p) => *p,
        }
    }
    fn occupant(&self, pods: &[Amphipod]) -> Option<Amphipod> {
        pods.iter().find(|it| &it.pos == self).cloned()
    }

    fn occupied(&self, positions: &[Amphipod]) -> bool {
        self.occupant(positions).is_some()
    }

    fn is_in_hallway(&self) -> bool {
        matches!(self, Hallway(..))
    }

    fn is_in_door(&self) -> bool {
        matches!(self, Hallway(2) | Hallway(4) | Hallway(6) | Hallway(8))
    }

    #[allow(clippy::match_like_matches_macro)]
    fn same_room(&self, other: &Position) -> bool {
        match (self, other) {
            (RoomA(..), RoomA(..)) => true,
            (RoomB(..), RoomB(..)) => true,
            (RoomC(..), RoomC(..)) => true,
            (RoomD(..), RoomD(..)) => true,
            _ => false,
        }
    }
}

use Position::*;

trait MovementCost {
    fn cost(&self) -> u32;
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum AmphipodType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

use AmphipodType::*;

impl Display for AmphipodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Amber => "A",
                Bronze => "B",
                Copper => "C",
                Desert => "D",
            }
        )
    }
}

impl MovementCost for AmphipodType {
    fn cost(&self) -> u32 {
        match self {
            Amber => 1,
            Bronze => 10,
            Copper => 100,
            Desert => 1000,
        }
    }
}

type AmphipodID = u8;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Amphipod {
    id: AmphipodID,
    t: AmphipodType,
    pos: Position,
    is_stuck_in_hallway: bool,
    stopped_when_stuck_in_hallway: bool,
}

impl MovementCost for Amphipod {
    fn cost(&self) -> u32 {
        self.t.cost()
    }
}

impl Amphipod {
    fn new(id: AmphipodID, t: AmphipodType, pos: Position) -> Amphipod {
        Amphipod {
            id,
            t,
            pos,
            is_stuck_in_hallway: false,
            stopped_when_stuck_in_hallway: false,
        }
    }
    fn is_room_match(&self, position: &Position) -> bool {
        match self.t {
            Amber => matches!(position, RoomA(..)),
            Bronze => matches!(position, RoomB(..)),
            Copper => matches!(position, RoomC(..)),
            Desert => matches!(position, RoomD(..)),
        }
    }
    fn is_home(&self) -> bool {
        self.is_room_match(&self.pos)
    }
    fn is_deep_home(&self) -> bool {
        self.is_home() && self.pos.index() == 0u8
    }
    fn is_shallow_home(&self) -> bool {
        self.is_home() && self.pos.index() == 1u8
    }
    fn deep_home(&self) -> Position {
        match self.t {
            Amber => RoomA(0),
            Bronze => RoomB(0),
            Copper => RoomC(0),
            Desert => RoomD(0),
        }
    }
    fn shallow_home(&self) -> Position {
        match self.t {
            Amber => RoomA(1),
            Bronze => RoomB(1),
            Copper => RoomC(1),
            Desert => RoomD(1),
        }
    }
    fn home_unoccupied_or_match(&self, pods: &[Amphipod]) -> bool {
        self.shallow_home()
            .occupant(pods)
            .map(|it| it.is_home())
            .unwrap_or(true)
            && self
                .deep_home()
                .occupant(pods)
                .map(|it| it.is_home())
                .unwrap_or(true)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    amphipods: Vec<Amphipod>,
    last_amphipod_moved: Option<Amphipod>,
}

#[derive(Clone, Copy, Debug)]
struct Move {
    source: Amphipod,
    dest: Amphipod,
    score: u32,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}({}): {} -> {} ",
            self.source.id, self.source.t, self.source.pos, self.dest.pos
        )
    }
}

fn pos_to_num(pod: &Amphipod) -> u64 {
    let pos = pod.pos;
    (match pos {
        Hallway(num) => num,
        RoomA(num) => 11 + num,
        RoomB(num) => 13 + num,
        RoomC(num) => 15 + num,
        RoomD(num) => 17 + num,
    }) as u64
        + (if pod.stopped_when_stuck_in_hallway {
            32
        } else {
            0
        })
        + (if pod.is_stuck_in_hallway { 64 } else { 0 })
}

impl State {
    fn pos_hash(&self) -> u64 {
        self.amphipods
            .iter()
            .fold(0u64, |acc, el| acc + (pos_to_num(el) << (7 * el.id)))
            + (self
                .last_amphipod_moved
                .map(|it| pos_to_num(&it))
                .unwrap_or(127)
                << 56)
    }
    fn is_complete(&self) -> bool {
        self.amphipods.iter().all(|it| it.is_home())
    }

    fn legal_moves_for_amphipod(&self, amphipod: &Amphipod) -> Vec<Move> {
        if amphipod.is_deep_home() {
            return vec![];
        }

        if amphipod.is_shallow_home()
            && amphipod
                .deep_home()
                .occupant(&self.amphipods)
                .map(|occ| occ.is_home())
                .unwrap_or_default()
        {
            return vec![];
        }

        let possible_targets = match amphipod.pos {
            RoomA(0) => vec![RoomA(1)],
            RoomA(1) => vec![RoomA(0), Hallway(2)],
            RoomB(0) => vec![RoomB(1)],
            RoomB(1) => vec![RoomB(0), Hallway(4)],
            RoomC(0) => vec![RoomC(1)],
            RoomC(1) => vec![RoomC(0), Hallway(6)],
            RoomD(0) => vec![RoomD(1)],
            RoomD(1) => vec![RoomD(0), Hallway(8)],
            Hallway(0) => vec![Hallway(1)],
            Hallway(10) => vec![Hallway(9)],
            Hallway(2) => vec![Hallway(1), Hallway(3), RoomA(1)],
            Hallway(4) => vec![Hallway(3), Hallway(5), RoomB(1)],
            Hallway(6) => vec![Hallway(5), Hallway(7), RoomC(1)],
            Hallway(8) => vec![Hallway(7), Hallway(9), RoomD(1)],
            Hallway(n) => vec![Hallway(n - 1), Hallway(n + 1)],
            _ => panic!("Invalid pos {:?}", amphipod.pos),
        };
        possible_targets
            .iter()
            .filter(|it| {
                it.is_in_hallway()
                    || (amphipod.is_room_match(it)
                        && amphipod.home_unoccupied_or_match(&self.amphipods))
                    || amphipod.pos.same_room(it)
            })
            .filter(|it| !it.occupied(&self.amphipods))
            .map(|pos| Move {
                source: *amphipod,
                dest: Amphipod {
                    pos: *pos,
                    ..*amphipod
                },
                score: amphipod.cost(),
            })
            .collect()
    }

    fn legal_moves(&self) -> Vec<Move> {
        if let Some(last) = self.last_amphipod_moved {
            if last.pos.is_in_door()
                || (last.pos.is_in_hallway() && last.stopped_when_stuck_in_hallway)
            {
                return self.legal_moves_for_amphipod(&last);
            }
        }

        // Always favor moving to the deep home spot from the shallow home if possible.
        for amphipod in self.amphipods.iter() {
            if amphipod.is_shallow_home() && !amphipod.deep_home().occupied(&self.amphipods) {
                return vec![Move {
                    source: *amphipod,
                    dest: Amphipod {
                        pos: amphipod.deep_home(),
                        ..*amphipod
                    },
                    score: amphipod.cost(),
                }];
            }
        }

        let mut moves = vec![];

        for amphipod in self.amphipods.iter() {
            moves.append(&mut self.legal_moves_for_amphipod(amphipod));
        }

        moves
    }

    fn apply_move(&self, mv: Move) -> State {
        let mut should_set_stuck_flag = false;
        if let Some(prev) = self.last_amphipod_moved {
            if prev.id != mv.source.id && prev.is_stuck_in_hallway {
                should_set_stuck_flag = true;
            }
        }
        let mut next_amphipods: Vec<Amphipod> = self
            .amphipods
            .iter()
            .filter(|it| {
                (it.id != mv.dest.id)
                    && (!should_set_stuck_flag
                        || it.id != self.last_amphipod_moved.map(|prev| prev.id).unwrap_or(255))
            })
            .cloned()
            .collect();

        let is_stuck_in_hallway = mv.dest.pos.is_in_hallway();

        if should_set_stuck_flag {
            next_amphipods.push(Amphipod {
                stopped_when_stuck_in_hallway: true,
                ..self.last_amphipod_moved.unwrap()
            });
        }

        let next_amphipod = Amphipod {
            is_stuck_in_hallway,
            stopped_when_stuck_in_hallway: mv.dest.stopped_when_stuck_in_hallway
                && mv.dest.pos.is_in_hallway(),
            ..mv.dest
        };
        next_amphipods.push(next_amphipod);

        State {
            amphipods: next_amphipods,
            last_amphipod_moved: Some(next_amphipod),
        }
    }
}

fn solve(
    from_state: &State,
    mut seen: HashSet<u64>,
    result_cache: &mut HashMap<u64, Option<(Vec<Move>, u32)>>,
) -> Option<(Vec<Move>, u32)> {
    let pos_hash = from_state.pos_hash();
    if from_state.is_complete() {
        result_cache.insert(pos_hash, Some((vec![], 0)));
        return Some((vec![], 0));
    }
    if result_cache.contains_key(&pos_hash) {
        return result_cache.get(&pos_hash).and_then(|it| it.clone());
    }
    if seen.contains(&pos_hash) {
        return None;
    }
    seen.insert(pos_hash);
    let moves = from_state.legal_moves();
    let mut best: Option<(Vec<Move>, u32)> = None;
    for mv in moves {
        let result = solve(&from_state.apply_move(mv), seen.clone(), result_cache)
            .map(|(mvs, score)| (mvs, score + mv.score));
        match (&best, result) {
            (Some((_, sb)), Some((mut mr, sr))) => {
                if sr < *sb {
                    mr.insert(0, mv);
                    best = Some((mr, sr));
                }
            }
            (None, Some((mut mr, sr))) => {
                mr.insert(0, mv);
                best = Some((mr, sr));
            }
            _ => (),
        }
    }
    result_cache.insert(pos_hash, best.clone());
    best
}

fn initial_state() -> State {
    State {
        last_amphipod_moved: None,
        amphipods: vec![
            Amphipod::new(0, Copper, RoomA(0)),
            Amphipod::new(1, Amber, RoomA(1)),
            Amphipod::new(2, Desert, RoomB(0)),
            Amphipod::new(3, Desert, RoomB(1)),
            Amphipod::new(4, Bronze, RoomC(0)),
            Amphipod::new(5, Copper, RoomC(1)),
            Amphipod::new(6, Bronze, RoomD(0)),
            Amphipod::new(7, Amber, RoomD(1)),
        ],
    }
}

fn simple_test_state() -> State {
    State {
        last_amphipod_moved: None,
        amphipods: vec![
            Amphipod::new(0, Amber, RoomA(0)),
            Amphipod::new(1, Amber, RoomA(1)),
            Amphipod::new(2, Copper, RoomB(0)),
            Amphipod::new(3, Copper, RoomB(1)),
            Amphipod::new(4, Bronze, RoomC(0)),
            Amphipod::new(5, Bronze, RoomC(1)),
            Amphipod::new(6, Desert, RoomD(0)),
            Amphipod::new(7, Desert, RoomD(1)),
        ],
    }
}

fn test_state() -> State {
    State {
        last_amphipod_moved: None,
        amphipods: vec![
            Amphipod::new(0, Amber, RoomA(0)),
            Amphipod::new(1, Bronze, RoomA(1)),
            Amphipod::new(2, Desert, RoomB(0)),
            Amphipod::new(3, Copper, RoomB(1)),
            Amphipod::new(4, Copper, RoomC(0)),
            Amphipod::new(5, Bronze, RoomC(1)),
            Amphipod::new(6, Amber, RoomD(0)),
            Amphipod::new(7, Desert, RoomD(1)),
        ],
    }
}

fn main() {
    // let ts = test_state();
    // let (_best_moves, best_score) = solve(&ts, HashSet::new(), &mut HashMap::new()).unwrap();
    // println!("Test: {:?}", best_score);
    let state = simple_test_state(); //initial_state();
    let (_best_moves, best_score) = solve(&state, HashSet::new(), &mut HashMap::new()).unwrap();
    println!("Part 1: {:?}", best_score);
}
