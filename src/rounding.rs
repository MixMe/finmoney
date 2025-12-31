//! Rounding strategies for monetary calculations.

/// Rounding strategies for monetary operations.
///
/// These strategies determine how values are rounded when precision needs to be reduced,
/// such as during division operations or when converting to tick sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FinMoneyRoundingStrategy {
    /// When a number is halfway between two others, it is rounded toward the nearest even number.
    /// Also known as "Bankers Rounding".
    ///
    /// Examples:
    /// - 6.5 -> 6
    /// - 7.5 -> 8
    /// - -6.5 -> -6
    /// - -7.5 -> -8
    MidpointNearestEven,

    /// When a number is halfway between two others, it is rounded toward the nearest number that
    /// is away from zero.
    ///
    /// Examples:
    /// - 6.4 -> 6
    /// - 6.5 -> 7
    /// - -6.5 -> -7
    MidpointAwayFromZero,

    /// When a number is halfway between two others, it is rounded toward the nearest number that
    /// is toward zero.
    ///
    /// Examples:
    /// - 6.4 -> 6
    /// - 6.5 -> 6
    /// - -6.5 -> -6
    MidpointTowardZero,

    /// The number is always rounded toward zero.
    ///
    /// Examples:
    /// - -6.8 -> -6
    /// - 6.8 -> 6
    ToZero,

    /// The number is always rounded away from zero.
    ///
    /// Examples:
    /// - -6.8 -> -7
    /// - 6.8 -> 7
    AwayFromZero,

    /// The number is always rounded towards negative infinity (floor).
    ///
    /// Examples:
    /// - 6.8 -> 6
    /// - -6.8 -> -7
    ToNegativeInfinity,

    /// The number is always rounded towards positive infinity (ceiling).
    ///
    /// Examples:
    /// - 6.8 -> 7
    /// - -6.8 -> -6
    ToPositiveInfinity,
}

impl FinMoneyRoundingStrategy {
    /// Converts this rounding strategy to the corresponding `rust_decimal::RoundingStrategy`.
    #[inline]
    pub fn to_decimal_strategy(self) -> rust_decimal::RoundingStrategy {
        match self {
            FinMoneyRoundingStrategy::MidpointNearestEven => {
                rust_decimal::RoundingStrategy::MidpointNearestEven
            }
            FinMoneyRoundingStrategy::MidpointAwayFromZero => {
                rust_decimal::RoundingStrategy::MidpointAwayFromZero
            }
            FinMoneyRoundingStrategy::MidpointTowardZero => {
                rust_decimal::RoundingStrategy::MidpointTowardZero
            }
            FinMoneyRoundingStrategy::ToZero => rust_decimal::RoundingStrategy::ToZero,
            FinMoneyRoundingStrategy::AwayFromZero => rust_decimal::RoundingStrategy::AwayFromZero,
            FinMoneyRoundingStrategy::ToNegativeInfinity => {
                rust_decimal::RoundingStrategy::ToNegativeInfinity
            }
            FinMoneyRoundingStrategy::ToPositiveInfinity => {
                rust_decimal::RoundingStrategy::ToPositiveInfinity
            }
        }
    }
}

impl Default for FinMoneyRoundingStrategy {
    /// Returns the default rounding strategy: `MidpointNearestEven` (Banker's rounding).
    fn default() -> Self {
        FinMoneyRoundingStrategy::MidpointNearestEven
    }
}
