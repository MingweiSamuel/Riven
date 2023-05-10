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
}
