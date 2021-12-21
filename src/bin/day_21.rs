use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct GameState {
    player_0_pos: u32,
    player_1_pos: u32,
    player_0_score: u32,
    player_1_score: u32,
    num_rolls: u32,
    die_state: u32,
    next_player: u8,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
struct WinInfo {
    player_0_wins: u64,
    player_1_wins: u64,
}

impl std::ops::Add<WinInfo> for WinInfo {
    type Output = WinInfo;
    fn add(self: WinInfo, other: WinInfo) -> WinInfo {
        WinInfo {
            player_0_wins: self.player_0_wins + other.player_0_wins,
            player_1_wins: self.player_1_wins + other.player_1_wins,
        }
    }
}

impl std::ops::AddAssign for WinInfo {
    fn add_assign(&mut self, rhs: WinInfo) {
        self.player_0_wins += rhs.player_0_wins;
        self.player_1_wins += rhs.player_1_wins;
    }
}

impl GameState {
    fn start() -> GameState {
        GameState {
            player_0_pos: 8,
            player_1_pos: 9,
            player_0_score: 0,
            player_1_score: 0,
            num_rolls: 0,
            die_state: 0,
            next_player: 0,
        }
    }

    fn sim_to_win(&mut self) -> GameState {
        let roll_total =
            (self.die_state + 1) % 100 + (self.die_state + 2) % 100 + (self.die_state + 3) % 100;
        self.die_state += 3;
        self.die_state %= 100;
        self.num_rolls += 3;
        if self.next_player == 0 {
            self.player_0_pos += roll_total;
            self.player_0_pos %= 10;
            self.player_0_score += self.player_0_pos + 1;
        } else {
            self.player_1_pos += roll_total;
            self.player_1_pos %= 10;
            self.player_1_score += self.player_1_pos + 1;
        }
        self.next_player += 1;
        self.next_player %= 2;

        if self.player_0_score >= 1000 || self.player_1_score >= 1000 {
            *self
        } else {
            self.sim_to_win()
        }
    }

    fn dirac_sim(&self, lookup: Rc<RefCell<HashMap<GameState, WinInfo>>>, info: &mut WinInfo) {
        for roll_total in 1..=3 {
            let num_rolls = self.num_rolls + 1;
            let mut player_0_pos = self.player_0_pos;
            let mut player_1_pos = self.player_1_pos;
            if self.next_player == 0 {
                player_0_pos += roll_total;
                player_0_pos %= 10;
            } else {
                player_1_pos += roll_total;
                player_1_pos %= 10;
            }
            let mut player_0_score = self.player_0_score;
            let mut player_1_score = self.player_1_score;
            let next_player = if num_rolls % 3 == 0 && num_rolls % 2 != 0 {
                assert!(self.next_player == 0);
                player_0_score += player_0_pos + 1;
                1
            } else if num_rolls % 3 == 0 {
                assert!(self.next_player == 1);
                player_1_score += player_1_pos + 1;
                0
            } else {
                self.next_player
            };

            let next_state = GameState {
                num_rolls,
                player_0_score,
                player_1_score,
                player_0_pos,
                player_1_pos,
                next_player,
                die_state: 0,
            };

            let this_win = if player_0_score >= 21 {
                Some(WinInfo {
                    player_0_wins: 1,
                    player_1_wins: 0,
                })
            } else if player_1_score >= 21 {
                Some(WinInfo {
                    player_0_wins: 0,
                    player_1_wins: 1,
                })
            } else {
                None
            };

            if let Some(this_info) = this_win {
                lookup.borrow_mut().insert(next_state, this_info);
                *info += this_info;
                continue;
            }

            let has_next_already = { lookup.borrow().contains_key(&next_state) };
            if has_next_already {
                let next_info = *lookup.borrow().get(&next_state).unwrap();
                *info += next_info;
            } else {
                let mut subtree_info = WinInfo::default();
                next_state.dirac_sim(lookup.clone(), &mut subtree_info);
                *info += subtree_info;
                let mut borrowed = lookup.borrow_mut();
                let mut next_info = borrowed.get(&next_state).cloned().unwrap_or_default();
                next_info += subtree_info;
                borrowed.insert(next_state, next_info);
            }
        }
    }
}

fn main() {
    let win = GameState::start().sim_to_win();
    let answer = win.num_rolls
        * if win.player_0_score >= 1000 {
            win.player_1_score
        } else {
            win.player_0_score
        };
    println!("Part 1: {}", answer);

    let mut base_info = WinInfo::default();
    let lookup = HashMap::new();
    GameState::start().dirac_sim(Rc::new(RefCell::new(lookup)), &mut base_info);
    println!(
        "Part 2: {}",
        std::cmp::max(base_info.player_0_wins, base_info.player_1_wins)
    )
}
