use reqwest::blocking::Client as ReqwestClient;
use serde::Deserialize;
use std::error::Error;

// Checks for permissions in https://github.com/rust-lang/team/

#[derive(Debug, Deserialize, Clone)]
struct TeamResponse {
    discord_ids: Vec<usize>,
}

#[derive(Debug)]
struct TeamClient {
    client: ReqwestClient,
}

impl TeamClient {
    pub fn new() -> Self {
        let team_client = ReqwestClient::new();

        TeamClient {
            client: team_client,
        }
    }
}

fn get_team_info() -> Result<TeamResponse, Box<dyn Error>> {
    let team_client = TeamClient::new();

    let team_request = team_client.client.get(
        &String::from("https://team-api.infra.rust-lang.org/v1/permissions/crates_io_ops_bot.staging_crates_io.json")
    );

    let team_response = team_request.send().and_then(|res| res.error_for_status())?;

    let team_json: TeamResponse = team_response.json()?;

    Ok(team_json)
}

fn team_info() -> std::result::Result<TeamResponse, Box<dyn Error>> {
    get_team_info()
}

pub fn is_authorized(id: &str) -> Result<bool, Box<dyn Error>> {
    let authorization_info = team_info()?;

    let result = discord_id_in_list(id, authorization_info);
    Ok(result)
}

fn discord_id_in_list(id: &str, team_response: TeamResponse) -> bool {
    team_response
        .discord_ids
        .contains(&id.parse::<usize>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_team_info() -> TeamResponse {
        let fake_id: usize = 12345;
        let fake_id_vec = vec![fake_id];

        TeamResponse {
            discord_ids: fake_id_vec,
        }
    }

    #[test]
    fn check_whether_user_is_authorized() {
        let team_info = test_team_info();

        assert!(discord_id_in_list("12345", team_info.clone()));
        assert!(!discord_id_in_list("67890", team_info));
    }
}
