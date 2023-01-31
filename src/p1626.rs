use crate::Solution;
use std::{borrow::Borrow, collections::BTreeSet};

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut players: Vec<Player> = scores
            .into_iter()
            .zip(ages)
            .map(|(score, age)| Player { score, age })
            .collect();
        players.sort_unstable();

        let mut team_set: BTreeSet<Team> = BTreeSet::new();

        for player in players {
            let last = team_set.range(..=Age(player.age)).last();
            match last {
                Some(last) => {
                    let new_team = Team {
                        max_age: Age(player.age),
                        sum_score: Score(last.sum_score.0 + player.score),
                    };
                    let mut set1 = team_set.split_off(&Age(player.age));
                    let mut set2 = set1.split_off(&Score(new_team.sum_score.0 + 1));
                    team_set.append(&mut set2);
                    team_set.insert(new_team);
                }
                None => {
                    let new_team = Team {
                        max_age: Age(player.age),
                        sum_score: Score(player.score),
                    };
                    team_set = team_set.split_off(&Score(new_team.sum_score.0 + 1));
                    team_set.insert(new_team);
                }
            }
        }

        team_set.iter().rev().next().unwrap().sum_score.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Player {
    pub score: i32,
    pub age: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Team {
    pub max_age: Age,
    pub sum_score: Score,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
struct Age(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
struct Score(i32);

impl Borrow<Age> for Team {
    fn borrow(&self) -> &Age {
        &self.max_age
    }
}

impl Borrow<Score> for Team {
    fn borrow(&self) -> &Score {
        &self.sum_score
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1626_test() {
        check(vec![1, 3, 5, 10, 15], vec![1, 2, 3, 4, 5], 34);
        check(vec![4, 5, 6, 5], vec![2, 1, 2, 1], 16);
        check(vec![1, 2, 3, 5], vec![8, 9, 10, 1], 6);
        check(vec![3, 1, 2], vec![1, 1, 1], 6);
        check(vec![10, 9, 8], vec![1, 2, 3], 10);
    }

    fn check(scores: Vec<i32>, ages: Vec<i32>, expect: i32) {
        assert_eq!(Solution::best_team_score(scores, ages), expect);
    }
}
