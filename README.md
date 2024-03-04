<h1 align="center">
    Riven<br>
</h1>
<p align="center">
    <a href="https://github.com/MingweiSamuel/Riven/"><img src="https://cdn.communitydragon.org/latest/champion/Riven/square" width="20" height="20" alt="Riven Github"></a>
    <a href="https://crates.io/crates/riven"><img src="https://img.shields.io/crates/v/riven?style=flat-square&logo=rust" alt="Crates.io"></a>
    <a href="https://docs.rs/riven/"><img src="https://img.shields.io/badge/docs.rs-Riven-blue?style=flat-square&logo=read-the-docs&logoColor=white" alt="Docs.rs"></a>
    <!--<a href="https://travis-ci.com/MingweiSamuel/Riven"><img src="https://img.shields.io/travis/com/mingweisamuel/riven?style=flat-square" alt="Travis CI"></a>-->
    <a href="https://github.com/rust-secure-code/safety-dance/"><img src="https://img.shields.io/badge/unsafe-forbidden-green.svg?style=flat-square" alt="unsafe forbidden"></a>
</p>

Rust Library for the [Riot Games API](https://developer.riotgames.com/).

Riven's goals are _speed_, _reliability_, and _maintainability_. Riven handles rate limits and large requests with ease.
Data structs and endpoints are automatically generated from the
[Riot API Reference](https://developer.riotgames.com/api-methods/) ([Swagger](http://www.mingweisamuel.com/riotapi-schema/tool/)).

# Design

* Fast, asynchronous, thread-safe.
* Automatically retries failed requests, configurable.
* Supports all endpoints, kept up-to-date using [riotapi-schema](https://github.com/MingweiSamuel/riotapi-schema).
* Can compile to Wasm for server-side or browser+proxy use.

# Usage

```rust
use riven::RiotApi;
use riven::consts::PlatformRoute;

// Enter tokio async runtime.
let rt = tokio::runtime::Runtime::new().unwrap();
rt.block_on(async {
    // Create RiotApi instance from key string.
    let api_key = std::env!("RGAPI_KEY"); // "RGAPI-01234567-89ab-cdef-0123-456789abcdef";
    let riot_api = RiotApi::new(api_key);

    // Get summoner data.
    let summoner = riot_api.summoner_v4()
        .get_by_summoner_name(PlatformRoute::NA1, "잘 못").await
        .expect("Get summoner failed.")
        .expect("There is no summoner with that name.");

    // Print summoner name.
    println!("{} Champion Masteries:", summoner.name);

    // Get champion mastery data.
    let masteries = riot_api.champion_mastery_v4()
        .get_all_champion_masteries_by_puuid(PlatformRoute::NA1, &summoner.puuid).await
        .expect("Get champion masteries failed.");

    // Print champion masteries.
    for (i, mastery) in masteries.iter().take(10).enumerate() {
        println!("{: >2}) {: <9}    {: >7} ({})", i + 1,
            mastery.champion_id.name().unwrap_or("UNKNOWN"),
            mastery.champion_points, mastery.champion_level);
    }
});
```
Output:
```text
잘 못 Champion Masteries:
 1) Riven        1236866 (7)
 2) Fiora         230679 (5)
 3) Katarina      175985 (5)
 4) Lee Sin       156070 (7)
 5) Jax           102662 (5)
 6) Gnar           76373 (6)
 7) Kai'Sa         64271 (5)
 8) Caitlyn        46614 (5)
 9) Irelia         46465 (5)
10) Vladimir       37176 (5)
```
The [`RiotApi` struct documentation](https://docs.rs/riven/latest/riven/struct.RiotApi.html)
contains additional usage information. The [tests](https://github.com/MingweiSamuel/Riven/tree/v/2.x.x/riven/tests)
and [example proxy](https://github.com/MingweiSamuel/Riven/tree/v/2.x.x/riven/examples/proxy)
provide more example usage.

## Feature Flags

### Nightly vs Stable

Enable the `nightly` feature to use nightly-only functionality. This enables
[nightly optimizations in the `parking_lot` crate](https://github.com/Amanieu/parking_lot#nightly-vs-stable).

```toml
riven = { version = "...", features = [ "nightly" ] }
```

### rustls

Riven uses [reqwest](https://github.com/seanmonstar/reqwest) for making requests. By default, reqwest uses the native TLS library.
If you prefer using [rustls](https://github.com/ctz/rustls) you can do so by turning off the Riven default features
and specifying the `rustls-tls` feature:

```toml
riven = { version = "...", default-features = false, features = [ "rustls-tls" ] }
```

### `log` or `tracing`

Riven is additionally able to produce [tracing](https://docs.rs/tracing) spans for requests if the `tracing` feature is enabled.
By default the `tracing` feature is disabled and Riven instead writes to [`log`](https://docs.rs/log).

## Docs

[On docs.rs](https://docs.rs/riven/).

## Error Handling

Riven returns either `Result<T>` or `Result<Option<T>>` within futures.

If the `Result` is errored, this indicates that the API request failed to
complete successfully, which may be due to bad user input, Riot server errors,
incorrect API key, etc.

If the `Option` is `None`, this indicates that the request completed
successfully but no data was returned. This happens in several situations, such
as getting a summoner (by name) or match (by id) that doesn't exist, or getting
spectator data for a summoner who is not in-game.
Specifically, the API returned a 404 HTTP status code in this situation.

The error type used by Riven is `riven::RiotApiError`. It provides some basic
diagnostic information, such as the source Reqwest error, the number of retries
attempted, and the Reqwest `Response` object.

You can configure the number of time Riven retries using
`RiotApiConfig::set_retries(...)` and the `RiotApi::from_config(config)`
constructor. By default, Riven retries up to 3 times (4 requests total).
Some errors, such as 400 client errors, are not retried as they would
inevitably fail again.

## Semantic Versioning

This package follows semantic versioning to an extent. However, the Riot API
itself changes often and does not follow semantic versioning, which makes
things difficult. Out-of-date versions will slowly partially cease to work due
to this.

When the API changes, this may result in breaking changes in the `models`
module, `endpoints` module, and some of the `consts` module. "Handle accessor"
methods may be removed from `RiotApi` if the corresponding endpoint is removed
from the Riot API. These breaking changes will increment the **MINOR** version,
not the major version.

Parts of Riven that do not depend on Riot API changes do follow semantic
versioning.

## Additional Help

Feel free to [make an issue](https://github.com/MingweiSamuel/Riven/issues/new)
if you are have any questions or trouble with Riven.

# Development

NodeJS is used to generate code for Riven. The
[`riven/srcgen`](https://github.com/MingweiSamuel/Riven/tree/v/2.x.x/riven/srcgen)
folder contains the code and [doT.js](https://olado.github.io/doT/index.html)
templates. `index.js` lists the JSON files downloaded and used to generate the
code.

To set up the srcgen, you will first need to install NodeJS. Then enter the
`riven/srcgen` folder and run `npm ci` (or `npm install`) to install
dependencies.

To run the srcgen use `node riven/srcgen` from the repository root.

