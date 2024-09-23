// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// 给定一场足球比赛的得分列表（每行一个）。每行的格式为：
// "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// 例如：England,France,4,2（英格兰进了4个球，法国进了2个球）。
//
// 你需要构建一个得分表，包含球队名称、球队进球数和球队失球数。
// 构建得分表的一种方法是使用哈希映射。解决方案部分使用了哈希映射，
// 请完成它以通过测试。
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::collections::HashMap;

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.
        // TODO: 使用从当前行提取的详细信息填充得分表。请记住，team_1进的球
        // 将是team_2失的球数，类似地，team_2进的球将是team_1失的球数。
                // Update team 1's scores
                let team_1 = scores.entry(team_1_name.clone()).or_insert(Team {
                    goals_scored: 0,
                    goals_conceded: 0,
                });
                team_1.goals_scored += team_1_score;
                team_1.goals_conceded += team_2_score;
        
                // Update team 2's scores
                let team_2 = scores.entry(team_2_name.clone()).or_insert(Team {
                    goals_scored: 0,
                    goals_conceded: 0,
                });
                team_2.goals_scored += team_2_score;
                team_2.goals_conceded += team_1_score;
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
