#![feature(test)]

use std::{cmp::Reverse, collections::HashSet};

extern crate test;

use itertools::Itertools;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use rayon::prelude::*;

use std::io::prelude::*;

const SSS: u8 = 100;

lazy_static! {
    static ref ALL_TEAMS: Vec<[u8; 4]> = {
        (0..SSS)
            .combinations(4)
            .map(|x| [x[0], x[1], x[2], x[3]])
            .collect_vec()
    };
}

// fn check() {
//     let t = include_str!("../message.txt");
//     let mut seen = HashSet::<[u8; 2]>::new();

//     for line in t.lines() {
//         let v = line
//             .split(' ')
//             .map(|x| x.parse::<u8>().unwrap())
//             .collect_vec();

//         if !add_seen(&[v[0], v[1], v[2], v[3]], &mut seen) {
//             panic!("seen {:?}", v);
//         }
//     }
// }

fn main() {
    let mut mmm = 0;

    let mut rng = rand::thread_rng();

    let pool = 100usize;
    let top_count = 25usize;

    println!("Seeding teams");

    // Create a gene pool
    let mut team_collection = (0..pool as u64)
        .collect_vec()
        .into_par_iter()
        .map(|i| {
            let mut rng = rand::rngs::StdRng::seed_from_u64(i);

            create_team_list(&mut rng)
        })
        .collect::<Vec<_>>();

    println!("Generated initial teams");

    for i in 0.. {
        // Select top 10

        team_collection.sort_by_key(|x| Reverse(x.len()));

        let mut top_teams = team_collection.into_iter().take(top_count).collect_vec();
        top_teams.shuffle(&mut rng);

        team_collection = top_teams
            .iter()
            .combinations(2)
            .take(pool - top_count)
            .collect_vec()
            .into_par_iter()
            .flat_map(|x| {
                let t1 = &x[0];
                let t2 = &x[1];
                let (nt1, nt2) = merge_teams(t1, t2);
                vec![fix_team(nt1), fix_team(nt2)]
            })
            .collect::<Vec<_>>();

        team_collection.append(&mut top_teams);

        let biggest = team_collection.iter().max_by_key(|x| x.len()).unwrap();

        println!(
            "Round: {}, round biggest: {}, overal biggest: {}",
            i,
            biggest.len(),
            mmm
        );

        if biggest.len() > mmm {
            mmm = biggest.len();
            print_team(i, biggest);
        }
    }

    // for i in 0.. {
    //     let team_list = create_team_list(&mut rng);
    //     if team_list.len() > mmm {
    //         mmm = team_list.len();
    //         index = i;

    //         let mut f = std::fs::File::create(format!("result-{}", i)).unwrap();
    //         f.write_all(&format!("{:?}", team_list).bytes().collect_vec()[..])
    //             .expect("could not write");
    //         println!(
    //             "{}: Length {}, max: {} at {}",
    //             i,
    //             team_list.len(),
    //             mmm,
    //             index
    //         );
    //     }
    // }
}

fn print_team(i: usize, t: &Vec<[u8; 4]>) {
    let mut f = std::fs::File::create(format!("result-{}", i)).unwrap();
    f.write_all(&format!("{:?}", t).bytes().collect_vec()[..])
        .expect("could not write");
    println!("{}: Length {}", i, t.len());
}

fn create_team_list<R: Rng>(rand: &mut R) -> Vec<[u8; 4]> {
    let mut teams = ALL_TEAMS.clone();

    //println!("Generated teams!");

    teams.shuffle(rand);

    //println!("Shuffled!");

    let mut seen = HashSet::<[u8; 2]>::new();
    let mut team_list = Vec::with_capacity(720);

    for team in teams {
        if !contains(&team, &seen) {
            add_seen(&team, &mut seen);
            team_list.push(team);
        }
    }
    team_list
}

fn merge_teams(team1: &[[u8; 4]], team2: &[[u8; 4]]) -> (Vec<[u8; 4]>, Vec<[u8; 4]>) {
    let h1 = split_team(team1);
    let h2 = split_team(team2);

    let v1 =
        h1.0.into_iter()
            .chain(h2.1.into_iter())
            .cloned()
            .collect_vec();
    let v2 =
        h2.0.into_iter()
            .chain(h1.1.into_iter())
            .cloned()
            .collect_vec();

    (v1, v2)
}

fn split_team(team: &[[u8; 4]]) -> (&[[u8; 4]], &[[u8; 4]]) {
    let mid = team.len() / 2;
    (&team[..mid], &team[mid..])
}

fn contains(team: &[u8; 4], set: &HashSet<[u8; 2]>) -> bool {
    team.into_iter()
        .combinations(2)
        .any(|f| set.contains(&[*f[0], *f[1]]))
}

fn add_seen(team: &[u8; 4], set: &mut HashSet<[u8; 2]>) -> bool {
    team.into_iter()
        .combinations(2)
        .all(|slce| set.insert([*slce[0], *slce[1]]))
}

fn fix_team(team: Vec<[u8; 4]>) -> Vec<[u8; 4]> {
    let mut seen = HashSet::new();
    let mut doubles = HashSet::new();
    for t in &team {
        if !add_seen(t, &mut seen) {
            add_seen(t, &mut doubles);
        }
    }

    let mut teams = team
        .into_iter()
        .filter(|x| !contains(x, &doubles))
        .collect_vec();

    let mut seen = HashSet::new();

    for t in &teams {
        if !add_seen(t, &mut seen) {
            unreachable!("doubles should have been removed");
        }
    }

    let mut iter_teams = ALL_TEAMS.clone();
    iter_teams.shuffle(&mut rand::thread_rng());

    for team in iter_teams {
        if !contains(&team, &seen) {
            add_seen(&team, &mut seen);
            teams.push(team);
        }
    }
    teams
}
