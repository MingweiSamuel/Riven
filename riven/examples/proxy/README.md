# Riven Example Proxy

This is a simple example implementation of a Riot API proxy server using
[`hyper`](https://github.com/hyperium/hyper). This adds the API key and
forwards requests to the Riot API, then returns and forwards responses back to
the requester.

This can handle not just GET endpoints but also POST and PUT (and any others)
with bodies, however it does not handle RSO endpoints which require an
`Authorization` header parameter. This handles errors but provides only minimal
failure information. Rate limits are enforced, so requests will wait to complete
when Riven is at the rate limit.

Put your API key into either the `RGAPI_KEY` environment variable or the `apikey.txt`
file, then run the proxy:
```bash
cargo run --example proxy --features riven/__proxy
```

Test in your browser or using `curl`. The first path segment specifies the region:
```bash
$ curl http://localhost:3000/americas/riot/account/v1/accounts/by-riot-id/LugnutsK/000
{"puuid":"...","gameName":"LugnutsK","tagLine":"000"}

$ curl http://localhost:3000/eu/val/status/v1/platform-data
{"id": "EU", "name": "Europe", "locales": ["..."], "maintenances": [], "incidents": []}

$ curl http://localhost:3000/americas/lol/tournament-stub/v5/providers \
    -H "Content-Type: application/json" -d '{"region":"JP","url":"https://github.com/MingweiSamuel/Riven"}'
1

$ curl http://localhost:3000/na1/unknown/endpoint
{"error":"Riot API endpoint method not found."}
```
