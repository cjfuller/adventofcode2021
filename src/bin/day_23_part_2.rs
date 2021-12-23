use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display, Formatter};

const ROOM_SIZE: u8 = 4;
const ROOM_SIZE_M1: u8 = ROOM_SIZE - 1;

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
    fn deepen(&self) -> Position {
        if self.index() == 0 {
            return *self;
        }
        match self {
            Hallway(_) => *self,
            RoomA(d) => RoomA(d - 1),
            RoomB(d) => RoomB(d - 1),
            RoomC(d) => RoomC(d - 1),
            RoomD(d) => RoomD(d - 1),
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

impl AmphipodType {
    fn id(&self) -> u8 {
        match self {
            Amber => 0,
            Bronze => 1,
            Copper => 2,
            Desert => 3,
        }
    }
}

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
    fn is_home_at(&self, depth: u8) -> bool {
        self.pos == self.home(depth)
    }
    fn home(&self, depth: u8) -> Position {
        match self.t {
            Amber => RoomA(depth),
            Bronze => RoomB(depth),
            Copper => RoomC(depth),
            Desert => RoomD(depth),
        }
    }
    fn home_unoccupied_or_match(&self, pods: &[Amphipod]) -> bool {
        (0..ROOM_SIZE).all(|depth| {
            self.home(depth)
                .occupant(pods)
                .map(|it| it.is_home())
                .unwrap_or(true)
        })
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

fn pos_to_num(pod: &Amphipod) -> u128 {
    let pos = pod.pos;
    (match pos {
        Hallway(num) => num,
        RoomA(num) => 11 + (Amber.id() * ROOM_SIZE) + num,
        RoomB(num) => 11 + (Bronze.id() * ROOM_SIZE) + num,
        RoomC(num) => 11 + (Copper.id() * ROOM_SIZE) + num,
        RoomD(num) => 11 + (Desert.id() * ROOM_SIZE) + num,
    }) as u128
        + (if pod.stopped_when_stuck_in_hallway {
            32
        } else {
            0
        })
        + (if pod.is_stuck_in_hallway { 64 } else { 0 })
}

impl State {
    fn pos_hash(&self) -> u128 {
        let mut a_s: Vec<u128> = self
            .amphipods
            .iter()
            .filter(|it| matches!(it.t, Amber))
            .map(pos_to_num)
            .collect();
        a_s.sort_unstable();
        let mut b_s: Vec<u128> = self
            .amphipods
            .iter()
            .filter(|it| matches!(it.t, Bronze))
            .map(pos_to_num)
            .collect();
        b_s.sort_unstable();
        let mut c_s: Vec<u128> = self
            .amphipods
            .iter()
            .filter(|it| matches!(it.t, Copper))
            .map(pos_to_num)
            .collect();
        c_s.sort_unstable();
        let mut d_s: Vec<u128> = self
            .amphipods
            .iter()
            .filter(|it| matches!(it.t, Desert))
            .map(pos_to_num)
            .collect();
        d_s.sort_unstable();
        let mut h = 0u128;
        for (i, a) in a_s.iter().enumerate() {
            h += a << (7 * (i + (Amber.id() * ROOM_SIZE) as usize));
        }
        for (i, b) in b_s.iter().enumerate() {
            h += b << (7 * (i + (Bronze.id() * ROOM_SIZE) as usize));
        }
        for (i, c) in c_s.iter().enumerate() {
            h += c << (7 * (i + (Copper.id() * ROOM_SIZE) as usize));
        }
        for (i, d) in d_s.iter().enumerate() {
            h += d << (7 * (i + (Desert.id() * ROOM_SIZE) as usize));
        }
        h += self
            .last_amphipod_moved
            .map(|it| pos_to_num(&it))
            .unwrap_or(127)
            << (ROOM_SIZE * 4 * 7);
        h
    }

    fn is_complete(&self) -> bool {
        self.amphipods.iter().all(|it| it.is_home())
    }

    fn legal_moves_for_amphipod(&self, amphipod: &Amphipod) -> Vec<Move> {
        if amphipod.is_home_at(0) {
            return vec![];
        }

        if amphipod.is_home()
            && (0..amphipod.pos.index()).all(|depth| {
                amphipod
                    .home(depth)
                    .occupant(&self.amphipods)
                    .map(|it| it.is_home())
                    .unwrap_or_default()
            })
        {
            return vec![];
        }

        let possible_targets = match amphipod.pos {
            RoomA(ROOM_SIZE_M1) => vec![RoomA(ROOM_SIZE - 2), Hallway(2)],
            RoomA(0) => vec![RoomA(1)],
            RoomA(d) => vec![RoomA(d - 1), RoomA(d + 1)],
            RoomB(ROOM_SIZE_M1) => vec![RoomB(ROOM_SIZE - 2), Hallway(4)],
            RoomB(0) => vec![RoomB(1)],
            RoomB(d) => vec![RoomB(d - 1), RoomB(d + 1)],
            RoomC(ROOM_SIZE_M1) => vec![RoomC(ROOM_SIZE - 2), Hallway(6)],
            RoomC(0) => vec![RoomC(1)],
            RoomC(d) => vec![RoomC(d - 1), RoomC(d + 1)],
            RoomD(ROOM_SIZE_M1) => vec![RoomD(ROOM_SIZE - 2), Hallway(8)],
            RoomD(0) => vec![RoomD(1)],
            RoomD(d) => vec![RoomD(d - 1), RoomD(d + 1)],
            Hallway(0) => vec![Hallway(1)],
            Hallway(10) => vec![Hallway(9)],
            Hallway(2) => vec![Hallway(1), Hallway(3), RoomA(ROOM_SIZE_M1)],
            Hallway(4) => vec![Hallway(3), Hallway(5), RoomB(ROOM_SIZE_M1)],
            Hallway(6) => vec![Hallway(5), Hallway(7), RoomC(ROOM_SIZE_M1)],
            Hallway(8) => vec![Hallway(7), Hallway(9), RoomD(ROOM_SIZE_M1)],
            Hallway(n) => vec![Hallway(n - 1), Hallway(n + 1)],
        };
        possible_targets
            .iter()
            .filter(|it| it.is_in_hallway() || it.index() < ROOM_SIZE)
            .filter(|it| !it.occupied(&self.amphipods))
            .filter(|it| {
                it.is_in_hallway()
                    || (amphipod.is_room_match(it)
                        && amphipod.home_unoccupied_or_match(&self.amphipods))
                    || amphipod.pos.same_room(it)
            })
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
            if amphipod.is_home()
                && amphipod.home_unoccupied_or_match(&self.amphipods)
                && !amphipod.pos.deepen().occupied(&self.amphipods)
            {
                return vec![Move {
                    source: *amphipod,
                    dest: Amphipod {
                        pos: amphipod.pos.deepen(),
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
    mut seen: HashSet<u128>,
    result_cache: &mut HashMap<u128, Option<u32>>,
) -> Option<u32> {
    let pos_hash = from_state.pos_hash();
    if from_state.is_complete() {
        result_cache.insert(pos_hash, Some(0));
        return Some(0);
    }
    if result_cache.contains_key(&pos_hash) {
        return result_cache.get(&pos_hash).and_then(|it| *it);
    }
    if seen.contains(&pos_hash) {
        return None;
    }
    seen.insert(pos_hash);
    let moves = from_state.legal_moves();
    let best: Option<u32> = if moves.is_empty() {
        None
    } else {
        let mut scores: Vec<Option<u32>> = vec![];
        for mv in moves {
            scores.push(
                solve(&from_state.apply_move(mv), seen.clone(), result_cache)
                    .map(|it| it + mv.score),
            );
        }
        scores
            .into_iter()
            .reduce(
                |best: Option<u32>, result: Option<u32>| match (best, result) {
                    (Some(b), Some(r)) => Some(std::cmp::min(b, r)),
                    (None, Some(r)) => Some(r),
                    _ => best,
                },
            )
            .unwrap()
    };
    result_cache.insert(pos_hash, best);
    best
}

fn initial_state() -> State {
    State {
        last_amphipod_moved: None,
        amphipods: vec![
            Amphipod::new(0, Copper, RoomA(0)),
            Amphipod::new(1, Amber, RoomA(3)),
            Amphipod::new(2, Desert, RoomB(0)),
            Amphipod::new(3, Desert, RoomB(3)),
            Amphipod::new(4, Bronze, RoomC(0)),
            Amphipod::new(5, Copper, RoomC(3)),
            Amphipod::new(6, Bronze, RoomD(0)),
            Amphipod::new(7, Amber, RoomD(3)),
            Amphipod::new(8, Desert, RoomA(1)),
            Amphipod::new(9, Desert, RoomA(2)),
            Amphipod::new(10, Bronze, RoomB(1)),
            Amphipod::new(11, Copper, RoomB(2)),
            Amphipod::new(12, Amber, RoomC(1)),
            Amphipod::new(13, Bronze, RoomC(2)),
            Amphipod::new(14, Copper, RoomD(1)),
            Amphipod::new(15, Amber, RoomD(2)),
        ],
    }
}

fn test_state() -> State {
    State {
        last_amphipod_moved: None,
        amphipods: vec![
            Amphipod::new(0, Amber, RoomA(0)),
            Amphipod::new(1, Bronze, RoomA(3)),
            Amphipod::new(2, Desert, RoomB(0)),
            Amphipod::new(3, Copper, RoomB(3)),
            Amphipod::new(4, Copper, RoomC(0)),
            Amphipod::new(5, Bronze, RoomC(3)),
            Amphipod::new(6, Amber, RoomD(0)),
            Amphipod::new(7, Desert, RoomD(3)),
            Amphipod::new(8, Desert, RoomA(1)),
            Amphipod::new(9, Desert, RoomA(2)),
            Amphipod::new(10, Bronze, RoomB(1)),
            Amphipod::new(11, Copper, RoomB(2)),
            Amphipod::new(12, Amber, RoomC(1)),
            Amphipod::new(13, Bronze, RoomC(2)),
            Amphipod::new(14, Copper, RoomD(1)),
            Amphipod::new(15, Amber, RoomD(2)),
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

fn old_test_state() -> State {
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

fn part_1() -> State {
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

fn main() {
    let ts = initial_state();
    let best_score = solve(&ts, HashSet::new(), &mut HashMap::new()).unwrap();
    println!("Part 2: {:?}", best_score);
}
