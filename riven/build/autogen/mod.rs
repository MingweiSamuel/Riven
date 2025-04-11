use std::borrow::Cow;
use std::future::Future;
use std::path::Path;

use syn::parse_quote;

mod champion;
mod endpoints;
mod enum_unknown;
mod game_type;
mod meta;
mod models;
mod newtype_enum;
mod route;
mod spec;

const CODEGEN_NOTE: &str = r#"Note: this item is generated based on <a href="https://github.com/MingweiSamuel/riotapi-schema" target="_blank">`riotapi-schema`</a>."#;

fn spec() -> impl Future<Output = &'static spec::Spec> {
    static SPEC: tokio::sync::OnceCell<spec::Spec> = tokio::sync::OnceCell::const_new();
    SPEC.get_or_init(|| get("http://www.mingweisamuel.com/riotapi-schema/openapi-3.0.0.json"))
}

pub fn autogen(src_dir: impl AsRef<Path>, consts_dir: impl AsRef<Path>) {
    let src_dir = src_dir.as_ref();
    let consts_dir = consts_dir.as_ref();

    let endpoints_gen = async {
        let spec = spec().await;
        write(src_dir, "endpoints.gen.rs", endpoints::endpoints(spec)).await;
    };
    let meta_gen = async {
        let spec = spec().await;
        write(src_dir, "meta.gen.rs", meta::meta(spec)).await;
    };
    let models_gen = async {
        let spec = spec().await;
        write(src_dir, "models.gen.rs", models::models(spec)).await;
    };

    let champions_gen = async {
        let champions = get::<Vec<_>>(
            "http://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/default/v1/champion-summary.json"
        );
        write(
            consts_dir,
            "champion.gen.rs",
            champion::champion(&champions.await),
        )
        .await;
    };
    let game_mode_gen = async {
        let game_modes =
            get::<Vec<_>>("http://www.mingweisamuel.com/riotapi-schema/enums/gameModes.json");
        write(
            consts_dir, "game_mode.gen.rs",
            enum_unknown::enum_unknown(
                &game_modes.await,
                parse_quote!(GameMode),
                parse_quote!(u8),
                "League of Legends game mode, such as Classic, ARAM, URF, One For All, Ascension, etc.",
            ),
        )
        .await;
    };
    let game_type_gen = async {
        let game_types =
            get::<Vec<_>>("http://www.mingweisamuel.com/riotapi-schema/enums/gameTypes.json");
        write(
            consts_dir,
            "game_type.gen.rs",
            game_type::game_type(&game_types.await),
        )
        .await;
    };
    let maps_gen = async {
        let maps = get::<Vec<_>>("http://www.mingweisamuel.com/riotapi-schema/enums/maps.json");
        write(
            consts_dir,
            "map.gen.rs",
            newtype_enum::newtype_enum(
                &maps.await,
                parse_quote!(Map),
                parse_quote!(u8),
                "A League of Legends map.",
            ),
        )
        .await;
    };
    let queue_gen = async {
        let queues = get::<Vec<_>>("http://www.mingweisamuel.com/riotapi-schema/enums/queues.json");
        write(
            consts_dir,
            "queue.gen.rs",
            newtype_enum::newtype_enum(
                &queues.await,
                parse_quote!(Queue),
                parse_quote!(u16),
                "A League of Legends matchmaking queue.",
            ),
        )
        .await;
    };
    let queue_type_gen = async {
        let queue_types =
            get::<Vec<_>>("http://www.mingweisamuel.com/riotapi-schema/enums/queueTypes.json");
        write(
            consts_dir,
            "queue_type.gen.rs",
            enum_unknown::enum_unknown(
                &queue_types.await,
                parse_quote!(QueueType),
                parse_quote!(u8),
                "LoL or TFT queue types.",
            ),
        )
        .await;
    };
    let route_gen = async {
        let routes_table = get("http://www.mingweisamuel.com/riotapi-schema/routesTable.json");
        write(
            consts_dir,
            "route.gen.rs",
            route::route(&routes_table.await),
        )
        .await;
    };
    let season_gen = async {
        let seasons =
            get::<Vec<_>>("http://www.mingweisamuel.com/riotapi-schema/enums/seasons.json");
        write(
            consts_dir,
            "season.gen.rs",
            newtype_enum::newtype_enum(
                &seasons.await,
                parse_quote!(Season),
                parse_quote!(u8),
                "A League of Legends season for competitive matchmaking.",
            ),
        )
        .await
    };

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            tokio::join!(
                champions_gen,
                endpoints_gen,
                meta_gen,
                models_gen,
                season_gen,
                queue_gen,
                queue_type_gen,
                game_mode_gen,
                game_type_gen,
                maps_gen,
                route_gen,
            );
        })
}

async fn write(dir: &Path, filename: &str, file: syn::File) {
    use std::io::Write;

    let path = dir.join(filename);
    let mut writer = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
    let contents = prettyplease::unparse(&file);
    let version = &*spec().await.info.version;
    tokio::task::spawn_blocking(move || {
        writeln!(
            writer,
            "// http://www.mingweisamuel.com/riotapi-schema/tool/"
        )?;
        writeln!(writer, "// Version: {}", version)?;
        writeln!(writer)?;
        writeln!(writer, "{}", contents)?;
        Ok::<(), std::io::Error>(())
    })
    .await
    .unwrap()
    .unwrap()
}

fn get<T: serde::de::DeserializeOwned>(url: &str) -> impl use<'_, T> + Future<Output = T> {
    static CLIENT: std::sync::OnceLock<reqwest::Client> = std::sync::OnceLock::new();
    let client = CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .user_agent("riven")
            .build()
            .unwrap()
    });
    async move {
        let bytes = client
            .get(url)
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .bytes()
            .await
            .unwrap();
        serde_json::from_slice::<T>(&bytes).unwrap()
    }
}

fn process_description(description: &str) -> impl Iterator<Item = Cow<'_, str>> {
    description.lines().map(str::trim).map(|line| {
        let mut line = Cow::Borrowed(line);
        let mut i = 0;
        while let Some(url_start) = line[i..]
            .find("https://")
            .or_else(|| line[i..].find("http://"))
            .map(|j| i + j)
        {
            let mut url_end = line[url_start..]
                .find(char::is_whitespace)
                .map_or(line.len(), |j| url_start + j);
            while line.as_bytes().get(url_end - 1) == Some(&b'.') {
                url_end -= 1;
            }

            let mut_line = line.to_mut();
            mut_line.insert(url_end, '>');
            mut_line.insert(url_start, '<');

            i = url_end + 1;
        }
        line
    })
}
