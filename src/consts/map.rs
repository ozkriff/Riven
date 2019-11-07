﻿///////////////////////////////////////////////
//                                           //
//                     !                     //
//   This file is automatically generated!   //
//           Do not directly edit!           //
//                                           //
///////////////////////////////////////////////

use serde_repr::{ Serialize_repr, Deserialize_repr };
use num_enum::{ IntoPrimitive, TryFromPrimitive };

/// League of Legends maps.
#[cfg_attr(feature = "nightly", non_exhaustive)]
#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq, Hash, PartialOrd, Ord)]
#[derive(Serialize_repr, Deserialize_repr)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Map {

    /// Summoner's Rift
    /// <br>Original Summer variant
    SUMMONERS_RIFT_ORIGINAL_SUMMER_VARIANT = 1,
    /// Summoner's Rift
    /// <br>Original Autumn variant
    SUMMONERS_RIFT_ORIGINAL_AUTUMN_VARIANT = 2,
    /// Summoner's Rift
    /// <br>Current Version
    SUMMONERS_RIFT = 11,

    /// The Proving Grounds
    /// <br>Tutorial Map
    THE_PROVING_GROUNDS = 3,

    /// Twisted Treeline
    /// <br>Original Version
    TWISTED_TREELINE_ORIGINAL_VERSION = 4,
    /// Twisted Treeline
    /// <br>Last TT map
    TWISTED_TREELINE = 10,

    /// The Crystal Scar
    /// <br>Dominion map
    THE_CRYSTAL_SCAR = 8,

    /// Howling Abyss
    /// <br>ARAM map
    HOWLING_ABYSS = 12,

    /// Butcher's Bridge
    /// <br>Alternate ARAM map
    BUTCHERS_BRIDGE = 14,

    /// Cosmic Ruins
    /// <br>Dark Star: Singularity map
    COSMIC_RUINS = 16,

    /// Valoran City Park
    /// <br>Star Guardian Invasion map
    VALORAN_CITY_PARK = 18,

    /// Substructure 43
    /// <br>PROJECT: Hunters map
    SUBSTRUCTURE43 = 19,

    /// Crash Site
    /// <br>Odyssey: Extraction map
    CRASH_SITE = 20,

    /// Nexus Blitz
    /// <br>Nexus Blitz map
    NEXUS_BLITZ = 21,
}
