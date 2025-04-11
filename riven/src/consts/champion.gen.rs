// http://www.mingweisamuel.com/riotapi-schema/tool/
// Version: 996d171a2b79e9bb85c549f47b07c6ef2721fc8a

#[macro_rules_attribute::apply(newtype_enum)]
#[repr(i16)]
/// A League of Legends champion.
///
/// This newtype acts as a C-like enum; each variant corresponds to an
/// integer value. Using a newtype allows _unknown_ variants to be
/// represented. This is important when Riot adds new champions.
///
/// Field | Name | Identifier | Id
/// ---|---|---|---
/// `NONE` | None (no ban) | | -1
///`1` | Annie | Annie | 1
///`2` | Olaf | Olaf | 2
///`3` | Galio | Galio | 3
///`4` | Twisted Fate | TwistedFate | 4
///`5` | Xin Zhao | XinZhao | 5
///`6` | Urgot | Urgot | 6
///`7` | LeBlanc | Leblanc | 7
///`8` | Vladimir | Vladimir | 8
///`9` | Fiddlesticks | FiddleSticks | 9
///`10` | Kayle | Kayle | 10
///`11` | Master Yi | MasterYi | 11
///`12` | Alistar | Alistar | 12
///`13` | Ryze | Ryze | 13
///`14` | Sion | Sion | 14
///`15` | Sivir | Sivir | 15
///`16` | Soraka | Soraka | 16
///`17` | Teemo | Teemo | 17
///`18` | Tristana | Tristana | 18
///`19` | Warwick | Warwick | 19
///`20` | Nunu & Willump | Nunu | 20
///`21` | Miss Fortune | MissFortune | 21
///`22` | Ashe | Ashe | 22
///`23` | Tryndamere | Tryndamere | 23
///`24` | Jax | Jax | 24
///`25` | Morgana | Morgana | 25
///`26` | Zilean | Zilean | 26
///`27` | Singed | Singed | 27
///`28` | Evelynn | Evelynn | 28
///`29` | Twitch | Twitch | 29
///`30` | Karthus | Karthus | 30
///`31` | Cho'Gath | Chogath | 31
///`32` | Amumu | Amumu | 32
///`33` | Rammus | Rammus | 33
///`34` | Anivia | Anivia | 34
///`35` | Shaco | Shaco | 35
///`36` | Dr. Mundo | DrMundo | 36
///`37` | Sona | Sona | 37
///`38` | Kassadin | Kassadin | 38
///`39` | Irelia | Irelia | 39
///`40` | Janna | Janna | 40
///`41` | Gangplank | Gangplank | 41
///`42` | Corki | Corki | 42
///`43` | Karma | Karma | 43
///`44` | Taric | Taric | 44
///`45` | Veigar | Veigar | 45
///`48` | Trundle | Trundle | 48
///`50` | Swain | Swain | 50
///`51` | Caitlyn | Caitlyn | 51
///`53` | Blitzcrank | Blitzcrank | 53
///`54` | Malphite | Malphite | 54
///`55` | Katarina | Katarina | 55
///`56` | Nocturne | Nocturne | 56
///`57` | Maokai | Maokai | 57
///`58` | Renekton | Renekton | 58
///`59` | Jarvan IV | JarvanIV | 59
///`60` | Elise | Elise | 60
///`61` | Orianna | Orianna | 61
///`62` | Wukong | MonkeyKing | 62
///`63` | Brand | Brand | 63
///`64` | Lee Sin | LeeSin | 64
///`67` | Vayne | Vayne | 67
///`68` | Rumble | Rumble | 68
///`69` | Cassiopeia | Cassiopeia | 69
///`72` | Skarner | Skarner | 72
///`74` | Heimerdinger | Heimerdinger | 74
///`75` | Nasus | Nasus | 75
///`76` | Nidalee | Nidalee | 76
///`77` | Udyr | Udyr | 77
///`78` | Poppy | Poppy | 78
///`79` | Gragas | Gragas | 79
///`80` | Pantheon | Pantheon | 80
///`81` | Ezreal | Ezreal | 81
///`82` | Mordekaiser | Mordekaiser | 82
///`83` | Yorick | Yorick | 83
///`84` | Akali | Akali | 84
///`85` | Kennen | Kennen | 85
///`86` | Garen | Garen | 86
///`89` | Leona | Leona | 89
///`90` | Malzahar | Malzahar | 90
///`91` | Talon | Talon | 91
///`92` | Riven | Riven | 92
///`96` | Kog'Maw | KogMaw | 96
///`98` | Shen | Shen | 98
///`99` | Lux | Lux | 99
///`101` | Xerath | Xerath | 101
///`102` | Shyvana | Shyvana | 102
///`103` | Ahri | Ahri | 103
///`104` | Graves | Graves | 104
///`105` | Fizz | Fizz | 105
///`106` | Volibear | Volibear | 106
///`107` | Rengar | Rengar | 107
///`110` | Varus | Varus | 110
///`111` | Nautilus | Nautilus | 111
///`112` | Viktor | Viktor | 112
///`113` | Sejuani | Sejuani | 113
///`114` | Fiora | Fiora | 114
///`115` | Ziggs | Ziggs | 115
///`117` | Lulu | Lulu | 117
///`119` | Draven | Draven | 119
///`120` | Hecarim | Hecarim | 120
///`121` | Kha'Zix | Khazix | 121
///`122` | Darius | Darius | 122
///`126` | Jayce | Jayce | 126
///`127` | Lissandra | Lissandra | 127
///`131` | Diana | Diana | 131
///`133` | Quinn | Quinn | 133
///`134` | Syndra | Syndra | 134
///`136` | Aurelion Sol | AurelionSol | 136
///`141` | Kayn | Kayn | 141
///`142` | Zoe | Zoe | 142
///`143` | Zyra | Zyra | 143
///`145` | Kai'Sa | Kaisa | 145
///`147` | Seraphine | Seraphine | 147
///`150` | Gnar | Gnar | 150
///`154` | Zac | Zac | 154
///`157` | Yasuo | Yasuo | 157
///`161` | Vel'Koz | Velkoz | 161
///`163` | Taliyah | Taliyah | 163
///`164` | Camille | Camille | 164
///`166` | Akshan | Akshan | 166
///`200` | Bel'Veth | Belveth | 200
///`201` | Braum | Braum | 201
///`202` | Jhin | Jhin | 202
///`203` | Kindred | Kindred | 203
///`221` | Zeri | Zeri | 221
///`222` | Jinx | Jinx | 222
///`223` | Tahm Kench | TahmKench | 223
///`233` | Briar | Briar | 233
///`234` | Viego | Viego | 234
///`235` | Senna | Senna | 235
///`236` | Lucian | Lucian | 236
///`238` | Zed | Zed | 238
///`240` | Kled | Kled | 240
///`245` | Ekko | Ekko | 245
///`246` | Qiyana | Qiyana | 246
///`254` | Vi | Vi | 254
///`266` | Aatrox | Aatrox | 266
///`267` | Nami | Nami | 267
///`268` | Azir | Azir | 268
///`350` | Yuumi | Yuumi | 350
///`360` | Samira | Samira | 360
///`412` | Thresh | Thresh | 412
///`420` | Illaoi | Illaoi | 420
///`421` | Rek'Sai | RekSai | 421
///`427` | Ivern | Ivern | 427
///`429` | Kalista | Kalista | 429
///`432` | Bard | Bard | 432
///`497` | Rakan | Rakan | 497
///`498` | Xayah | Xayah | 498
///`516` | Ornn | Ornn | 516
///`517` | Sylas | Sylas | 517
///`518` | Neeko | Neeko | 518
///`523` | Aphelios | Aphelios | 523
///`526` | Rell | Rell | 526
///`555` | Pyke | Pyke | 555
///`711` | Vex | Vex | 711
///`777` | Yone | Yone | 777
///`799` | Ambessa | Ambessa | 799
///`800` | Mel | Mel | 800
///`875` | Sett | Sett | 875
///`876` | Lillia | Lillia | 876
///`887` | Gwen | Gwen | 887
///`888` | Renata Glasc | Renata | 888
///`893` | Aurora | Aurora | 893
///`895` | Nilah | Nilah | 895
///`897` | K'Sante | KSante | 897
///`901` | Smolder | Smolder | 901
///`902` | Milio | Milio | 902
///`910` | Hwei | Hwei | 910
///`950` | Naafiri | Naafiri | 950
pub enum Champion {
    /// `-1`, none. Appears when a champion ban is not used in champ select.
    NONE = -1,
    ///`1`
    ANNIE = 1,
    ///`2`
    OLAF = 2,
    ///`3`
    GALIO = 3,
    ///`4`
    TWISTED_FATE = 4,
    ///`5`
    XIN_ZHAO = 5,
    ///`6`
    URGOT = 6,
    ///`7`
    LEBLANC = 7,
    ///`8`
    VLADIMIR = 8,
    ///`9`
    FIDDLE_STICKS = 9,
    ///`10`
    KAYLE = 10,
    ///`11`
    MASTER_YI = 11,
    ///`12`
    ALISTAR = 12,
    ///`13`
    RYZE = 13,
    ///`14`
    SION = 14,
    ///`15`
    SIVIR = 15,
    ///`16`
    SORAKA = 16,
    ///`17`
    TEEMO = 17,
    ///`18`
    TRISTANA = 18,
    ///`19`
    WARWICK = 19,
    ///`20`
    NUNU = 20,
    ///`21`
    MISS_FORTUNE = 21,
    ///`22`
    ASHE = 22,
    ///`23`
    TRYNDAMERE = 23,
    ///`24`
    JAX = 24,
    ///`25`
    MORGANA = 25,
    ///`26`
    ZILEAN = 26,
    ///`27`
    SINGED = 27,
    ///`28`
    EVELYNN = 28,
    ///`29`
    TWITCH = 29,
    ///`30`
    KARTHUS = 30,
    ///`31`
    CHOGATH = 31,
    ///`32`
    AMUMU = 32,
    ///`33`
    RAMMUS = 33,
    ///`34`
    ANIVIA = 34,
    ///`35`
    SHACO = 35,
    ///`36`
    DR_MUNDO = 36,
    ///`37`
    SONA = 37,
    ///`38`
    KASSADIN = 38,
    ///`39`
    IRELIA = 39,
    ///`40`
    JANNA = 40,
    ///`41`
    GANGPLANK = 41,
    ///`42`
    CORKI = 42,
    ///`43`
    KARMA = 43,
    ///`44`
    TARIC = 44,
    ///`45`
    VEIGAR = 45,
    ///`48`
    TRUNDLE = 48,
    ///`50`
    SWAIN = 50,
    ///`51`
    CAITLYN = 51,
    ///`53`
    BLITZCRANK = 53,
    ///`54`
    MALPHITE = 54,
    ///`55`
    KATARINA = 55,
    ///`56`
    NOCTURNE = 56,
    ///`57`
    MAOKAI = 57,
    ///`58`
    RENEKTON = 58,
    ///`59`
    JARVAN_IV = 59,
    ///`60`
    ELISE = 60,
    ///`61`
    ORIANNA = 61,
    ///`62`
    MONKEY_KING = 62,
    ///`63`
    BRAND = 63,
    ///`64`
    LEE_SIN = 64,
    ///`67`
    VAYNE = 67,
    ///`68`
    RUMBLE = 68,
    ///`69`
    CASSIOPEIA = 69,
    ///`72`
    SKARNER = 72,
    ///`74`
    HEIMERDINGER = 74,
    ///`75`
    NASUS = 75,
    ///`76`
    NIDALEE = 76,
    ///`77`
    UDYR = 77,
    ///`78`
    POPPY = 78,
    ///`79`
    GRAGAS = 79,
    ///`80`
    PANTHEON = 80,
    ///`81`
    EZREAL = 81,
    ///`82`
    MORDEKAISER = 82,
    ///`83`
    YORICK = 83,
    ///`84`
    AKALI = 84,
    ///`85`
    KENNEN = 85,
    ///`86`
    GAREN = 86,
    ///`89`
    LEONA = 89,
    ///`90`
    MALZAHAR = 90,
    ///`91`
    TALON = 91,
    ///`92`
    RIVEN = 92,
    ///`96`
    KOG_MAW = 96,
    ///`98`
    SHEN = 98,
    ///`99`
    LUX = 99,
    ///`101`
    XERATH = 101,
    ///`102`
    SHYVANA = 102,
    ///`103`
    AHRI = 103,
    ///`104`
    GRAVES = 104,
    ///`105`
    FIZZ = 105,
    ///`106`
    VOLIBEAR = 106,
    ///`107`
    RENGAR = 107,
    ///`110`
    VARUS = 110,
    ///`111`
    NAUTILUS = 111,
    ///`112`
    VIKTOR = 112,
    ///`113`
    SEJUANI = 113,
    ///`114`
    FIORA = 114,
    ///`115`
    ZIGGS = 115,
    ///`117`
    LULU = 117,
    ///`119`
    DRAVEN = 119,
    ///`120`
    HECARIM = 120,
    ///`121`
    KHAZIX = 121,
    ///`122`
    DARIUS = 122,
    ///`126`
    JAYCE = 126,
    ///`127`
    LISSANDRA = 127,
    ///`131`
    DIANA = 131,
    ///`133`
    QUINN = 133,
    ///`134`
    SYNDRA = 134,
    ///`136`
    AURELION_SOL = 136,
    ///`141`
    KAYN = 141,
    ///`142`
    ZOE = 142,
    ///`143`
    ZYRA = 143,
    ///`145`
    KAISA = 145,
    ///`147`
    SERAPHINE = 147,
    ///`150`
    GNAR = 150,
    ///`154`
    ZAC = 154,
    ///`157`
    YASUO = 157,
    ///`161`
    VELKOZ = 161,
    ///`163`
    TALIYAH = 163,
    ///`164`
    CAMILLE = 164,
    ///`166`
    AKSHAN = 166,
    ///`200`
    BELVETH = 200,
    ///`201`
    BRAUM = 201,
    ///`202`
    JHIN = 202,
    ///`203`
    KINDRED = 203,
    ///`221`
    ZERI = 221,
    ///`222`
    JINX = 222,
    ///`223`
    TAHM_KENCH = 223,
    ///`233`
    BRIAR = 233,
    ///`234`
    VIEGO = 234,
    ///`235`
    SENNA = 235,
    ///`236`
    LUCIAN = 236,
    ///`238`
    ZED = 238,
    ///`240`
    KLED = 240,
    ///`245`
    EKKO = 245,
    ///`246`
    QIYANA = 246,
    ///`254`
    VI = 254,
    ///`266`
    AATROX = 266,
    ///`267`
    NAMI = 267,
    ///`268`
    AZIR = 268,
    ///`350`
    YUUMI = 350,
    ///`360`
    SAMIRA = 360,
    ///`412`
    THRESH = 412,
    ///`420`
    ILLAOI = 420,
    ///`421`
    REK_SAI = 421,
    ///`427`
    IVERN = 427,
    ///`429`
    KALISTA = 429,
    ///`432`
    BARD = 432,
    ///`497`
    RAKAN = 497,
    ///`498`
    XAYAH = 498,
    ///`516`
    ORNN = 516,
    ///`517`
    SYLAS = 517,
    ///`518`
    NEEKO = 518,
    ///`523`
    APHELIOS = 523,
    ///`526`
    RELL = 526,
    ///`555`
    PYKE = 555,
    ///`711`
    VEX = 711,
    ///`777`
    YONE = 777,
    ///`799`
    AMBESSA = 799,
    ///`800`
    MEL = 800,
    ///`875`
    SETT = 875,
    ///`876`
    LILLIA = 876,
    ///`887`
    GWEN = 887,
    ///`888`
    RENATA = 888,
    ///`893`
    AURORA = 893,
    ///`895`
    NILAH = 895,
    ///`897`
    K_SANTE = 897,
    ///`901`
    SMOLDER = 901,
    ///`902`
    MILIO = 902,
    ///`910`
    HWEI = 910,
    ///`950`
    NAAFIRI = 950,
}
impl Champion {
    /// The champion's name (`en_US` localization).
    pub const fn name(self) -> Option<&'static str> {
        match self {
            Self::ANNIE => Some("Annie"),
            Self::OLAF => Some("Olaf"),
            Self::GALIO => Some("Galio"),
            Self::TWISTED_FATE => Some("Twisted Fate"),
            Self::XIN_ZHAO => Some("Xin Zhao"),
            Self::URGOT => Some("Urgot"),
            Self::LEBLANC => Some("LeBlanc"),
            Self::VLADIMIR => Some("Vladimir"),
            Self::FIDDLE_STICKS => Some("Fiddlesticks"),
            Self::KAYLE => Some("Kayle"),
            Self::MASTER_YI => Some("Master Yi"),
            Self::ALISTAR => Some("Alistar"),
            Self::RYZE => Some("Ryze"),
            Self::SION => Some("Sion"),
            Self::SIVIR => Some("Sivir"),
            Self::SORAKA => Some("Soraka"),
            Self::TEEMO => Some("Teemo"),
            Self::TRISTANA => Some("Tristana"),
            Self::WARWICK => Some("Warwick"),
            Self::NUNU => Some("Nunu & Willump"),
            Self::MISS_FORTUNE => Some("Miss Fortune"),
            Self::ASHE => Some("Ashe"),
            Self::TRYNDAMERE => Some("Tryndamere"),
            Self::JAX => Some("Jax"),
            Self::MORGANA => Some("Morgana"),
            Self::ZILEAN => Some("Zilean"),
            Self::SINGED => Some("Singed"),
            Self::EVELYNN => Some("Evelynn"),
            Self::TWITCH => Some("Twitch"),
            Self::KARTHUS => Some("Karthus"),
            Self::CHOGATH => Some("Cho'Gath"),
            Self::AMUMU => Some("Amumu"),
            Self::RAMMUS => Some("Rammus"),
            Self::ANIVIA => Some("Anivia"),
            Self::SHACO => Some("Shaco"),
            Self::DR_MUNDO => Some("Dr. Mundo"),
            Self::SONA => Some("Sona"),
            Self::KASSADIN => Some("Kassadin"),
            Self::IRELIA => Some("Irelia"),
            Self::JANNA => Some("Janna"),
            Self::GANGPLANK => Some("Gangplank"),
            Self::CORKI => Some("Corki"),
            Self::KARMA => Some("Karma"),
            Self::TARIC => Some("Taric"),
            Self::VEIGAR => Some("Veigar"),
            Self::TRUNDLE => Some("Trundle"),
            Self::SWAIN => Some("Swain"),
            Self::CAITLYN => Some("Caitlyn"),
            Self::BLITZCRANK => Some("Blitzcrank"),
            Self::MALPHITE => Some("Malphite"),
            Self::KATARINA => Some("Katarina"),
            Self::NOCTURNE => Some("Nocturne"),
            Self::MAOKAI => Some("Maokai"),
            Self::RENEKTON => Some("Renekton"),
            Self::JARVAN_IV => Some("Jarvan IV"),
            Self::ELISE => Some("Elise"),
            Self::ORIANNA => Some("Orianna"),
            Self::MONKEY_KING => Some("Wukong"),
            Self::BRAND => Some("Brand"),
            Self::LEE_SIN => Some("Lee Sin"),
            Self::VAYNE => Some("Vayne"),
            Self::RUMBLE => Some("Rumble"),
            Self::CASSIOPEIA => Some("Cassiopeia"),
            Self::SKARNER => Some("Skarner"),
            Self::HEIMERDINGER => Some("Heimerdinger"),
            Self::NASUS => Some("Nasus"),
            Self::NIDALEE => Some("Nidalee"),
            Self::UDYR => Some("Udyr"),
            Self::POPPY => Some("Poppy"),
            Self::GRAGAS => Some("Gragas"),
            Self::PANTHEON => Some("Pantheon"),
            Self::EZREAL => Some("Ezreal"),
            Self::MORDEKAISER => Some("Mordekaiser"),
            Self::YORICK => Some("Yorick"),
            Self::AKALI => Some("Akali"),
            Self::KENNEN => Some("Kennen"),
            Self::GAREN => Some("Garen"),
            Self::LEONA => Some("Leona"),
            Self::MALZAHAR => Some("Malzahar"),
            Self::TALON => Some("Talon"),
            Self::RIVEN => Some("Riven"),
            Self::KOG_MAW => Some("Kog'Maw"),
            Self::SHEN => Some("Shen"),
            Self::LUX => Some("Lux"),
            Self::XERATH => Some("Xerath"),
            Self::SHYVANA => Some("Shyvana"),
            Self::AHRI => Some("Ahri"),
            Self::GRAVES => Some("Graves"),
            Self::FIZZ => Some("Fizz"),
            Self::VOLIBEAR => Some("Volibear"),
            Self::RENGAR => Some("Rengar"),
            Self::VARUS => Some("Varus"),
            Self::NAUTILUS => Some("Nautilus"),
            Self::VIKTOR => Some("Viktor"),
            Self::SEJUANI => Some("Sejuani"),
            Self::FIORA => Some("Fiora"),
            Self::ZIGGS => Some("Ziggs"),
            Self::LULU => Some("Lulu"),
            Self::DRAVEN => Some("Draven"),
            Self::HECARIM => Some("Hecarim"),
            Self::KHAZIX => Some("Kha'Zix"),
            Self::DARIUS => Some("Darius"),
            Self::JAYCE => Some("Jayce"),
            Self::LISSANDRA => Some("Lissandra"),
            Self::DIANA => Some("Diana"),
            Self::QUINN => Some("Quinn"),
            Self::SYNDRA => Some("Syndra"),
            Self::AURELION_SOL => Some("Aurelion Sol"),
            Self::KAYN => Some("Kayn"),
            Self::ZOE => Some("Zoe"),
            Self::ZYRA => Some("Zyra"),
            Self::KAISA => Some("Kai'Sa"),
            Self::SERAPHINE => Some("Seraphine"),
            Self::GNAR => Some("Gnar"),
            Self::ZAC => Some("Zac"),
            Self::YASUO => Some("Yasuo"),
            Self::VELKOZ => Some("Vel'Koz"),
            Self::TALIYAH => Some("Taliyah"),
            Self::CAMILLE => Some("Camille"),
            Self::AKSHAN => Some("Akshan"),
            Self::BELVETH => Some("Bel'Veth"),
            Self::BRAUM => Some("Braum"),
            Self::JHIN => Some("Jhin"),
            Self::KINDRED => Some("Kindred"),
            Self::ZERI => Some("Zeri"),
            Self::JINX => Some("Jinx"),
            Self::TAHM_KENCH => Some("Tahm Kench"),
            Self::BRIAR => Some("Briar"),
            Self::VIEGO => Some("Viego"),
            Self::SENNA => Some("Senna"),
            Self::LUCIAN => Some("Lucian"),
            Self::ZED => Some("Zed"),
            Self::KLED => Some("Kled"),
            Self::EKKO => Some("Ekko"),
            Self::QIYANA => Some("Qiyana"),
            Self::VI => Some("Vi"),
            Self::AATROX => Some("Aatrox"),
            Self::NAMI => Some("Nami"),
            Self::AZIR => Some("Azir"),
            Self::YUUMI => Some("Yuumi"),
            Self::SAMIRA => Some("Samira"),
            Self::THRESH => Some("Thresh"),
            Self::ILLAOI => Some("Illaoi"),
            Self::REK_SAI => Some("Rek'Sai"),
            Self::IVERN => Some("Ivern"),
            Self::KALISTA => Some("Kalista"),
            Self::BARD => Some("Bard"),
            Self::RAKAN => Some("Rakan"),
            Self::XAYAH => Some("Xayah"),
            Self::ORNN => Some("Ornn"),
            Self::SYLAS => Some("Sylas"),
            Self::NEEKO => Some("Neeko"),
            Self::APHELIOS => Some("Aphelios"),
            Self::RELL => Some("Rell"),
            Self::PYKE => Some("Pyke"),
            Self::VEX => Some("Vex"),
            Self::YONE => Some("Yone"),
            Self::AMBESSA => Some("Ambessa"),
            Self::MEL => Some("Mel"),
            Self::SETT => Some("Sett"),
            Self::LILLIA => Some("Lillia"),
            Self::GWEN => Some("Gwen"),
            Self::RENATA => Some("Renata Glasc"),
            Self::AURORA => Some("Aurora"),
            Self::NILAH => Some("Nilah"),
            Self::K_SANTE => Some("K'Sante"),
            Self::SMOLDER => Some("Smolder"),
            Self::MILIO => Some("Milio"),
            Self::HWEI => Some("Hwei"),
            Self::NAAFIRI => Some("Naafiri"),
            _ => None,
        }
    }
    /// The champion's identifier key. Somtimes called "key", "identifier", or "alias".
    /// This is mainly used in DDragon paths.
    ///
    /// This is generally the `en_US` name with spaces and punctuation removed,
    /// capitalization preserved, however the follow are exceptions:
    ///
    /// Field | Name | Identifier | Id
    /// ---|---|---|---
    ///`1` | Annie | Annie | 1
    ///`2` | Olaf | Olaf | 2
    ///`3` | Galio | Galio | 3
    ///`4` | Twisted Fate | TwistedFate | 4
    ///`5` | Xin Zhao | XinZhao | 5
    ///`6` | Urgot | Urgot | 6
    ///`7` | LeBlanc | Leblanc | 7
    ///`8` | Vladimir | Vladimir | 8
    ///`9` | Fiddlesticks | FiddleSticks | 9
    ///`10` | Kayle | Kayle | 10
    ///`11` | Master Yi | MasterYi | 11
    ///`12` | Alistar | Alistar | 12
    ///`13` | Ryze | Ryze | 13
    ///`14` | Sion | Sion | 14
    ///`15` | Sivir | Sivir | 15
    ///`16` | Soraka | Soraka | 16
    ///`17` | Teemo | Teemo | 17
    ///`18` | Tristana | Tristana | 18
    ///`19` | Warwick | Warwick | 19
    ///`20` | Nunu & Willump | Nunu | 20
    ///`21` | Miss Fortune | MissFortune | 21
    ///`22` | Ashe | Ashe | 22
    ///`23` | Tryndamere | Tryndamere | 23
    ///`24` | Jax | Jax | 24
    ///`25` | Morgana | Morgana | 25
    ///`26` | Zilean | Zilean | 26
    ///`27` | Singed | Singed | 27
    ///`28` | Evelynn | Evelynn | 28
    ///`29` | Twitch | Twitch | 29
    ///`30` | Karthus | Karthus | 30
    ///`31` | Cho'Gath | Chogath | 31
    ///`32` | Amumu | Amumu | 32
    ///`33` | Rammus | Rammus | 33
    ///`34` | Anivia | Anivia | 34
    ///`35` | Shaco | Shaco | 35
    ///`36` | Dr. Mundo | DrMundo | 36
    ///`37` | Sona | Sona | 37
    ///`38` | Kassadin | Kassadin | 38
    ///`39` | Irelia | Irelia | 39
    ///`40` | Janna | Janna | 40
    ///`41` | Gangplank | Gangplank | 41
    ///`42` | Corki | Corki | 42
    ///`43` | Karma | Karma | 43
    ///`44` | Taric | Taric | 44
    ///`45` | Veigar | Veigar | 45
    ///`48` | Trundle | Trundle | 48
    ///`50` | Swain | Swain | 50
    ///`51` | Caitlyn | Caitlyn | 51
    ///`53` | Blitzcrank | Blitzcrank | 53
    ///`54` | Malphite | Malphite | 54
    ///`55` | Katarina | Katarina | 55
    ///`56` | Nocturne | Nocturne | 56
    ///`57` | Maokai | Maokai | 57
    ///`58` | Renekton | Renekton | 58
    ///`59` | Jarvan IV | JarvanIV | 59
    ///`60` | Elise | Elise | 60
    ///`61` | Orianna | Orianna | 61
    ///`62` | Wukong | MonkeyKing | 62
    ///`63` | Brand | Brand | 63
    ///`64` | Lee Sin | LeeSin | 64
    ///`67` | Vayne | Vayne | 67
    ///`68` | Rumble | Rumble | 68
    ///`69` | Cassiopeia | Cassiopeia | 69
    ///`72` | Skarner | Skarner | 72
    ///`74` | Heimerdinger | Heimerdinger | 74
    ///`75` | Nasus | Nasus | 75
    ///`76` | Nidalee | Nidalee | 76
    ///`77` | Udyr | Udyr | 77
    ///`78` | Poppy | Poppy | 78
    ///`79` | Gragas | Gragas | 79
    ///`80` | Pantheon | Pantheon | 80
    ///`81` | Ezreal | Ezreal | 81
    ///`82` | Mordekaiser | Mordekaiser | 82
    ///`83` | Yorick | Yorick | 83
    ///`84` | Akali | Akali | 84
    ///`85` | Kennen | Kennen | 85
    ///`86` | Garen | Garen | 86
    ///`89` | Leona | Leona | 89
    ///`90` | Malzahar | Malzahar | 90
    ///`91` | Talon | Talon | 91
    ///`92` | Riven | Riven | 92
    ///`96` | Kog'Maw | KogMaw | 96
    ///`98` | Shen | Shen | 98
    ///`99` | Lux | Lux | 99
    ///`101` | Xerath | Xerath | 101
    ///`102` | Shyvana | Shyvana | 102
    ///`103` | Ahri | Ahri | 103
    ///`104` | Graves | Graves | 104
    ///`105` | Fizz | Fizz | 105
    ///`106` | Volibear | Volibear | 106
    ///`107` | Rengar | Rengar | 107
    ///`110` | Varus | Varus | 110
    ///`111` | Nautilus | Nautilus | 111
    ///`112` | Viktor | Viktor | 112
    ///`113` | Sejuani | Sejuani | 113
    ///`114` | Fiora | Fiora | 114
    ///`115` | Ziggs | Ziggs | 115
    ///`117` | Lulu | Lulu | 117
    ///`119` | Draven | Draven | 119
    ///`120` | Hecarim | Hecarim | 120
    ///`121` | Kha'Zix | Khazix | 121
    ///`122` | Darius | Darius | 122
    ///`126` | Jayce | Jayce | 126
    ///`127` | Lissandra | Lissandra | 127
    ///`131` | Diana | Diana | 131
    ///`133` | Quinn | Quinn | 133
    ///`134` | Syndra | Syndra | 134
    ///`136` | Aurelion Sol | AurelionSol | 136
    ///`141` | Kayn | Kayn | 141
    ///`142` | Zoe | Zoe | 142
    ///`143` | Zyra | Zyra | 143
    ///`145` | Kai'Sa | Kaisa | 145
    ///`147` | Seraphine | Seraphine | 147
    ///`150` | Gnar | Gnar | 150
    ///`154` | Zac | Zac | 154
    ///`157` | Yasuo | Yasuo | 157
    ///`161` | Vel'Koz | Velkoz | 161
    ///`163` | Taliyah | Taliyah | 163
    ///`164` | Camille | Camille | 164
    ///`166` | Akshan | Akshan | 166
    ///`200` | Bel'Veth | Belveth | 200
    ///`201` | Braum | Braum | 201
    ///`202` | Jhin | Jhin | 202
    ///`203` | Kindred | Kindred | 203
    ///`221` | Zeri | Zeri | 221
    ///`222` | Jinx | Jinx | 222
    ///`223` | Tahm Kench | TahmKench | 223
    ///`233` | Briar | Briar | 233
    ///`234` | Viego | Viego | 234
    ///`235` | Senna | Senna | 235
    ///`236` | Lucian | Lucian | 236
    ///`238` | Zed | Zed | 238
    ///`240` | Kled | Kled | 240
    ///`245` | Ekko | Ekko | 245
    ///`246` | Qiyana | Qiyana | 246
    ///`254` | Vi | Vi | 254
    ///`266` | Aatrox | Aatrox | 266
    ///`267` | Nami | Nami | 267
    ///`268` | Azir | Azir | 268
    ///`350` | Yuumi | Yuumi | 350
    ///`360` | Samira | Samira | 360
    ///`412` | Thresh | Thresh | 412
    ///`420` | Illaoi | Illaoi | 420
    ///`421` | Rek'Sai | RekSai | 421
    ///`427` | Ivern | Ivern | 427
    ///`429` | Kalista | Kalista | 429
    ///`432` | Bard | Bard | 432
    ///`497` | Rakan | Rakan | 497
    ///`498` | Xayah | Xayah | 498
    ///`516` | Ornn | Ornn | 516
    ///`517` | Sylas | Sylas | 517
    ///`518` | Neeko | Neeko | 518
    ///`523` | Aphelios | Aphelios | 523
    ///`526` | Rell | Rell | 526
    ///`555` | Pyke | Pyke | 555
    ///`711` | Vex | Vex | 711
    ///`777` | Yone | Yone | 777
    ///`799` | Ambessa | Ambessa | 799
    ///`800` | Mel | Mel | 800
    ///`875` | Sett | Sett | 875
    ///`876` | Lillia | Lillia | 876
    ///`887` | Gwen | Gwen | 887
    ///`888` | Renata Glasc | Renata | 888
    ///`893` | Aurora | Aurora | 893
    ///`895` | Nilah | Nilah | 895
    ///`897` | K'Sante | KSante | 897
    ///`901` | Smolder | Smolder | 901
    ///`902` | Milio | Milio | 902
    ///`910` | Hwei | Hwei | 910
    ///`950` | Naafiri | Naafiri | 950
    pub const fn identifier(self) -> Option<&'static str> {
        match self {
            Self::ANNIE => Some("Annie"),
            Self::OLAF => Some("Olaf"),
            Self::GALIO => Some("Galio"),
            Self::TWISTED_FATE => Some("TwistedFate"),
            Self::XIN_ZHAO => Some("XinZhao"),
            Self::URGOT => Some("Urgot"),
            Self::LEBLANC => Some("Leblanc"),
            Self::VLADIMIR => Some("Vladimir"),
            Self::FIDDLE_STICKS => Some("FiddleSticks"),
            Self::KAYLE => Some("Kayle"),
            Self::MASTER_YI => Some("MasterYi"),
            Self::ALISTAR => Some("Alistar"),
            Self::RYZE => Some("Ryze"),
            Self::SION => Some("Sion"),
            Self::SIVIR => Some("Sivir"),
            Self::SORAKA => Some("Soraka"),
            Self::TEEMO => Some("Teemo"),
            Self::TRISTANA => Some("Tristana"),
            Self::WARWICK => Some("Warwick"),
            Self::NUNU => Some("Nunu"),
            Self::MISS_FORTUNE => Some("MissFortune"),
            Self::ASHE => Some("Ashe"),
            Self::TRYNDAMERE => Some("Tryndamere"),
            Self::JAX => Some("Jax"),
            Self::MORGANA => Some("Morgana"),
            Self::ZILEAN => Some("Zilean"),
            Self::SINGED => Some("Singed"),
            Self::EVELYNN => Some("Evelynn"),
            Self::TWITCH => Some("Twitch"),
            Self::KARTHUS => Some("Karthus"),
            Self::CHOGATH => Some("Chogath"),
            Self::AMUMU => Some("Amumu"),
            Self::RAMMUS => Some("Rammus"),
            Self::ANIVIA => Some("Anivia"),
            Self::SHACO => Some("Shaco"),
            Self::DR_MUNDO => Some("DrMundo"),
            Self::SONA => Some("Sona"),
            Self::KASSADIN => Some("Kassadin"),
            Self::IRELIA => Some("Irelia"),
            Self::JANNA => Some("Janna"),
            Self::GANGPLANK => Some("Gangplank"),
            Self::CORKI => Some("Corki"),
            Self::KARMA => Some("Karma"),
            Self::TARIC => Some("Taric"),
            Self::VEIGAR => Some("Veigar"),
            Self::TRUNDLE => Some("Trundle"),
            Self::SWAIN => Some("Swain"),
            Self::CAITLYN => Some("Caitlyn"),
            Self::BLITZCRANK => Some("Blitzcrank"),
            Self::MALPHITE => Some("Malphite"),
            Self::KATARINA => Some("Katarina"),
            Self::NOCTURNE => Some("Nocturne"),
            Self::MAOKAI => Some("Maokai"),
            Self::RENEKTON => Some("Renekton"),
            Self::JARVAN_IV => Some("JarvanIV"),
            Self::ELISE => Some("Elise"),
            Self::ORIANNA => Some("Orianna"),
            Self::MONKEY_KING => Some("MonkeyKing"),
            Self::BRAND => Some("Brand"),
            Self::LEE_SIN => Some("LeeSin"),
            Self::VAYNE => Some("Vayne"),
            Self::RUMBLE => Some("Rumble"),
            Self::CASSIOPEIA => Some("Cassiopeia"),
            Self::SKARNER => Some("Skarner"),
            Self::HEIMERDINGER => Some("Heimerdinger"),
            Self::NASUS => Some("Nasus"),
            Self::NIDALEE => Some("Nidalee"),
            Self::UDYR => Some("Udyr"),
            Self::POPPY => Some("Poppy"),
            Self::GRAGAS => Some("Gragas"),
            Self::PANTHEON => Some("Pantheon"),
            Self::EZREAL => Some("Ezreal"),
            Self::MORDEKAISER => Some("Mordekaiser"),
            Self::YORICK => Some("Yorick"),
            Self::AKALI => Some("Akali"),
            Self::KENNEN => Some("Kennen"),
            Self::GAREN => Some("Garen"),
            Self::LEONA => Some("Leona"),
            Self::MALZAHAR => Some("Malzahar"),
            Self::TALON => Some("Talon"),
            Self::RIVEN => Some("Riven"),
            Self::KOG_MAW => Some("KogMaw"),
            Self::SHEN => Some("Shen"),
            Self::LUX => Some("Lux"),
            Self::XERATH => Some("Xerath"),
            Self::SHYVANA => Some("Shyvana"),
            Self::AHRI => Some("Ahri"),
            Self::GRAVES => Some("Graves"),
            Self::FIZZ => Some("Fizz"),
            Self::VOLIBEAR => Some("Volibear"),
            Self::RENGAR => Some("Rengar"),
            Self::VARUS => Some("Varus"),
            Self::NAUTILUS => Some("Nautilus"),
            Self::VIKTOR => Some("Viktor"),
            Self::SEJUANI => Some("Sejuani"),
            Self::FIORA => Some("Fiora"),
            Self::ZIGGS => Some("Ziggs"),
            Self::LULU => Some("Lulu"),
            Self::DRAVEN => Some("Draven"),
            Self::HECARIM => Some("Hecarim"),
            Self::KHAZIX => Some("Khazix"),
            Self::DARIUS => Some("Darius"),
            Self::JAYCE => Some("Jayce"),
            Self::LISSANDRA => Some("Lissandra"),
            Self::DIANA => Some("Diana"),
            Self::QUINN => Some("Quinn"),
            Self::SYNDRA => Some("Syndra"),
            Self::AURELION_SOL => Some("AurelionSol"),
            Self::KAYN => Some("Kayn"),
            Self::ZOE => Some("Zoe"),
            Self::ZYRA => Some("Zyra"),
            Self::KAISA => Some("Kaisa"),
            Self::SERAPHINE => Some("Seraphine"),
            Self::GNAR => Some("Gnar"),
            Self::ZAC => Some("Zac"),
            Self::YASUO => Some("Yasuo"),
            Self::VELKOZ => Some("Velkoz"),
            Self::TALIYAH => Some("Taliyah"),
            Self::CAMILLE => Some("Camille"),
            Self::AKSHAN => Some("Akshan"),
            Self::BELVETH => Some("Belveth"),
            Self::BRAUM => Some("Braum"),
            Self::JHIN => Some("Jhin"),
            Self::KINDRED => Some("Kindred"),
            Self::ZERI => Some("Zeri"),
            Self::JINX => Some("Jinx"),
            Self::TAHM_KENCH => Some("TahmKench"),
            Self::BRIAR => Some("Briar"),
            Self::VIEGO => Some("Viego"),
            Self::SENNA => Some("Senna"),
            Self::LUCIAN => Some("Lucian"),
            Self::ZED => Some("Zed"),
            Self::KLED => Some("Kled"),
            Self::EKKO => Some("Ekko"),
            Self::QIYANA => Some("Qiyana"),
            Self::VI => Some("Vi"),
            Self::AATROX => Some("Aatrox"),
            Self::NAMI => Some("Nami"),
            Self::AZIR => Some("Azir"),
            Self::YUUMI => Some("Yuumi"),
            Self::SAMIRA => Some("Samira"),
            Self::THRESH => Some("Thresh"),
            Self::ILLAOI => Some("Illaoi"),
            Self::REK_SAI => Some("RekSai"),
            Self::IVERN => Some("Ivern"),
            Self::KALISTA => Some("Kalista"),
            Self::BARD => Some("Bard"),
            Self::RAKAN => Some("Rakan"),
            Self::XAYAH => Some("Xayah"),
            Self::ORNN => Some("Ornn"),
            Self::SYLAS => Some("Sylas"),
            Self::NEEKO => Some("Neeko"),
            Self::APHELIOS => Some("Aphelios"),
            Self::RELL => Some("Rell"),
            Self::PYKE => Some("Pyke"),
            Self::VEX => Some("Vex"),
            Self::YONE => Some("Yone"),
            Self::AMBESSA => Some("Ambessa"),
            Self::MEL => Some("Mel"),
            Self::SETT => Some("Sett"),
            Self::LILLIA => Some("Lillia"),
            Self::GWEN => Some("Gwen"),
            Self::RENATA => Some("Renata"),
            Self::AURORA => Some("Aurora"),
            Self::NILAH => Some("Nilah"),
            Self::K_SANTE => Some("KSante"),
            Self::SMOLDER => Some("Smolder"),
            Self::MILIO => Some("Milio"),
            Self::HWEI => Some("Hwei"),
            Self::NAAFIRI => Some("Naafiri"),
            _ => None,
        }
    }
}
impl std::str::FromStr for Champion {
    type Err = ParseChampionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = ['\0'; 4];
        s.chars()
            .take(4)
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_uppercase())
            .enumerate()
            .for_each(|(i, c)| chars[i] = c);
        match chars {
            ['A', 'A', 'T', 'R'] => Ok(Champion::AATROX),
            ['A', 'H', 'R', 'I'] => Ok(Champion::AHRI),
            ['A', 'K', 'A', 'L'] => Ok(Champion::AKALI),
            ['A', 'K', 'S', 'H'] => Ok(Champion::AKSHAN),
            ['A', 'L', 'I', 'S'] => Ok(Champion::ALISTAR),
            ['A', 'M', 'B', 'E'] => Ok(Champion::AMBESSA),
            ['A', 'M', 'U', 'M'] => Ok(Champion::AMUMU),
            ['A', 'N', 'I', 'V'] => Ok(Champion::ANIVIA),
            ['A', 'N', 'N', 'I'] => Ok(Champion::ANNIE),
            ['A', 'P', 'H', 'E'] => Ok(Champion::APHELIOS),
            ['A', 'S', 'H', 'E'] => Ok(Champion::ASHE),
            ['A', 'U', 'R', 'E'] => Ok(Champion::AURELION_SOL),
            ['A', 'U', 'R', 'O'] => Ok(Champion::AURORA),
            ['A', 'Z', 'I', 'R'] => Ok(Champion::AZIR),
            ['B', 'A', 'R', 'D'] => Ok(Champion::BARD),
            ['B', 'E', 'L', '\0'] => Ok(Champion::BELVETH),
            ['B', 'E', 'L', 'V'] => Ok(Champion::BELVETH),
            ['B', 'L', 'I', 'T'] => Ok(Champion::BLITZCRANK),
            ['B', 'R', 'A', 'N'] => Ok(Champion::BRAND),
            ['B', 'R', 'A', 'U'] => Ok(Champion::BRAUM),
            ['B', 'R', 'I', 'A'] => Ok(Champion::BRIAR),
            ['C', 'A', 'I', 'T'] => Ok(Champion::CAITLYN),
            ['C', 'A', 'M', 'I'] => Ok(Champion::CAMILLE),
            ['C', 'A', 'S', 'S'] => Ok(Champion::CASSIOPEIA),
            ['C', 'H', 'O', '\0'] => Ok(Champion::CHOGATH),
            ['C', 'H', 'O', 'G'] => Ok(Champion::CHOGATH),
            ['C', 'O', 'R', 'K'] => Ok(Champion::CORKI),
            ['D', 'A', 'R', 'I'] => Ok(Champion::DARIUS),
            ['D', 'I', 'A', 'N'] => Ok(Champion::DIANA),
            ['D', 'R', '\0', '\0'] => Ok(Champion::DR_MUNDO),
            ['D', 'R', 'A', 'V'] => Ok(Champion::DRAVEN),
            ['D', 'R', 'M', 'U'] => Ok(Champion::DR_MUNDO),
            ['E', 'K', 'K', 'O'] => Ok(Champion::EKKO),
            ['E', 'L', 'I', 'S'] => Ok(Champion::ELISE),
            ['E', 'V', 'E', 'L'] => Ok(Champion::EVELYNN),
            ['E', 'Z', 'R', 'E'] => Ok(Champion::EZREAL),
            ['F', 'I', 'D', 'D'] => Ok(Champion::FIDDLE_STICKS),
            ['F', 'I', 'O', 'R'] => Ok(Champion::FIORA),
            ['F', 'I', 'Z', 'Z'] => Ok(Champion::FIZZ),
            ['G', 'A', 'L', 'I'] => Ok(Champion::GALIO),
            ['G', 'A', 'N', 'G'] => Ok(Champion::GANGPLANK),
            ['G', 'A', 'R', 'E'] => Ok(Champion::GAREN),
            ['G', 'N', 'A', 'R'] => Ok(Champion::GNAR),
            ['G', 'R', 'A', 'G'] => Ok(Champion::GRAGAS),
            ['G', 'R', 'A', 'V'] => Ok(Champion::GRAVES),
            ['G', 'W', 'E', 'N'] => Ok(Champion::GWEN),
            ['H', 'E', 'C', 'A'] => Ok(Champion::HECARIM),
            ['H', 'E', 'I', 'M'] => Ok(Champion::HEIMERDINGER),
            ['H', 'W', 'E', 'I'] => Ok(Champion::HWEI),
            ['I', 'L', 'L', 'A'] => Ok(Champion::ILLAOI),
            ['I', 'R', 'E', 'L'] => Ok(Champion::IRELIA),
            ['I', 'V', 'E', 'R'] => Ok(Champion::IVERN),
            ['J', 'A', 'N', 'N'] => Ok(Champion::JANNA),
            ['J', 'A', 'R', 'V'] => Ok(Champion::JARVAN_IV),
            ['J', 'A', 'X', '\0'] => Ok(Champion::JAX),
            ['J', 'A', 'Y', 'C'] => Ok(Champion::JAYCE),
            ['J', 'H', 'I', 'N'] => Ok(Champion::JHIN),
            ['J', 'I', 'N', 'X'] => Ok(Champion::JINX),
            ['K', '\0', '\0', '\0'] => Ok(Champion::K_SANTE),
            ['K', 'A', 'I', '\0'] => Ok(Champion::KAISA),
            ['K', 'A', 'I', 'S'] => Ok(Champion::KAISA),
            ['K', 'A', 'L', 'I'] => Ok(Champion::KALISTA),
            ['K', 'A', 'R', 'M'] => Ok(Champion::KARMA),
            ['K', 'A', 'R', 'T'] => Ok(Champion::KARTHUS),
            ['K', 'A', 'S', 'S'] => Ok(Champion::KASSADIN),
            ['K', 'A', 'T', 'A'] => Ok(Champion::KATARINA),
            ['K', 'A', 'Y', 'L'] => Ok(Champion::KAYLE),
            ['K', 'A', 'Y', 'N'] => Ok(Champion::KAYN),
            ['K', 'E', 'N', 'N'] => Ok(Champion::KENNEN),
            ['K', 'H', 'A', '\0'] => Ok(Champion::KHAZIX),
            ['K', 'H', 'A', 'Z'] => Ok(Champion::KHAZIX),
            ['K', 'I', 'N', 'D'] => Ok(Champion::KINDRED),
            ['K', 'L', 'E', 'D'] => Ok(Champion::KLED),
            ['K', 'O', 'G', '\0'] => Ok(Champion::KOG_MAW),
            ['K', 'O', 'G', 'M'] => Ok(Champion::KOG_MAW),
            ['K', 'S', 'A', 'N'] => Ok(Champion::K_SANTE),
            ['L', 'E', 'B', 'L'] => Ok(Champion::LEBLANC),
            ['L', 'E', 'E', '\0'] => Ok(Champion::LEE_SIN),
            ['L', 'E', 'E', 'S'] => Ok(Champion::LEE_SIN),
            ['L', 'E', 'O', 'N'] => Ok(Champion::LEONA),
            ['L', 'I', 'L', 'L'] => Ok(Champion::LILLIA),
            ['L', 'I', 'S', 'S'] => Ok(Champion::LISSANDRA),
            ['L', 'U', 'C', 'I'] => Ok(Champion::LUCIAN),
            ['L', 'U', 'L', 'U'] => Ok(Champion::LULU),
            ['L', 'U', 'X', '\0'] => Ok(Champion::LUX),
            ['M', 'A', 'L', 'P'] => Ok(Champion::MALPHITE),
            ['M', 'A', 'L', 'Z'] => Ok(Champion::MALZAHAR),
            ['M', 'A', 'O', 'K'] => Ok(Champion::MAOKAI),
            ['M', 'A', 'S', 'T'] => Ok(Champion::MASTER_YI),
            ['M', 'E', 'L', '\0'] => Ok(Champion::MEL),
            ['M', 'I', 'L', 'I'] => Ok(Champion::MILIO),
            ['M', 'I', 'S', 'S'] => Ok(Champion::MISS_FORTUNE),
            ['M', 'O', 'N', 'K'] => Ok(Champion::MONKEY_KING),
            ['M', 'O', 'R', 'D'] => Ok(Champion::MORDEKAISER),
            ['M', 'O', 'R', 'G'] => Ok(Champion::MORGANA),
            ['N', 'A', 'A', 'F'] => Ok(Champion::NAAFIRI),
            ['N', 'A', 'M', 'I'] => Ok(Champion::NAMI),
            ['N', 'A', 'S', 'U'] => Ok(Champion::NASUS),
            ['N', 'A', 'U', 'T'] => Ok(Champion::NAUTILUS),
            ['N', 'E', 'E', 'K'] => Ok(Champion::NEEKO),
            ['N', 'I', 'D', 'A'] => Ok(Champion::NIDALEE),
            ['N', 'I', 'L', 'A'] => Ok(Champion::NILAH),
            ['N', 'O', 'C', 'T'] => Ok(Champion::NOCTURNE),
            ['N', 'U', 'N', 'U'] => Ok(Champion::NUNU),
            ['O', 'L', 'A', 'F'] => Ok(Champion::OLAF),
            ['O', 'R', 'I', 'A'] => Ok(Champion::ORIANNA),
            ['O', 'R', 'N', 'N'] => Ok(Champion::ORNN),
            ['P', 'A', 'N', 'T'] => Ok(Champion::PANTHEON),
            ['P', 'O', 'P', 'P'] => Ok(Champion::POPPY),
            ['P', 'Y', 'K', 'E'] => Ok(Champion::PYKE),
            ['Q', 'I', 'Y', 'A'] => Ok(Champion::QIYANA),
            ['Q', 'U', 'I', 'N'] => Ok(Champion::QUINN),
            ['R', 'A', 'K', 'A'] => Ok(Champion::RAKAN),
            ['R', 'A', 'M', 'M'] => Ok(Champion::RAMMUS),
            ['R', 'E', 'K', '\0'] => Ok(Champion::REK_SAI),
            ['R', 'E', 'K', 'S'] => Ok(Champion::REK_SAI),
            ['R', 'E', 'L', 'L'] => Ok(Champion::RELL),
            ['R', 'E', 'N', 'A'] => Ok(Champion::RENATA),
            ['R', 'E', 'N', 'E'] => Ok(Champion::RENEKTON),
            ['R', 'E', 'N', 'G'] => Ok(Champion::RENGAR),
            ['R', 'I', 'V', 'E'] => Ok(Champion::RIVEN),
            ['R', 'U', 'M', 'B'] => Ok(Champion::RUMBLE),
            ['R', 'Y', 'Z', 'E'] => Ok(Champion::RYZE),
            ['S', 'A', 'M', 'I'] => Ok(Champion::SAMIRA),
            ['S', 'E', 'J', 'U'] => Ok(Champion::SEJUANI),
            ['S', 'E', 'N', 'N'] => Ok(Champion::SENNA),
            ['S', 'E', 'R', 'A'] => Ok(Champion::SERAPHINE),
            ['S', 'E', 'T', 'T'] => Ok(Champion::SETT),
            ['S', 'H', 'A', 'C'] => Ok(Champion::SHACO),
            ['S', 'H', 'E', 'N'] => Ok(Champion::SHEN),
            ['S', 'H', 'Y', 'V'] => Ok(Champion::SHYVANA),
            ['S', 'I', 'N', 'G'] => Ok(Champion::SINGED),
            ['S', 'I', 'O', 'N'] => Ok(Champion::SION),
            ['S', 'I', 'V', 'I'] => Ok(Champion::SIVIR),
            ['S', 'K', 'A', 'R'] => Ok(Champion::SKARNER),
            ['S', 'M', 'O', 'L'] => Ok(Champion::SMOLDER),
            ['S', 'O', 'N', 'A'] => Ok(Champion::SONA),
            ['S', 'O', 'R', 'A'] => Ok(Champion::SORAKA),
            ['S', 'W', 'A', 'I'] => Ok(Champion::SWAIN),
            ['S', 'Y', 'L', 'A'] => Ok(Champion::SYLAS),
            ['S', 'Y', 'N', 'D'] => Ok(Champion::SYNDRA),
            ['T', 'A', 'H', 'M'] => Ok(Champion::TAHM_KENCH),
            ['T', 'A', 'L', 'I'] => Ok(Champion::TALIYAH),
            ['T', 'A', 'L', 'O'] => Ok(Champion::TALON),
            ['T', 'A', 'R', 'I'] => Ok(Champion::TARIC),
            ['T', 'E', 'E', 'M'] => Ok(Champion::TEEMO),
            ['T', 'H', 'R', 'E'] => Ok(Champion::THRESH),
            ['T', 'R', 'I', 'S'] => Ok(Champion::TRISTANA),
            ['T', 'R', 'U', 'N'] => Ok(Champion::TRUNDLE),
            ['T', 'R', 'Y', 'N'] => Ok(Champion::TRYNDAMERE),
            ['T', 'W', 'I', 'S'] => Ok(Champion::TWISTED_FATE),
            ['T', 'W', 'I', 'T'] => Ok(Champion::TWITCH),
            ['U', 'D', 'Y', 'R'] => Ok(Champion::UDYR),
            ['U', 'R', 'G', 'O'] => Ok(Champion::URGOT),
            ['V', 'A', 'R', 'U'] => Ok(Champion::VARUS),
            ['V', 'A', 'Y', 'N'] => Ok(Champion::VAYNE),
            ['V', 'E', 'I', 'G'] => Ok(Champion::VEIGAR),
            ['V', 'E', 'L', '\0'] => Ok(Champion::VELKOZ),
            ['V', 'E', 'L', 'K'] => Ok(Champion::VELKOZ),
            ['V', 'E', 'X', '\0'] => Ok(Champion::VEX),
            ['V', 'I', '\0', '\0'] => Ok(Champion::VI),
            ['V', 'I', 'E', 'G'] => Ok(Champion::VIEGO),
            ['V', 'I', 'K', 'T'] => Ok(Champion::VIKTOR),
            ['V', 'L', 'A', 'D'] => Ok(Champion::VLADIMIR),
            ['V', 'O', 'L', 'I'] => Ok(Champion::VOLIBEAR),
            ['W', 'A', 'R', 'W'] => Ok(Champion::WARWICK),
            ['W', 'U', 'K', 'O'] => Ok(Champion::MONKEY_KING),
            ['X', 'A', 'Y', 'A'] => Ok(Champion::XAYAH),
            ['X', 'E', 'R', 'A'] => Ok(Champion::XERATH),
            ['X', 'I', 'N', '\0'] => Ok(Champion::XIN_ZHAO),
            ['X', 'I', 'N', 'Z'] => Ok(Champion::XIN_ZHAO),
            ['Y', 'A', 'S', 'U'] => Ok(Champion::YASUO),
            ['Y', 'O', 'N', 'E'] => Ok(Champion::YONE),
            ['Y', 'O', 'R', 'I'] => Ok(Champion::YORICK),
            ['Y', 'U', 'U', 'M'] => Ok(Champion::YUUMI),
            ['Z', 'A', 'C', '\0'] => Ok(Champion::ZAC),
            ['Z', 'E', 'D', '\0'] => Ok(Champion::ZED),
            ['Z', 'E', 'R', 'I'] => Ok(Champion::ZERI),
            ['Z', 'I', 'G', 'G'] => Ok(Champion::ZIGGS),
            ['Z', 'I', 'L', 'E'] => Ok(Champion::ZILEAN),
            ['Z', 'O', 'E', '\0'] => Ok(Champion::ZOE),
            ['Z', 'Y', 'R', 'A'] => Ok(Champion::ZYRA),
            unknown => Err(ParseChampionError(unknown)),
        }
    }
}

