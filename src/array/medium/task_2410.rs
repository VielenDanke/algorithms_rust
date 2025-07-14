/*
You are given a 0-indexed integer array players, where players[i] represents the ability of the ith player.
You are also given a 0-indexed integer array trainers, where trainers[j] represents the training capacity of the jth trainer.

The ith player can match with the jth trainer if the player's ability is less than or equal to the trainer's training capacity.
 Additionally, the ith player can be matched with at most one trainer, and the jth trainer can be matched with at most one player.

Return the maximum number of matchings between players and trainers that satisfy these conditions.


 */
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        let mut res = 0;

        let mut players_bh = BinaryHeap::new();
        let mut trainers_bh = BinaryHeap::new();

        players.iter().for_each(|&p| players_bh.push(Reverse(p)));
        trainers.iter().for_each(|&t| trainers_bh.push(Reverse(t)));

        while let Some(p) = players_bh.pop() {
            let mut is_match = false;

            while let Some(t) = trainers_bh.pop() {
                if p.0 <= t.0 {
                    is_match = true;
                    break;
                }
            }
            if is_match {
                res += 1;
            }
            if trainers_bh.is_empty() {
                break;
            }
        }

        res
    }

    pub fn match_players_and_trainers_sort(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
        players.sort_unstable();
        trainers.sort_unstable();
        let mut players_idx = 0;
        let mut trainers_idx = 0;
        let mut res = 0;
        while players_idx < players.len() && trainers_idx < trainers.len() {
            if players[players_idx] <= trainers[trainers_idx] {
                res += 1;
                players_idx += 1;
                trainers_idx += 1;
            } else {
                trainers_idx += 1;
            }
        }
        res
    }
}
