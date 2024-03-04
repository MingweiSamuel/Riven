mod testutils;
use riven::consts::*;
use testutils::*;

const ROUTE: PlatformRoute = PlatformRoute::EUW1;

// Champion Mastery tests

#[riven_test]
async fn championmastery_getscore_ma5tery() -> Result<(), String> {
    let sum = riot_api()
        .summoner_v4()
        .get_by_summoner_name(ROUTE, "ma5tery");
    let sum = sum
        .await
        .map_err(|e| format!("Error getting summoner: {}", e))?
        .ok_or_else(|| "Failed to find summoner".to_owned())?;

    let p = riot_api()
        .champion_mastery_v4()
        .get_champion_mastery_score_by_puuid(ROUTE, &sum.puuid);
    let s = p
        .await
        .map_err(|e| format!("Error getting champion mastery score: {}", e))?;
    rassert!(
        (969..=1000).contains(&s),
        "Unexpected ma5tery score: {}.",
        s
    );
    Ok(())
}

#[riven_test]
async fn championmastery_getall_ma5tery() -> Result<(), String> {
    let sum = riot_api()
        .summoner_v4()
        .get_by_summoner_name(ROUTE, "ma5tery");
    let sum = sum
        .await
        .map_err(|e| format!("Error getting summoner: {}", e))?
        .ok_or_else(|| "Failed to find summoner".to_owned())?;

    let p = riot_api()
        .champion_mastery_v4()
        .get_all_champion_masteries_by_puuid(ROUTE, &sum.puuid);
    let s = p
        .await
        .map_err(|e| format!("Error getting all champion masteries: {}", e))?;
    rassert!(s.len() >= 142, "Expected masteries: {}.", s.len());
    Ok(())
}

/// https://github.com/RiotGames/developer-relations/issues/602
#[riven_test]
async fn spectator_v4_combo() -> Result<(), String> {
    let featured_p = riot_api().spectator_v4().get_featured_games(ROUTE);
    let featured = featured_p.await.map_err(|e| e.to_string())?;

    if featured.game_list.is_empty() {
        eprintln!("Featured game list is empty!");
        return Ok(());
    }

    let featured_game = &featured.game_list[0];
    let participant = &featured_game.participants[0];
    let summoner_id = participant.summoner_id.as_ref().ok_or_else(|| {
        format!(
            "Summoner in spectator featured game missing summoner ID: {}",
            &participant.summoner_name
        )
    })?;

    let livegame_p = riot_api()
        .spectator_v4()
        .get_current_game_info_by_summoner(ROUTE, &summoner_id);
    let livegame_o = livegame_p.await.map_err(|e| e.to_string())?;
    if let Some(livegame) = livegame_o {
        let participant_match = livegame
            .participants
            .iter()
            .find(|p| p.summoner_name == participant.summoner_name);
        rassert!(
            participant_match.is_some(),
            "Failed to find summoner in match: {}.",
            &participant.summoner_name
        );
    }
    Ok(())
}

#[riven_test]
async fn spectator_v5_combo() -> Result<(), String> {
    let featured_p = riot_api().spectator_v5().get_featured_games(ROUTE);
    let featured = featured_p.await.map_err(|e| e.to_string())?;

    if featured.game_list.is_empty() {
        eprintln!("Featured game list is empty!");
        return Ok(());
    }

    let featured_game = &featured.game_list[0];
    let participant = &featured_game.participants[0];
    let puuid = participant.puuid.as_ref().ok_or_else(|| {
        format!(
            "Summoner in spectator featured game missing summoner ID: {}",
            &participant.summoner_name
        )
    })?;

    let livegame_p = riot_api()
        .spectator_v5()
        .get_current_game_info_by_puuid(ROUTE, &puuid);
    let livegame_o = livegame_p.await.map_err(|e| e.to_string())?;
    if let Some(livegame) = livegame_o {
        let participant_match = livegame
            .participants
            .iter()
            .find(|p| p.summoner_name == participant.summoner_name);
        rassert!(
            participant_match.is_some(),
            "Failed to find summoner in match: {}.",
            &participant.summoner_name
        );
    }
    Ok(())
}
