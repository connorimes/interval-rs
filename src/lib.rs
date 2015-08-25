//! This crate provides the `Interval` trait.

/// An `Interval` is for anything that has a minimum useful access interval.
///
/// Examples could include hardware sensors with known refresh intervals, or data sources that are
/// updated regularly.
///
/// This trait does not define the behavior if resources are accessed more or less frequently than
/// declared by implementors.
/// It also does not guarantee that the source will actually provide new data at each interval.
pub trait Interval {
    /// The `u64` return value is the interval in nanoseconds.
    fn interval(&self) -> u64;
}
