// http://www.mingweisamuel.com/riotapi-schema/tool/
// Version: 996d171a2b79e9bb85c549f47b07c6ef2721fc8a

use reqwest::Method;
/// Metadata for endpoints. Each tuple corresponds to one endpoint and contains
/// the HTTP [`Method`], `str` path, and the method's `str` ID.
pub static ALL_ENDPOINTS: [(Method, &str, &str); 87usize] = [
    (Method::GET, "/riot/account/v1/accounts/by-puuid/{puuid}", "account-v1.getByPuuid"),
    (
        Method::GET,
        "/riot/account/v1/accounts/by-riot-id/{gameName}/{tagLine}",
        "account-v1.getByRiotId",
    ),
    (Method::GET, "/riot/account/v1/accounts/me", "account-v1.getByAccessToken"),
    (
        Method::GET,
        "/riot/account/v1/active-shards/by-game/{game}/by-puuid/{puuid}",
        "account-v1.getActiveShard",
    ),
    (
        Method::GET,
        "/lol/champion-mastery/v4/champion-masteries/by-puuid/{encryptedPUUID}",
        "champion-mastery-v4.getAllChampionMasteriesByPUUID",
    ),
    (
        Method::GET,
        "/lol/champion-mastery/v4/champion-masteries/by-puuid/{encryptedPUUID}/by-champion/{championId}",
        "champion-mastery-v4.getChampionMasteryByPUUID",
    ),
    (
        Method::GET,
        "/lol/champion-mastery/v4/champion-masteries/by-puuid/{encryptedPUUID}/top",
        "champion-mastery-v4.getTopChampionMasteriesByPUUID",
    ),
    (
        Method::GET,
        "/lol/champion-mastery/v4/scores/by-puuid/{encryptedPUUID}",
        "champion-mastery-v4.getChampionMasteryScoreByPUUID",
    ),
    (Method::GET, "/lol/platform/v3/champion-rotations", "champion-v3.getChampionInfo"),
    (
        Method::GET,
        "/lol/clash/v1/players/by-puuid/{puuid}",
        "clash-v1.getPlayersByPUUID",
    ),
    (Method::GET, "/lol/clash/v1/teams/{teamId}", "clash-v1.getTeamById"),
    (Method::GET, "/lol/clash/v1/tournaments", "clash-v1.getTournaments"),
    (
        Method::GET,
        "/lol/clash/v1/tournaments/by-team/{teamId}",
        "clash-v1.getTournamentByTeam",
    ),
    (
        Method::GET,
        "/lol/clash/v1/tournaments/{tournamentId}",
        "clash-v1.getTournamentById",
    ),
    (
        Method::GET,
        "/lol/league-exp/v4/entries/{queue}/{tier}/{division}",
        "league-exp-v4.getLeagueEntries",
    ),
    (
        Method::GET,
        "/lol/league/v4/challengerleagues/by-queue/{queue}",
        "league-v4.getChallengerLeague",
    ),
    (
        Method::GET,
        "/lol/league/v4/entries/by-puuid/{encryptedPUUID}",
        "league-v4.getLeagueEntriesByPUUID",
    ),
    (
        Method::GET,
        "/lol/league/v4/entries/by-summoner/{encryptedSummonerId}",
        "league-v4.getLeagueEntriesForSummoner",
    ),
    (
        Method::GET,
        "/lol/league/v4/entries/{queue}/{tier}/{division}",
        "league-v4.getLeagueEntries",
    ),
    (
        Method::GET,
        "/lol/league/v4/grandmasterleagues/by-queue/{queue}",
        "league-v4.getGrandmasterLeague",
    ),
    (Method::GET, "/lol/league/v4/leagues/{leagueId}", "league-v4.getLeagueById"),
    (
        Method::GET,
        "/lol/league/v4/masterleagues/by-queue/{queue}",
        "league-v4.getMasterLeague",
    ),
    (
        Method::GET,
        "/lol/challenges/v1/challenges/config",
        "lol-challenges-v1.getAllChallengeConfigs",
    ),
    (
        Method::GET,
        "/lol/challenges/v1/challenges/percentiles",
        "lol-challenges-v1.getAllChallengePercentiles",
    ),
    (
        Method::GET,
        "/lol/challenges/v1/challenges/{challengeId}/config",
        "lol-challenges-v1.getChallengeConfigs",
    ),
    (
        Method::GET,
        "/lol/challenges/v1/challenges/{challengeId}/leaderboards/by-level/{level}",
        "lol-challenges-v1.getChallengeLeaderboards",
    ),
    (
        Method::GET,
        "/lol/challenges/v1/challenges/{challengeId}/percentiles",
        "lol-challenges-v1.getChallengePercentiles",
    ),
    (
        Method::GET,
        "/lol/challenges/v1/player-data/{puuid}",
        "lol-challenges-v1.getPlayerData",
    ),
    (Method::GET, "/lol/rso-match/v1/matches/ids", "lol-rso-match-v1.getMatchIds"),
    (Method::GET, "/lol/rso-match/v1/matches/{matchId}", "lol-rso-match-v1.getMatch"),
    (
        Method::GET,
        "/lol/rso-match/v1/matches/{matchId}/timeline",
        "lol-rso-match-v1.getTimeline",
    ),
    (Method::GET, "/lol/status/v4/platform-data", "lol-status-v4.getPlatformData"),
    (Method::GET, "/lor/deck/v1/decks/me", "lor-deck-v1.getDecks"),
    (Method::POST, "/lor/deck/v1/decks/me", "lor-deck-v1.createDeck"),
    (Method::GET, "/lor/inventory/v1/cards/me", "lor-inventory-v1.getCards"),
    (
        Method::GET,
        "/lor/match/v1/matches/by-puuid/{puuid}/ids",
        "lor-match-v1.getMatchIdsByPUUID",
    ),
    (Method::GET, "/lor/match/v1/matches/{matchId}", "lor-match-v1.getMatch"),
    (Method::GET, "/lor/ranked/v1/leaderboards", "lor-ranked-v1.getLeaderboards"),
    (Method::GET, "/lor/status/v1/platform-data", "lor-status-v1.getPlatformData"),
    (
        Method::GET,
        "/lol/match/v5/matches/by-puuid/{puuid}/ids",
        "match-v5.getMatchIdsByPUUID",
    ),
    (Method::GET, "/lol/match/v5/matches/{matchId}", "match-v5.getMatch"),
    (Method::GET, "/lol/match/v5/matches/{matchId}/timeline", "match-v5.getTimeline"),
    (
        Method::GET,
        "/lol/spectator/tft/v5/active-games/by-puuid/{encryptedPUUID}",
        "spectator-tft-v5.getCurrentGameInfoByPuuid",
    ),
    (
        Method::GET,
        "/lol/spectator/tft/v5/featured-games",
        "spectator-tft-v5.getFeaturedGames",
    ),
    (
        Method::GET,
        "/lol/spectator/v5/active-games/by-summoner/{encryptedPUUID}",
        "spectator-v5.getCurrentGameInfoByPuuid",
    ),
    (Method::GET, "/lol/spectator/v5/featured-games", "spectator-v5.getFeaturedGames"),
    (
        Method::GET,
        "/fulfillment/v1/summoners/by-puuid/{rsoPUUID}",
        "summoner-v4.getByRSOPUUID",
    ),
    (
        Method::GET,
        "/lol/summoner/v4/summoners/by-account/{encryptedAccountId}",
        "summoner-v4.getByAccountId",
    ),
    (
        Method::GET,
        "/lol/summoner/v4/summoners/by-puuid/{encryptedPUUID}",
        "summoner-v4.getByPUUID",
    ),
    (Method::GET, "/lol/summoner/v4/summoners/me", "summoner-v4.getByAccessToken"),
    (
        Method::GET,
        "/lol/summoner/v4/summoners/{encryptedSummonerId}",
        "summoner-v4.getBySummonerId",
    ),
    (Method::GET, "/tft/league/v1/challenger", "tft-league-v1.getChallengerLeague"),
    (
        Method::GET,
        "/tft/league/v1/entries/by-summoner/{summonerId}",
        "tft-league-v1.getLeagueEntriesForSummoner",
    ),
    (
        Method::GET,
        "/tft/league/v1/entries/{tier}/{division}",
        "tft-league-v1.getLeagueEntries",
    ),
    (Method::GET, "/tft/league/v1/grandmaster", "tft-league-v1.getGrandmasterLeague"),
    (Method::GET, "/tft/league/v1/leagues/{leagueId}", "tft-league-v1.getLeagueById"),
    (Method::GET, "/tft/league/v1/master", "tft-league-v1.getMasterLeague"),
    (
        Method::GET,
        "/tft/league/v1/rated-ladders/{queue}/top",
        "tft-league-v1.getTopRatedLadder",
    ),
    (
        Method::GET,
        "/tft/match/v1/matches/by-puuid/{puuid}/ids",
        "tft-match-v1.getMatchIdsByPUUID",
    ),
    (Method::GET, "/tft/match/v1/matches/{matchId}", "tft-match-v1.getMatch"),
    (Method::GET, "/tft/status/v1/platform-data", "tft-status-v1.getPlatformData"),
    (
        Method::GET,
        "/tft/summoner/v1/summoners/by-account/{encryptedAccountId}",
        "tft-summoner-v1.getByAccountId",
    ),
    (
        Method::GET,
        "/tft/summoner/v1/summoners/by-puuid/{encryptedPUUID}",
        "tft-summoner-v1.getByPUUID",
    ),
    (Method::GET, "/tft/summoner/v1/summoners/me", "tft-summoner-v1.getByAccessToken"),
    (
        Method::GET,
        "/tft/summoner/v1/summoners/{encryptedSummonerId}",
        "tft-summoner-v1.getBySummonerId",
    ),
    (
        Method::POST,
        "/lol/tournament-stub/v5/codes",
        "tournament-stub-v5.createTournamentCode",
    ),
    (
        Method::GET,
        "/lol/tournament-stub/v5/codes/{tournamentCode}",
        "tournament-stub-v5.getTournamentCode",
    ),
    (
        Method::GET,
        "/lol/tournament-stub/v5/lobby-events/by-code/{tournamentCode}",
        "tournament-stub-v5.getLobbyEventsByCode",
    ),
    (
        Method::POST,
        "/lol/tournament-stub/v5/providers",
        "tournament-stub-v5.registerProviderData",
    ),
    (
        Method::POST,
        "/lol/tournament-stub/v5/tournaments",
        "tournament-stub-v5.registerTournament",
    ),
    (Method::POST, "/lol/tournament/v5/codes", "tournament-v5.createTournamentCode"),
    (
        Method::GET,
        "/lol/tournament/v5/codes/{tournamentCode}",
        "tournament-v5.getTournamentCode",
    ),
    (
        Method::PUT,
        "/lol/tournament/v5/codes/{tournamentCode}",
        "tournament-v5.updateCode",
    ),
    (
        Method::GET,
        "/lol/tournament/v5/games/by-code/{tournamentCode}",
        "tournament-v5.getGames",
    ),
    (
        Method::GET,
        "/lol/tournament/v5/lobby-events/by-code/{tournamentCode}",
        "tournament-v5.getLobbyEventsByCode",
    ),
    (Method::POST, "/lol/tournament/v5/providers", "tournament-v5.registerProviderData"),
    (Method::POST, "/lol/tournament/v5/tournaments", "tournament-v5.registerTournament"),
    (
        Method::GET,
        "/val/match/console/v1/matches/{matchId}",
        "val-console-match-v1.getMatch",
    ),
    (
        Method::GET,
        "/val/match/console/v1/matchlists/by-puuid/{puuid}",
        "val-console-match-v1.getMatchlist",
    ),
    (
        Method::GET,
        "/val/match/console/v1/recent-matches/by-queue/{queue}",
        "val-console-match-v1.getRecent",
    ),
    (
        Method::GET,
        "/val/console/ranked/v1/leaderboards/by-act/{actId}",
        "val-console-ranked-v1.getLeaderboard",
    ),
    (Method::GET, "/val/content/v1/contents", "val-content-v1.getContent"),
    (Method::GET, "/val/match/v1/matches/{matchId}", "val-match-v1.getMatch"),
    (
        Method::GET,
        "/val/match/v1/matchlists/by-puuid/{puuid}",
        "val-match-v1.getMatchlist",
    ),
    (
        Method::GET,
        "/val/match/v1/recent-matches/by-queue/{queue}",
        "val-match-v1.getRecent",
    ),
    (
        Method::GET,
        "/val/ranked/v1/leaderboards/by-act/{actId}",
        "val-ranked-v1.getLeaderboard",
    ),
    (Method::GET, "/val/status/v1/platform-data", "val-status-v1.getPlatformData"),
];

