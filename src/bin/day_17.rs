use std::cmp::Ordering;
use std::ops::RangeInclusive;

const MIN_X: i32 = 211;
const MAX_X: i32 = 232;
const MIN_Y: i32 = -124;
const MAX_Y: i32 = -69;
const TARGET_X: RangeInclusive<i32> = MIN_X..=MAX_X;
const TARGET_Y: RangeInclusive<i32> = MIN_Y..=MAX_Y;

struct Probe {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,
    max_y: i32,
}

impl Probe {
    fn fire(vel_x: i32, vel_y: i32) -> Probe {
        Probe {
            pos_x: 0,
            pos_y: 0,
            max_y: 0,
            vel_x,
            vel_y,
        }
    }

    fn step(&mut self) {
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;
        self.vel_x = match self.vel_x.cmp(&0) {
            Ordering::Greater => self.vel_x - 1,
            Ordering::Less => self.vel_x + 1,
            Ordering::Equal => 0,
        };
        self.vel_y -= 1;
        self.max_y = std::cmp::max(self.max_y, self.pos_y);
    }

    fn hit_target(&self) -> bool {
        TARGET_X.contains(&self.pos_x) && TARGET_Y.contains(&self.pos_y)
    }
    fn overshot_target(&self) -> bool {
        self.pos_x > MAX_X || self.pos_y < MIN_Y
    }
    fn sim_until_done(&mut self) {
        while !self.overshot_target() && !self.hit_target() {
            self.step()
        }
    }
}

fn main() {
    let mut max_height: i32 = 0;
    let mut hit_count: i32 = 0;
    for xvel in 1..250 {
        for yvel in -130..300 {
            let mut probe = Probe::fire(xvel, yvel);
            probe.sim_until_done();
            if probe.hit_target() {
                hit_count += 1;
                // println!("Hit with (xv, yv) = ({}, {})", xvel, yvel);
                max_height = std::cmp::max(max_height, probe.max_y);
            }
        }
    }
    println!("Part 1: {}", max_height);
    println!("Part 2: {}", hit_count);
}
