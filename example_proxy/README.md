# Riven Example Proxy

<span color="red">This is not yet updated for V2.</span>

This is a simple example implementation of a Riot API proxy server using `hyper`. This adds the API key and forwards
requests to the Riot API, then returns and forwards responses back to the requester. It handles error cases but only
provides minimal failure information. HTTP requests will wait to complete when Riven is waiting on rate limits.

Set `RGAPI_KEY` env var then run:
```bash
export RGAPI_KEY=RGAPI-XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX
cargo run
```

Test in your browser or using `curl`. The first path segment specifies the region:
```json
$ curl http://localhost:3000/na1/lol/summoner/v4/summoners/by-name/LugnutsK
{"id":"...","accountId":"...","puuid":"...","name":"LugnutsK","profileIconId":4540,"revisionDate":1589704662000,"summonerLevel":111}

$ curl http://localhost:3000/na1/valorant/v4/players/by-name/LugnutsK # not yet :)
{"error":"Riot API endpoint method not found."}
```
