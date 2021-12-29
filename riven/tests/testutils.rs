#![allow(dead_code)]

use lazy_static::lazy_static;

use riven::consts::RegionalRoute;
use riven::{RiotApi, RiotApiConfig};

lazy_static! {
    pub static ref RIOT_API: RiotApi = {
        let api_key = std::env::var("RGAPI_KEY")
            .ok()
            .or_else(|| std::fs::read_to_string("apikey.txt").ok())
            .expect("Failed to find RGAPI_KEY env var or apikey.txt.");
        RiotApi::new(RiotApiConfig::with_key(api_key.trim()).preconfig_burst())
    };
}

pub mod ids {
    pub const SUMMONER_ID_LUGNUTSK: &str = "SBM8Ubipo4ge2yj7bhEzL7yvV0C9Oc1XA2l6v5okGMA_nCw";
    pub const SUMMONER_ID_MA5TERY: &str = "IbC4uyFEEW3ZkZw6FZF4bViw3P1EynclAcI6-p-vCpI99Ec";
    pub const SUMMONER_ID_C9SNEAKY: &str = "ghHSdADqgxKwcRl_vWndx6wKiyZx0xKQv-LOhOcU5LU";
    pub const ACCOUNT_ID_C9SNEAKY: &str = "ML_CcLT94UUHp1iDvXOXCidfmzzPrk_Jbub1f_INhw";
    pub const ACCOUNT_ID_LUGNUTSK: &str = "iheZF2uJ50S84Hfq6Ob8GNlJAUmBmac-EsEEWBJjD01q1jQ";
}

pub async fn match_v5_get(route: RegionalRoute, matches: &[&'static str]) -> Result<(), String> {
    for &matche in matches {
        let p = RIOT_API.match_v5().get_match(route, matche);
        let m = p
            .await
            .map_err(|e| format!("Failed to get match {}: {:?}", matche, e))?
            .ok_or(format!("Match {} not found.", matche))?;

        if matche != &*m.metadata.match_id {
            return Err(format!(
                "Bad match id? Sent {}, received {}.",
                matche, m.metadata.match_id
            ));
        }
        if m.metadata.participants.is_empty() {
            return Err("Match should have participants.".to_owned());
        }
        if m.metadata.participants.len() != m.info.participants.len() {
            return Err("Match participants do not line up with participant UUIDs.".to_owned());
        }
        for participant in &m.info.participants {
            participant.champion().map_err(|e| format!("Failed to determine champion: {}", e))?;
        }
        if m.info.teams.is_empty() {
            return Err("Match should have teams.".to_owned());
        }
    }
    Ok(())
}

pub async fn match_v5_get_timeline(
    route: RegionalRoute,
    matches: &[&'static str],
) -> Result<(), String> {
    for &matche in matches {
        let p = RIOT_API.match_v5().get_timeline(route, matche);
        let m = p
            .await
            .map_err(|e| format!("Failed to get match {}: {:?}", matche, e))?
            .ok_or(format!("Match {} not found.", matche))?;
        if matche != &*m.metadata.match_id {
            return Err(format!(
                "Bad match id? Sent {}, received {}.",
                matche, m.metadata.match_id
            ));
        }
        if m.metadata.participants.is_empty() {
            return Err("Match should have participants.".to_owned());
        }
        if let Some(game_id) = m.info.game_id {
            if matche[(matche.find('_').unwrap() + 1)..] != game_id.to_string() {
                return Err("Match number ID should match.".to_owned());
            }
        }
        if m.info.frames.is_empty() {
            return Err("Match timleine should have frames.".to_owned());
        }
    }
    Ok(())
}
