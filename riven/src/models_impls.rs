use crate::consts::Champion;
use crate::models::match_v5::Participant;

impl Participant {
    /// This method takes the [`Self::champion_id`] field if it is valid
    /// (`Ok`), otherwise it attempts to parse [`Self::champion_name`] and
    /// returns the `Result`.
    ///
    /// This is needed because some of Riot's [`Self::champion_id`] data is
    /// corrupted, as they describe in the docs:
    ///
    /// > Prior to patch 11.4, on Feb 18th, 2021, this field returned invalid
    /// > championIds. We recommend determining the champion based on the
    /// > championName field for matches played prior to patch 11.4.
    ///
    /// This issue is reported here: <https://github.com/RiotGames/developer-relations/issues/553>.
    pub fn champion(&self) -> Result<Champion, <Champion as std::str::FromStr>::Err> {
        #[allow(deprecated)]
        self.champion_id.or_else(|_| self.champion_name.parse())
    }

    /// This method returns the name portion of the riot ID for this summoner.
    ///
    /// Prior to patch 14.5, this was in the [`Self::riot_id_name`] field.
    /// After, this moved to the `Self.riot_id_game_name` field.
    ///
    /// This method simply returns whichever of the two fields is not `None`.
    pub fn riot_id_game_name(&self) -> Option<&str> {
        self.riot_id_game_name
            .as_deref()
            .or(self.riot_id_name.as_deref())
    }
}
