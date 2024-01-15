use super::*;

#[test]
fn check_as_ref() {
    assert_eq!("MATCHED_GAME", GameType::MATCHED_GAME.as_ref());
}

#[test]
fn check_to_string() {
    assert_eq!("MATCHED_GAME", GameType::MATCHED_GAME.to_string());
}

#[test]
fn check_from_string() {
    assert_eq!(Ok(GameType::MATCHED_GAME), "MATCHED_GAME".parse());
    assert_eq!(Ok(GameType::MATCHED_GAME), "MATCHED".parse());
}

#[test]
fn check_serialize() {
    assert_eq!(Some("\"MATCHED_GAME\""),
        serde_json::to_string(&GameType::MATCHED_GAME)
            .ok().as_deref());
}
