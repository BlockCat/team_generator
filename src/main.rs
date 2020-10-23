#![feature(test)]

use std::collections::{HashMap, HashSet};

extern crate test;

use itertools::Itertools;
use rand::seq::SliceRandom;
use std::iter::FromIterator;

use std::io::prelude::*;

const SSS: u8 = 100;

fn main() {
    let mut mmm = 0;
    let mut index = 0;
    for i in 0.. {
        let mut teams = (0..SSS)
            .combinations(4)
            .into_iter()
            .map(|x| [x[0], x[1], x[2], x[3]])
            .collect_vec();

        println!("Generated teams!");

        teams.shuffle(&mut rand::thread_rng());

        println!("Shuffled!");

        let mut seen = HashSet::<[u8; 2]>::new();
        let mut team_list = Vec::new();

        for team in teams {
            if !contains(team, &seen) {
                add_seen(team, &mut seen);
                team_list.push(team);
            }
        }

        let mut f = std::fs::File::create(format!("result-{}", i)).unwrap();
        f.write_all(&format!("{:?}", team_list).bytes().collect_vec()[..]);

        if team_list.len() > mmm {
            mmm = team_list.len();
            index = i;
        }
        println!("{}: Length {}, max: {} at {}", i, team_list.len(), mmm, index);
    }
}

fn contains(team: [u8; 4], set: &HashSet<[u8; 2]>) -> bool {
    team.into_iter()
        .combinations(2)
        .any(|f| set.contains(&[*f[0], *f[1]]))
}

fn add_seen(team: [u8; 4], set: &mut HashSet<[u8; 2]>) {
    for slce in team.into_iter().combinations(2) {
        set.insert([*slce[0], *slce[1]]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| main());
    }
}
