<h1 align="center">
    Riven<br>
    <a href="https://github.com/MingweiSamuel/Riven/"><img src="https://cdn.communitydragon.org/latest/champion/Riven/square" width="20" height="20" alt="Riven Github"></a>
    <a href="https://crates.io/crates/riven"><img src="https://img.shields.io/crates/v/riven?style=flat-square&logo=rust" alt="Crates.io"></a>
    <a href="https://docs.rs/riven/"><img src="https://img.shields.io/badge/docs.rs-Riven-blue?style=flat-square&logo=read-the-docs&logoColor=white" alt="Docs.rs"></a>
    <a href="https://travis-ci.com/MingweiSamuel/Riven"><img src="https://img.shields.io/travis/com/mingweisamuel/riven?style=flat-square" alt="Travis CI"></a>
    <a href="https://github.com/rust-secure-code/safety-dance/"><img src="https://img.shields.io/badge/unsafe-forbidden-green.svg?style=flat-square" alt="unsafe forbidden"></a>
</h1>

Rust Library for the [Riot Games API](https://developer.riotgames.com/).

Rivens's goals are _speed_, _reliability_, and _maintainability_. Riven handles rate limits and large requests with ease.
Data structs and endpoints are automatically generated from the
[Riot API Reference](https://developer.riotgames.com/api-methods/) ([Swagger](http://www.mingweisamuel.com/riotapi-schema/tool/)).

## Design

* Fast, asynchronous, thread-safe.
* Automatically retries failed requests.
* TFT API Support.

## Usage

```rust
use riven::RiotApi;
use riven::consts::Region;

// Riven Enter tokio async runtime.
let rt = tokio::runtime::Runtime::new().unwrap();
rt.block_on(async {
    // Create RiotApi instance from key.
    let api_key = "RGAPI-01234567-89ab-cdef-0123-456789abcdef";
    let api_key = std::env::var("RGAPI_KEY").ok()
        .or_else(|| std::fs::read_to_string("apikey.txt").ok()).unwrap();
    let riot_api = RiotApi::with_key(api_key);

    // Get summoner data.
    let summoner = riot_api.summoner_v4()
        .get_by_summoner_name(Region::NA, "잘못").await
        .expect("Get summoner failed.")
        .expect("There is no summoner with that name.");

    // Print summoner name.
    println!("{} Champion Masteries:", summoner.name);

    // Get champion mastery data.
    let masteries = riot_api.champion_mastery_v4()
        .get_all_champion_masteries(Region::NA, &summoner.id).await
        .expect("Get champion masteries failed.");

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

### Nightly vs Stable

Enable the `nightly` feature to use nightly-only functionality. Mainly enables
[nightly optimizations in the `parking_lot` crate](https://github.com/Amanieu/parking_lot#nightly-vs-stable).
Also required for running async integration tests.

### Docs

[On docs.rs](https://docs.rs/riven/).

### Error Handling

Riven returns `Result<Option<T>>` within futures. If the `Result` is errored,
this indicates that the API request failed to complete successfully, which may be
due to bad user input, Riot server errors, incorrect API key, etc. If the `Option`
is `None`, this indicates that the request completed successfully but no data was
returned. This happens if a summoner (by name) or match (by id) doesn't exist.

### Additional Info

Feel free to [make an issue](https://github.com/MingweiSamuel/Riven/issues/new)
if you are have any questions or trouble using Riven.
