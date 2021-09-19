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

Set `RGAPI_KEY` env var then run:
```bash
export RGAPI_KEY=RGAPI-XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX
cargo run
```

Test in your browser or using `curl`. The first path segment specifies the region:
```json
$ curl http://localhost:3000/na1/lol/summoner/v4/summoners/by-name/LugnutsK
{"id":"...","accountId":"...","puuid":"...","name":"LugnutsK","profileIconId":4540,"revisionDate":1589704662000,"summonerLevel":111}

$ curl http://localhost:3000/eu/val/status/v1/platform-data
{"id": "EU", "name": "Europe", "locales": ["..."], "maintenances": [], "incidents": []}

$ curl http://localhost:3000/americas/lol/tournament-stub/v4/providers -H "Content-Type: application/json" -d '{"region":"JP","url":"https://github.com/MingweiSamuel/Riven"}'
764

$ curl http://localhost:3000/na1/unknown/endpoint
{"error":"Riot API endpoint method not found."}
```
