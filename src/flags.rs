//! Flag definitions for Senzing SDK operations
//!
//! This module defines the bitflags used to control the behavior of various
//! Senzing SDK operations, mirroring the C# SDK's SzFlag enumeration.
//!
//! For comprehensive flag documentation and usage examples, see:
//! <https://www.senzing.com/docs/flags/4/>

use bitflags::bitflags;

bitflags! {
    /// Bitflags for controlling Senzing SDK operations
    ///
    /// For detailed flag descriptions and usage patterns, see the official documentation:
    /// <https://www.senzing.com/docs/flags/4/>
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct SzFlags: u64 {
        // Export flags
        const EXPORT_DEFAULT_FLAGS = 0;
        const EXPORT_INCLUDE_MULTI_RECORD_ENTITIES = 1 << 0;
        const EXPORT_INCLUDE_POSSIBLY_SAME = 1 << 1;
        const EXPORT_INCLUDE_POSSIBLY_RELATED = 1 << 2;
        const EXPORT_INCLUDE_NAME_ONLY = 1 << 3;
        const EXPORT_INCLUDE_DISCLOSED = 1 << 4;
        const EXPORT_INCLUDE_SINGLE_RECORD_ENTITIES = 1 << 5;
        const EXPORT_INCLUDE_ALL_ENTITIES = Self::EXPORT_INCLUDE_MULTI_RECORD_ENTITIES.bits() |
                                           Self::EXPORT_INCLUDE_SINGLE_RECORD_ENTITIES.bits();
        const EXPORT_INCLUDE_ALL_HAVING_RELATIONSHIPS = Self::EXPORT_INCLUDE_POSSIBLY_SAME.bits() |
                                                        Self::EXPORT_INCLUDE_POSSIBLY_RELATED.bits() |
                                                        Self::EXPORT_INCLUDE_NAME_ONLY.bits() |
                                                        Self::EXPORT_INCLUDE_DISCLOSED.bits();

        // Entity flags
        const ENTITY_DEFAULT_FLAGS = Self::ENTITY_INCLUDE_RECORD_SUMMARY.bits() |
                                     Self::ENTITY_INCLUDE_RECORD_DATA.bits() |
                                     Self::ENTITY_INCLUDE_RECORD_MATCHING_INFO.bits();
        const ENTITY_INCLUDE_POSSIBLY_SAME_RELATIONS = 1 << 6;
        const ENTITY_INCLUDE_POSSIBLY_RELATED_RELATIONS = 1 << 7;
        const ENTITY_INCLUDE_NAME_ONLY_RELATIONS = 1 << 8;
        const ENTITY_INCLUDE_DISCLOSED_RELATIONS = 1 << 9;
        const ENTITY_INCLUDE_ALL_RELATIONS = Self::ENTITY_INCLUDE_POSSIBLY_SAME_RELATIONS.bits() |
                                            Self::ENTITY_INCLUDE_POSSIBLY_RELATED_RELATIONS.bits() |
                                            Self::ENTITY_INCLUDE_NAME_ONLY_RELATIONS.bits() |
                                            Self::ENTITY_INCLUDE_DISCLOSED_RELATIONS.bits();
        const ENTITY_INCLUDE_ALL_FEATURES = 1 << 10;
        const ENTITY_INCLUDE_REPRESENTATIVE_FEATURES = 1 << 11;
        const ENTITY_INCLUDE_ENTITY_NAME = 1 << 12;
        const ENTITY_INCLUDE_RECORD_SUMMARY = 1 << 13;
        const ENTITY_INCLUDE_RECORD_DATA = 1 << 14;
        const ENTITY_INCLUDE_RECORD_MATCHING_INFO = 1 << 15;
        const ENTITY_INCLUDE_RECORD_JSON_DATA = 1 << 16;
        const ENTITY_INCLUDE_RECORD_UNMAPPED_DATA = 1 << 17;
        const ENTITY_INCLUDE_RECORD_FEATURE_IDS = 1 << 18;
        const ENTITY_INCLUDE_RELATED_ENTITY_NAME = 1 << 19;
        const ENTITY_INCLUDE_RELATED_MATCHING_INFO = 1 << 20;
        const ENTITY_INCLUDE_RELATED_RECORD_SUMMARY = 1 << 21;
        const ENTITY_INCLUDE_RELATED_RECORD_DATA = 1 << 22;

        // Record flags
        const RECORD_DEFAULT_FLAGS = 0;

        // Search flags
        const SEARCH_BY_ATTRIBUTES_ALL = Self::SEARCH_BY_ATTRIBUTES_STRONG.bits() |
                                        Self::SEARCH_BY_ATTRIBUTES_MINIMAL_STRONG.bits() |
                                        Self::SEARCH_BY_ATTRIBUTES_MINIMAL_ALL.bits();
        const SEARCH_BY_ATTRIBUTES_STRONG = 1 << 23;
        const SEARCH_BY_ATTRIBUTES_MINIMAL_STRONG = 1 << 24;
        const SEARCH_BY_ATTRIBUTES_MINIMAL_ALL = 1 << 25;
        const SEARCH_BY_ATTRIBUTES_DEFAULT_FLAGS = Self::SEARCH_BY_ATTRIBUTES_ALL.bits();

        // Find flags
        const FIND_PATH_DEFAULT_FLAGS = Self::FIND_PATH_INCLUDE_MATCHING_INFO.bits();
        const FIND_PATH_INCLUDE_MATCHING_INFO = 1 << 26;
        const FIND_NETWORK_DEFAULT_FLAGS = Self::FIND_NETWORK_INCLUDE_MATCHING_INFO.bits();
        const FIND_NETWORK_INCLUDE_MATCHING_INFO = 1 << 27;

        // Why flags
        const WHY_ENTITIES_DEFAULT_FLAGS = Self::WHY_ENTITY_INCLUDE_ENTITY_NAME.bits() |
                                          Self::WHY_ENTITY_INCLUDE_RECORD_SUMMARY.bits();
        const WHY_ENTITY_INCLUDE_POSSIBLY_SAME_RELATIONS = 1 << 28;
        const WHY_ENTITY_INCLUDE_POSSIBLY_RELATED_RELATIONS = 1 << 29;
        const WHY_ENTITY_INCLUDE_NAME_ONLY_RELATIONS = 1 << 30;
        const WHY_ENTITY_INCLUDE_DISCLOSED_RELATIONS = 1 << 31;
        const WHY_ENTITY_INCLUDE_ALL_RELATIONS = Self::WHY_ENTITY_INCLUDE_POSSIBLY_SAME_RELATIONS.bits() |
                                                Self::WHY_ENTITY_INCLUDE_POSSIBLY_RELATED_RELATIONS.bits() |
                                                Self::WHY_ENTITY_INCLUDE_NAME_ONLY_RELATIONS.bits() |
                                                Self::WHY_ENTITY_INCLUDE_DISCLOSED_RELATIONS.bits();
        const WHY_ENTITY_INCLUDE_ALL_FEATURES = 1 << 32;
        const WHY_ENTITY_INCLUDE_REPRESENTATIVE_FEATURES = 1 << 33;
        const WHY_ENTITY_INCLUDE_ENTITY_NAME = 1 << 34;
        const WHY_ENTITY_INCLUDE_RECORD_SUMMARY = 1 << 35;
        const WHY_ENTITY_INCLUDE_RECORD_DATA = 1 << 36;
        const WHY_ENTITY_INCLUDE_RECORD_MATCHING_INFO = 1 << 37;
        const WHY_ENTITY_INCLUDE_RECORD_JSON_DATA = 1 << 38;
        const WHY_ENTITY_INCLUDE_RECORD_UNMAPPED_DATA = 1 << 39;
        const WHY_ENTITY_INCLUDE_RECORD_FEATURE_IDS = 1 << 40;
        const WHY_ENTITY_INCLUDE_RELATED_ENTITY_NAME = 1 << 41;
        const WHY_ENTITY_INCLUDE_RELATED_MATCHING_INFO = 1 << 42;
        const WHY_ENTITY_INCLUDE_RELATED_RECORD_SUMMARY = 1 << 43;
        const WHY_ENTITY_INCLUDE_RELATED_RECORD_DATA = 1 << 44;

        // Virtual entity flags
        const VIRTUAL_ENTITY_DEFAULT_FLAGS = Self::ENTITY_INCLUDE_RECORD_SUMMARY.bits();

        // How flags
        const HOW_ENTITY_DEFAULT_FLAGS = Self::HOW_ENTITY_INCLUDE_NAME.bits();
        const HOW_ENTITY_INCLUDE_NAME = 1 << 45;
    }
}

impl Default for SzFlags {
    fn default() -> Self {
        SzFlags::empty()
    }
}

impl SzFlags {
    /// Default flags for adding records
    pub const ADD_RECORD_DEFAULT: SzFlags = SzFlags::empty();

    /// Default flags for deleting records
    pub const DELETE_RECORD_DEFAULT: SzFlags = SzFlags::empty();

    /// Default flags for reevaluating records
    pub const REEVALUATE_RECORD_DEFAULT: SzFlags = SzFlags::empty();

    /// Default flags for reevaluating entities
    pub const REEVALUATE_ENTITY_DEFAULT: SzFlags = SzFlags::empty();

    /// Default flags for getting entities
    pub const GET_ENTITY_DEFAULT: SzFlags = SzFlags::ENTITY_DEFAULT_FLAGS;

    /// Default flags for getting records
    pub const GET_RECORD_DEFAULT: SzFlags = SzFlags::RECORD_DEFAULT_FLAGS;

    /// Default flags for searching by attributes
    pub const SEARCH_BY_ATTRIBUTES_DEFAULT: SzFlags = SzFlags::SEARCH_BY_ATTRIBUTES_DEFAULT_FLAGS;

    /// Default flags for finding interesting entities
    pub const FIND_INTERESTING_ENTITIES_DEFAULT: SzFlags = SzFlags::ENTITY_DEFAULT_FLAGS;

    /// Default flags for finding paths
    pub const FIND_PATH_DEFAULT: SzFlags = SzFlags::FIND_PATH_DEFAULT_FLAGS;

    /// Default flags for finding networks
    pub const FIND_NETWORK_DEFAULT: SzFlags = SzFlags::FIND_NETWORK_DEFAULT_FLAGS;

    /// Default flags for why entity operations
    pub const WHY_ENTITY_DEFAULT: SzFlags = SzFlags::WHY_ENTITIES_DEFAULT_FLAGS;

    /// Default flags for why record operations
    pub const WHY_RECORD_DEFAULT: SzFlags = SzFlags::WHY_ENTITIES_DEFAULT_FLAGS;

    /// Default flags for why search operations
    pub const WHY_SEARCH_DEFAULT: SzFlags = SzFlags::WHY_ENTITIES_DEFAULT_FLAGS;

    /// Default flags for how entity operations
    pub const HOW_ENTITY_DEFAULT: SzFlags = SzFlags::HOW_ENTITY_DEFAULT_FLAGS;

    /// Default flags for virtual entity operations
    pub const GET_VIRTUAL_ENTITY_DEFAULT: SzFlags = SzFlags::VIRTUAL_ENTITY_DEFAULT_FLAGS;

    /// Converts optional flags to i64 for FFI calls
    ///
    /// This helper consolidates the common pattern of converting `Option<SzFlags>`
    /// to the i64 value expected by FFI functions.
    #[inline]
    pub fn to_ffi(flags: Option<SzFlags>, default: SzFlags) -> i64 {
        flags.unwrap_or(default).bits() as i64
    }
}
