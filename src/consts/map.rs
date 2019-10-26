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
#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq, Hash, PartialOrd, Ord)]
#[derive(Serialize_repr, Deserialize_repr)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Map {

    /// Summoner's Rift
    /// <br>Original Summer variant
    SummonersRiftOriginalSummerVariant = 1,
    /// Summoner's Rift
    /// <br>Original Autumn variant
    SummonersRiftOriginalAutumnVariant = 2,
    /// Summoner's Rift
    /// <br>Current Version
    SummonersRift = 11,

    /// The Proving Grounds
    /// <br>Tutorial Map
    TheProvingGrounds = 3,

    /// Twisted Treeline
    /// <br>Original Version
    TwistedTreelineOriginalVersion = 4,
    /// Twisted Treeline
    /// <br>Last TT map
    TwistedTreeline = 10,

    /// The Crystal Scar
    /// <br>Dominion map
    TheCrystalScar = 8,

    /// Howling Abyss
    /// <br>ARAM map
    HowlingAbyss = 12,

    /// Butcher's Bridge
    /// <br>Alternate ARAM map
    ButchersBridge = 14,

    /// Cosmic Ruins
    /// <br>Dark Star: Singularity map
    CosmicRuins = 16,

    /// Valoran City Park
    /// <br>Star Guardian Invasion map
    ValoranCityPark = 18,

    /// Substructure 43
    /// <br>PROJECT: Hunters map
    Substructure43 = 19,

    /// Crash Site
    /// <br>Odyssey: Extraction map
    CrashSite = 20,

    /// Nexus Blitz
    /// <br>Nexus Blitz map
    NexusBlitz = 21,
}
