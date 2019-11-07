﻿///////////////////////////////////////////////
//                                           //
//                     !                     //
//   This file is automatically generated!   //
//           Do not directly edit!           //
//                                           //
///////////////////////////////////////////////

use serde_repr::{ Serialize_repr, Deserialize_repr };
use num_enum::{ IntoPrimitive, TryFromPrimitive };

/// League of Legends matchmaking queue.
#[cfg_attr(feature = "nightly", non_exhaustive)]
#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Serialize_repr, Deserialize_repr)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum Queue {

    /// Custom games.
    CUSTOM_GAMES_ = 0,

    /// 5v5 Blind Pick games games on Summoner's Rift.
    /// <br>Deprecated in patch 7.19 in favor of queueId 430
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 430")]
    SUMMONERS_RIFT_5V5_BLIND_PICK_GAMES_DEPRECATED = 2,
    /// 5v5 Blind Pick games games on Summoner's Rift.
    SUMMONERS_RIFT_5V5_BLIND_PICK_GAMES = 430,

    /// 5v5 Ranked Solo games games on Summoner's Rift.
    /// <br>Deprecated in favor of queueId 420
    #[deprecated(note="Deprecated in favor of queueId 420")]
    SUMMONERS_RIFT_5V5_RANKED_SOLO_GAMES_DEPRECATED = 4,
    /// 5v5 Ranked Solo games games on Summoner's Rift.
    SUMMONERS_RIFT_5V5_RANKED_SOLO_GAMES = 420,

    /// 5v5 Ranked Premade games games on Summoner's Rift.
    /// <br>Game mode deprecated
    #[deprecated(note="Game mode deprecated")]
    SUMMONERS_RIFT_5V5_RANKED_PREMADE_GAMES = 6,

    /// Co-op vs AI games games on Summoner's Rift.
    /// <br>Deprecated in favor of queueId 32 and 33
    #[deprecated(note="Deprecated in favor of queueId 32 and 33")]
    SUMMONERS_RIFT_CO_OP_VS_AI_GAMES = 7,

    /// 3v3 Normal games games on Twisted Treeline.
    /// <br>Deprecated in patch 7.19 in favor of queueId 460
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 460")]
    TWISTED_TREELINE_3V3_NORMAL_GAMES = 8,

    /// 3v3 Ranked Flex games games on Twisted Treeline.
    /// <br>Deprecated in patch 7.19 in favor of queueId 470
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 470")]
    TWISTED_TREELINE_3V3_RANKED_FLEX_GAMES_DEPRECATED = 9,
    /// 3v3 Ranked Flex games games on Twisted Treeline.
    TWISTED_TREELINE_3V3_RANKED_FLEX_GAMES = 470,

    /// 5v5 Draft Pick games games on Summoner's Rift.
    /// <br>Deprecated in favor of queueId 400
    #[deprecated(note="Deprecated in favor of queueId 400")]
    SUMMONERS_RIFT_5V5_DRAFT_PICK_GAMES_DEPRECATED = 14,
    /// 5v5 Draft Pick games games on Summoner's Rift.
    SUMMONERS_RIFT_5V5_DRAFT_PICK_GAMES = 400,

    /// 5v5 Dominion Blind Pick games games on Crystal Scar.
    /// <br>Game mode deprecated
    #[deprecated(note="Game mode deprecated")]
    CRYSTAL_SCAR_5V5_DOMINION_BLIND_PICK_GAMES = 16,

    /// 5v5 Dominion Draft Pick games games on Crystal Scar.
    /// <br>Game mode deprecated
    #[deprecated(note="Game mode deprecated")]
    CRYSTAL_SCAR_5V5_DOMINION_DRAFT_PICK_GAMES = 17,

    /// Dominion Co-op vs AI games games on Crystal Scar.
    /// <br>Game mode deprecated
    #[deprecated(note="Game mode deprecated")]
    CRYSTAL_SCAR_DOMINION_CO_OP_VS_AI_GAMES = 25,

    /// Co-op vs AI Intro Bot games games on Summoner's Rift.
    /// <br>Deprecated in patch 7.19 in favor of queueId 830
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 830")]
    SUMMONERS_RIFT_CO_OP_VS_AI_INTRO_BOT_GAMES_DEPRECATED = 31,
    /// Co-op vs. AI Intro Bot games games on Summoner's Rift.
    SUMMONERS_RIFT_CO_OP_VS_AI_INTRO_BOT_GAMES = 830,

    /// Co-op vs AI Beginner Bot games games on Summoner's Rift.
    /// <br>Deprecated in patch 7.19 in favor of queueId 840
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 840")]
    SUMMONERS_RIFT_CO_OP_VS_AI_BEGINNER_BOT_GAMES_DEPRECATED = 32,
    /// Co-op vs. AI Beginner Bot games games on Summoner's Rift.
    SUMMONERS_RIFT_CO_OP_VS_AI_BEGINNER_BOT_GAMES = 840,

    /// Co-op vs AI Intermediate Bot games games on Summoner's Rift.
    /// <br>Deprecated in patch 7.19 in favor of queueId 850
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 850")]
    SUMMONERS_RIFT_CO_OP_VS_AI_INTERMEDIATE_BOT_GAMES_DEPRECATED = 33,
    /// Co-op vs. AI Intermediate Bot games games on Summoner's Rift.
    SUMMONERS_RIFT_CO_OP_VS_AI_INTERMEDIATE_BOT_GAMES = 850,

    /// 3v3 Ranked Team games games on Twisted Treeline.
    /// <br>Game mode deprecated
    #[deprecated(note="Game mode deprecated")]
    TWISTED_TREELINE_3V3_RANKED_TEAM_GAMES = 41,

    /// 5v5 Ranked Team games games on Summoner's Rift.
    /// <br>Game mode deprecated
    #[deprecated(note="Game mode deprecated")]
    SUMMONERS_RIFT_5V5_RANKED_TEAM_GAMES = 42,

    /// Co-op vs AI games games on Twisted Treeline.
    /// <br>Deprecated in patch 7.19 in favor of queueId 800
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 800")]
    TWISTED_TREELINE_CO_OP_VS_AI_GAMES = 52,

    /// 5v5 Team Builder games games on Summoner's Rift.
    /// <br>Game mode deprecated
    #[deprecated(note="Game mode deprecated")]
    SUMMONERS_RIFT_5V5_TEAM_BUILDER_GAMES = 61,

    /// 5v5 ARAM games games on Howling Abyss.
    /// <br>Deprecated in patch 7.19 in favor of queueId 450
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 450")]
    HOWLING_ABYSS_5V5_ARAM_GAMES_DEPRECATED = 65,
    /// 5v5 ARAM games games on Howling Abyss.
    HOWLING_ABYSS_5V5_ARAM_GAMES = 450,

    /// ARAM Co-op vs AI games games on Howling Abyss.
    /// <br>Game mode deprecated
    #[deprecated(note="Game mode deprecated")]
    HOWLING_ABYSS_ARAM_CO_OP_VS_AI_GAMES = 67,

    /// One for All games games on Summoner's Rift.
    /// <br>Deprecated in patch 8.6 in favor of queueId 1020
    #[deprecated(note="Deprecated in patch 8.6 in favor of queueId 1020")]
    SUMMONERS_RIFT_ONE_FOR_ALL_GAMES_DEPRECATED = 70,
    /// One for All games games on Summoner's Rift.
    SUMMONERS_RIFT_ONE_FOR_ALL_GAMES = 1020,

    /// 1v1 Snowdown Showdown games games on Howling Abyss.
    HOWLING_ABYSS_1V1_SNOWDOWN_SHOWDOWN_GAMES = 72,

    /// 2v2 Snowdown Showdown games games on Howling Abyss.
    HOWLING_ABYSS_2V2_SNOWDOWN_SHOWDOWN_GAMES = 73,

    /// 6v6 Hexakill games games on Summoner's Rift.
    SUMMONERS_RIFT_6V6_HEXAKILL_GAMES = 75,

    /// Ultra Rapid Fire games games on Summoner's Rift.
    SUMMONERS_RIFT_ULTRA_RAPID_FIRE_GAMES = 76,

    /// One For All: Mirror Mode games games on Howling Abyss.
    HOWLING_ABYSS_ONE_FOR_ALL_MIRROR_MODE_GAMES = 78,

    /// Co-op vs AI Ultra Rapid Fire games games on Summoner's Rift.
    SUMMONERS_RIFT_CO_OP_VS_AI_ULTRA_RAPID_FIRE_GAMES = 83,

    /// Doom Bots Rank 1 games games on Summoner's Rift.
    /// <br>Deprecated in patch 7.19 in favor of queueId 950
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 950")]
    SUMMONERS_RIFT_DOOM_BOTS_RANK1_GAMES = 91,

    /// Doom Bots Rank 2 games games on Summoner's Rift.
    /// <br>Deprecated in patch 7.19 in favor of queueId 950
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 950")]
    SUMMONERS_RIFT_DOOM_BOTS_RANK2_GAMES = 92,

    /// Doom Bots Rank 5 games games on Summoner's Rift.
    /// <br>Deprecated in patch 7.19 in favor of queueId 950
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 950")]
    SUMMONERS_RIFT_DOOM_BOTS_RANK5_GAMES = 93,

    /// Ascension games games on Crystal Scar.
    /// <br>Deprecated in patch 7.19 in favor of queueId 910
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 910")]
    CRYSTAL_SCAR_ASCENSION_GAMES_DEPRECATED = 96,
    /// Ascension games games on Crystal Scar.
    CRYSTAL_SCAR_ASCENSION_GAMES = 910,

    /// 6v6 Hexakill games games on Twisted Treeline.
    TWISTED_TREELINE_6V6_HEXAKILL_GAMES = 98,

    /// 5v5 ARAM games games on Butcher's Bridge.
    BUTCHERS_BRIDGE_5V5_ARAM_GAMES = 100,

    /// Legend of the Poro King games games on Howling Abyss.
    /// <br>Deprecated in patch 7.19 in favor of queueId 920
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 920")]
    HOWLING_ABYSS_LEGEND_OF_THE_PORO_KING_GAMES_DEPRECATED = 300,
    /// Legend of the Poro King games games on Howling Abyss.
    HOWLING_ABYSS_LEGEND_OF_THE_PORO_KING_GAMES = 920,

    /// Nemesis games games on Summoner's Rift.
    SUMMONERS_RIFT_NEMESIS_GAMES = 310,

    /// Black Market Brawlers games games on Summoner's Rift.
    SUMMONERS_RIFT_BLACK_MARKET_BRAWLERS_GAMES = 313,

    /// Nexus Siege games games on Summoner's Rift.
    /// <br>Deprecated in patch 7.19 in favor of queueId 940
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 940")]
    SUMMONERS_RIFT_NEXUS_SIEGE_GAMES_DEPRECATED = 315,
    /// Nexus Siege games games on Summoner's Rift.
    SUMMONERS_RIFT_NEXUS_SIEGE_GAMES = 940,

    /// Definitely Not Dominion games games on Crystal Scar.
    CRYSTAL_SCAR_DEFINITELY_NOT_DOMINION_GAMES = 317,

    /// ARURF games games on Summoner's Rift.
    /// <br>Deprecated in patch 7.19 in favor of queueId 900
    #[deprecated(note="Deprecated in patch 7.19 in favor of queueId 900")]
    SUMMONERS_RIFT_ARURF_GAMES = 318,

    /// All Random games games on Summoner's Rift.
    SUMMONERS_RIFT_ALL_RANDOM_GAMES = 325,

    /// 5v5 Ranked Dynamic games games on Summoner's Rift.
    /// <br>Game mode deprecated in patch 6.22
    #[deprecated(note="Game mode deprecated in patch 6.22")]
    SUMMONERS_RIFT_5V5_RANKED_DYNAMIC_GAMES = 410,

    /// 5v5 Ranked Flex games games on Summoner's Rift.
    SUMMONERS_RIFT_5V5_RANKED_FLEX_GAMES = 440,

    /// 3v3 Blind Pick games games on Twisted Treeline.
    TWISTED_TREELINE_3V3_BLIND_PICK_GAMES = 460,

    /// Blood Hunt Assassin games games on Summoner's Rift.
    SUMMONERS_RIFT_BLOOD_HUNT_ASSASSIN_GAMES = 600,

    /// Dark Star: Singularity games games on Cosmic Ruins.
    COSMIC_RUINS_DARK_STAR_SINGULARITY_GAMES = 610,

    /// Clash games games on Summoner's Rift.
    SUMMONERS_RIFT_CLASH_GAMES = 700,

    /// Co-op vs. AI Intermediate Bot games games on Twisted Treeline.
    TWISTED_TREELINE_CO_OP_VS_AI_INTERMEDIATE_BOT_GAMES = 800,

    /// Co-op vs. AI Intro Bot games games on Twisted Treeline.
    TWISTED_TREELINE_CO_OP_VS_AI_INTRO_BOT_GAMES = 810,

    /// Co-op vs. AI Beginner Bot games games on Twisted Treeline.
    TWISTED_TREELINE_CO_OP_VS_AI_BEGINNER_BOT_GAMES = 820,

    /// URF games games on Summoner's Rift.
    SUMMONERS_RIFT_URF_GAMES = 900,

    /// Doom Bots Voting games games on Summoner's Rift.
    SUMMONERS_RIFT_DOOM_BOTS_VOTING_GAMES = 950,

    /// Doom Bots Standard games games on Summoner's Rift.
    SUMMONERS_RIFT_DOOM_BOTS_STANDARD_GAMES = 960,

    /// Star Guardian Invasion: Normal games games on Valoran City Park.
    VALORAN_CITY_PARK_STAR_GUARDIAN_INVASION_NORMAL_GAMES = 980,

    /// Star Guardian Invasion: Onslaught games games on Valoran City Park.
    VALORAN_CITY_PARK_STAR_GUARDIAN_INVASION_ONSLAUGHT_GAMES = 990,

    /// PROJECT: Hunters games games on Overcharge.
    OVERCHARGE_PROJECT_HUNTERS_GAMES = 1000,

    /// Snow ARURF games games on Summoner's Rift.
    SUMMONERS_RIFT_SNOW_ARURF_GAMES = 1010,

    /// Odyssey Extraction: Intro games games on Crash Site.
    CRASH_SITE_ODYSSEY_EXTRACTION_INTRO_GAMES = 1030,

    /// Odyssey Extraction: Cadet games games on Crash Site.
    CRASH_SITE_ODYSSEY_EXTRACTION_CADET_GAMES = 1040,

    /// Odyssey Extraction: Crewmember games games on Crash Site.
    CRASH_SITE_ODYSSEY_EXTRACTION_CREWMEMBER_GAMES = 1050,

    /// Odyssey Extraction: Captain games games on Crash Site.
    CRASH_SITE_ODYSSEY_EXTRACTION_CAPTAIN_GAMES = 1060,

    /// Odyssey Extraction: Onslaught games games on Crash Site.
    CRASH_SITE_ODYSSEY_EXTRACTION_ONSLAUGHT_GAMES = 1070,

    /// Teamfight Tactics games games on Convergence.
    CONVERGENCE_TEAMFIGHT_TACTICS_GAMES = 1090,

    /// Ranked Teamfight Tactics games games on Convergence.
    CONVERGENCE_RANKED_TEAMFIGHT_TACTICS_GAMES = 1100,

    /// Nexus Blitz games games on Nexus Blitz.
    /// <br>Deprecated in patch 9.2
    #[deprecated(note="Deprecated in patch 9.2")]
    NEXUS_BLITZ_NEXUS_BLITZ_GAMES = 1200,
}