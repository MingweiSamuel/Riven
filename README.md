# Riven
[![Crates.io](https://img.shields.io/crates/v/riven?style=flat-square)](https://crates.io/crates/riven)
[![Travis CI](https://img.shields.io/travis/com/mingweisamuel/riven?style=flat-square)](https://travis-ci.com/MingweiSamuel/Riven)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square)](https://github.com/rust-secure-code/safety-dance/)

Rust Library for the [Riot Games API](https://developer.riotgames.com/).

Rivens's goals are _speed_, _reliability_, and _maintainability_. Riven handles rate limits and large requests with ease.
Data structs and endpoints are automatically generated from the
[Riot API Reference](https://developer.riotgames.com/api-methods/) ([Swagger](http://www.mingweisamuel.com/riotapi-schema/tool/)).

Riven currently uses nightly Rust.

## Features

* Fast, asynchronous, thread-safe.
* Automatically retries failed requests.
* TFT API Support.

## Usage

```rust
use riven::RiotApi;
use riven::consts::Region;

// Enter tokio async runtime.
let rt = tokio::runtime::Runtime::new().unwrap();
rt.block_on(async {
    // Create RiotApi instance from key.
    let api_key = "RGAPI-01234567-89ab-cdef-0123-456789abcdef";
    # let api_key = std::env::var("RGAPI_KEY").ok().or_else(|| std::fs::read_to_string("apikey.txt").ok()).unwrap();
    let riot_api = RiotApi::with_key(api_key);

    // Get summoner data.
    let summoner = riot_api.summoner_v4()
        .get_by_summoner_name(Region::NA, "잘못").await
        .expect("Get summoner failed.")
        .expect("Summoner not found.");

    // Print summoner name.
    println!("{} Champion Masteries:", summoner.name);

    // Get champion mastery data.
    let masteries = riot_api.champion_mastery_v4()
        .get_all_champion_masteries(Region::NA, &summoner.id).await
        .expect("Get champion masteries failed.")
        .unwrap();

    // Print champioon masteries.
    for (i, mastery) in masteries[..10].iter().enumerate() {
        println!("{: >2}) {: <9}    {: >7} ({})", i + 1,
            mastery.champion_id.to_string(),
            mastery.champion_points, mastery.champion_level);
    }
});
```
Output:
```text
잘 못 Champion Masteries:
 1) Riven        1219895 (7)
 2) Fiora         229714 (5)
 3) Katarina      175985 (5)
 4) Lee Sin       150546 (7)
 5) Jax           100509 (5)
 6) Gnar           76373 (6)
 7) Kai'Sa         64271 (5)
 8) Caitlyn        46479 (5)
 9) Irelia         46465 (5)
10) Vladimir       37176 (5)
```
