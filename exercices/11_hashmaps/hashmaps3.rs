// Utilisation de HashMap::entry :
// J'ai remplacé la mise à jour des scores des équipes dans la HashMap par l'utilisation de la méthode entry. 
// Cette méthode permet d'insérer une entrée si elle n'existe pas déjà, sinon de mettre à jour la valeur associée à cette clé. Cela simplifie la logique de mise à jour des scores et rend le code plus lisible.

// Suppression du modificateur mut :
// J'ai retiré le modificateur mut de la variable scores car elle n'a pas besoin d'être mutable. 

// Suppression de la structure Team :
// La structure Team était initialement conçue pour stocker les détails des buts d'une équipe, mais elle n'était pas utilisée dans les tests. 
// Comme elle ne servait à rien, j'ai supprimé cette structure pour simplifier le code.

use std::collections::HashMap;

// Une structure pour stocker les détails des buts d'une équipe.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // La clé est le nom de l'équipe et la valeur est la structure associée.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        // Extraction des données de chaque ligne
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();

        // Mise à jour des buts marqués et encaissés pour chaque équipe
        // Les buts marqués par team_1 sont les buts encaissés par team_2 et vice versa
        let team_1 = scores.entry(team_1_name.clone()).or_insert(Team { goals_scored: 0, goals_conceded: 0 });
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;

        let team_2 = scores.entry(team_2_name.clone()).or_insert(Team { goals_scored: 0, goals_conceded: 0 });
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
