use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::Bound;
use std::ops::Bound::*;
use std::rc::Rc;

/// A utility data structure to represent intervals.
/// It supports open, close and unbounded intervals
///
/// # Examples
/// ```
/// use rudac::util::Interval;
/// use std::ops::Bound::*;
///
/// // initialize interval [2,4]
/// let interval1 = Interval::new(Included(2), Included(4));
///
/// // initialize interval [2,4)
/// let interval2 = Interval::new(Included(2), Excluded(4));
///
/// // initialize point [4,4]
/// let point1 = Interval::point(4);
///
/// // compare intervals
/// // first, lower bounds are compared. if they're equal, higher bounds will be compared
/// assert!(interval2 < interval1);
///
/// // check if two intervals overlap
/// assert!(Interval::overlaps(&interval1, &interval2));
///
/// // check if one point and an interval overlap
/// assert!(Interval::overlaps(&interval1, &point1));
/// assert!(!Interval::overlaps(&interval2, &point1));
///
/// // check if one interval contains another interval
/// assert!(Interval::contains(&interval1, &interval2));
///
/// // get overlapped interval between two intervals
/// assert!(Interval::get_overlap(&interval1, &interval2).unwrap() == interval2);
/// ```
#[derive(Debug)]
pub struct Interval<T: Ord> {
    low: Rc<Bound<T>>,
    high: Rc<Bound<T>>,
}

impl<T: Ord> Interval<T> {
    /// Creates a new interval
    ///
    /// # Arguments
    /// * `low`: lower bound of the interval
    /// * `high`: higher bound of the interval
    ///
    /// # Panics
    /// * panics if `low` > `high`. `low` == `high` is acceptable if interval is closed at both sides: [low, high]
    ///
    /// # Example
    /// ```
    /// use rudac::util::Interval;
    /// use std::ops::Bound::*;
    ///
    /// // create the interval [2,4)
    /// let interval1 = Interval::new(Included(2), Excluded(4));
    ///
    /// // create the interval (-inf,4)
    /// let interval2 = Interval::new(Unbounded, Excluded(4));
    ///
    ///
    /// // create the interval (1,+inf)
    /// let interval3 = Interval::new(Excluded(1), Unbounded);
    /// ```
    pub fn new(low: Bound<T>, high: Bound<T>) -> Interval<T> {
        let interval = Interval {
            low: Rc::new(low),
            high: Rc::new(high),
        };

        if !Interval::valid(&interval) {
            panic!("Interval is not valid")
        }
        interval
    }

    /// Creates a point.
    ///
    /// # Arguments
    /// * `value`: value of the point
    ///
    /// # Examples
    /// ```
    /// use rudac::util::Interval;
    /// use std::ops::Bound::*;
    ///
    /// // create point (2) or equivalently interval [2,2]
    /// let point1 = Interval::point(2);
    /// ```
    pub fn point(value: T) -> Interval<T> {
        let low = Rc::new(Included(value));
        let high = Rc::clone(&low);

        let interval = Interval { low, high };

        if !Interval::valid(&interval) {
            panic!("Interval is not valid")
        }

        interval
    }

    /// Creates a duplicate of the interval
    ///
    /// # Examples
    /// ```
    /// use rudac::util::Interval;
    /// use std::ops::Bound::*;
    ///
    /// let interval = Interval::new(Included(2), Unbounded);
    /// let duplicate = interval.duplicate();
    ///
    /// assert!(interval == duplicate);
    /// ```
    pub fn duplicate(&self) -> Interval<T> {
        Interval {
            low: self.get_low(),
            high: self.get_high(),
        }
    }

    fn valid(interval: &Interval<T>) -> bool {
        match (&interval.low(), &interval.high()) {
            (Included(low), Included(high)) => low <= high,

            (Included(low), Excluded(high))
            | (Excluded(low), Included(high))
            | (Excluded(low), Excluded(high)) => low < high,

            _ => true,
        }
    }

    /// Get reference to lower bound of the interval
    pub fn low(&self) -> &Bound<T> {
        self.low.as_ref()
    }

    /// Get a duplicate of lower bound of the interval
    pub fn get_low(&self) -> Rc<Bound<T>> {
        Rc::clone(&self.low)
    }

    /// Get reference to higher bound of the interval
    pub fn high(&self) -> &Bound<T> {
        self.high.as_ref()
    }

    /// Get a duplicate of higher bound of the interval
    pub fn get_high(&self) -> Rc<Bound<T>> {
        Rc::clone(&self.high)
    }

    /// Returns true if `first` and `second` intervals overlap, false otherwise
    ///
    /// # Examples
    /// ```
    /// use rudac::util::Interval;
    /// use std::ops::Bound::*;
    ///
    /// let interval1 = Interval::new(Included(2), Included(4));
    /// let interval2 = Interval::new(Included(2), Excluded(4));
    /// let point1 = Interval::point(4);
    ///
    /// assert!(Interval::overlaps(&interval1, &interval2));
    /// assert!(Interval::overlaps(&interval1, &point1));
    /// assert!(!Interval::overlaps(&interval2, &point1));
    /// ```
    pub fn overlaps(first: &Interval<T>, second: &Interval<T>) -> bool {
        let high: &Bound<T>;
        let low: &Bound<T>;

        if *first == *second {
            return true;
        } else if first < second {
            low = second.low();
            high = first.high();
        } else {
            low = first.low();
            high = second.high();
        }

        match (low, high) {
            (Included(_low), Included(_high)) => _high >= _low,
            (Included(_low), Excluded(_high)) => _high > _low,
            (Excluded(_low), Included(_high)) => _high > _low,
            (Excluded(_low), Excluded(_high)) => _high > _low,
            _ => true,
        }
    }

    /// Returns true if `second` is a sub-interval of `first`, false otherwise
    ///
    /// # Examples
    /// ```
    /// use rudac::util::Interval;
    /// use std::ops::Bound::*;
    ///
    /// let interval1 = Interval::new(Included(2), Included(4));
    /// let interval2 = Interval::new(Included(2), Excluded(4));
    ///
    /// assert!(Interval::contains(&interval1, &interval2));
    /// ```
    pub fn contains(first: &Interval<T>, second: &Interval<T>) -> bool {
        if Interval::overlaps(first, second) {
            let overlap = Interval::get_overlap(first, second).unwrap();

            overlap == *second
        } else {
            false
        }
    }

    /// Get overlapped interval of `first` and `second`, `None` otherwise
    ///
    /// # Examples
    /// ```
    /// use rudac::util::Interval;
    /// use std::ops::Bound::*;
    ///
    /// // initialize intervals
    /// let interval1 = Interval::new(Included(2), Included(4));
    /// let interval2 = Interval::new(Included(2), Excluded(4));
    ///
    /// assert!(Interval::get_overlap(&interval1, &interval2).unwrap() == interval2);
    /// ```
    pub fn get_overlap(first: &Interval<T>, second: &Interval<T>) -> Option<Interval<T>> {
        if !Interval::overlaps(first, second) {
            return None;
        }

        let low = match (&first.low(), &second.low()) {
            (Included(low1), Included(low2))
            | (Excluded(low1), Included(low2))
            | (Excluded(low1), Excluded(low2)) => {
                if low1 >= low2 {
                    Rc::clone(&first.low)
                } else {
                    Rc::clone(&second.low)
                }
            }
            (Included(low1), Excluded(low2)) => {
                if low1 > low2 {
                    Rc::clone(&first.low)
                } else {
                    Rc::clone(&second.low)
                }
            }
            (Unbounded, Included(_)) | (Unbounded, Excluded(_)) => Rc::clone(&second.low),
            (Included(_), Unbounded) | (Excluded(_), Unbounded) => Rc::clone(&first.low),

            (Unbounded, Unbounded) => Rc::new(Unbounded),
        };

        let high = match (&first.high(), &second.high()) {
            (Included(high1), Included(high2))
            | (Excluded(high1), Included(high2))
            | (Excluded(high1), Excluded(high2)) => {
                if high1 <= high2 {
                    Rc::clone(&first.high)
                } else {
                    Rc::clone(&second.high)
                }
            }
            (Included(high1), Excluded(high2)) => {
                if high1 < high2 {
                    Rc::clone(&first.high)
                } else {
                    Rc::clone(&second.high)
                }
            }
            (Unbounded, Included(_)) | (Unbounded, Excluded(_)) => Rc::clone(&second.high),
            (Included(_), Unbounded) | (Excluded(_), Unbounded) => Rc::clone(&first.high),

            (Unbounded, Unbounded) => Rc::new(Unbounded),
        };

        Some(Interval { low, high })
    }
}

impl<T: Ord + std::fmt::Display> std::fmt::Display for Interval<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let low: String;
        let high: String;

        low = match &self.low() {
            Included(low) => format!("[{}", low),
            Excluded(low) => format!("({}", low),
            Unbounded => format!("(_"),
        };

        high = match &self.high() {
            Included(high) => format!("{}]", high),
            Excluded(high) => format!("{})", high),
            Unbounded => format!("_)"),
        };

        write!(f, "{},{}", low, high)
    }
}

impl<T: Ord> PartialEq for Interval<T> {
    fn eq(&self, other: &Self) -> bool {
        self.low == other.low && self.high == other.high
    }
}

impl<T: Ord> Eq for Interval<T> {}

impl<T: Ord> PartialOrd for Interval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // compare low end of the intervals
        let mut result = match (&self.low(), &other.low()) {
            (Included(low1), Included(low2)) => {
                if low1 < low2 {
                    Some(Ordering::Less)
                } else if low1 == low2 {
                    None
                } else {
                    Some(Ordering::Greater)
                }
            }
            (Included(low1), Excluded(low2)) => {
                if low1 <= low2 {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            (Excluded(low1), Included(low2)) => {
                if low1 < low2 {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            (Excluded(low1), Excluded(low2)) => {
                if low1 < low2 {
                    Some(Ordering::Less)
                } else if low1 == low2 {
                    None
                } else {
                    Some(Ordering::Greater)
                }
            }

            (Unbounded, Included(_)) => Some(Ordering::Less),
            (Unbounded, Excluded(_)) => Some(Ordering::Less),

            (Included(_), Unbounded) => Some(Ordering::Greater),
            (Excluded(_), Unbounded) => Some(Ordering::Greater),

            (Unbounded, Unbounded) => None,
        };

        // if low end was not enough to determine ordering, use high end
        if result.is_none() {
            result = match (&self.high(), &other.high()) {
                (Included(high1), Included(high2)) => {
                    if high1 < high2 {
                        Some(Ordering::Less)
                    } else if high1 == high2 {
                        Some(Ordering::Equal)
                    } else {
                        Some(Ordering::Greater)
                    }
                }
                (Included(high1), Excluded(high2)) => {
                    if high1 < high2 {
                        Some(Ordering::Less)
                    } else {
                        Some(Ordering::Greater)
                    }
                }
                (Excluded(high1), Included(high2)) => {
                    if high1 <= high2 {
                        Some(Ordering::Less)
                    } else {
                        Some(Ordering::Greater)
                    }
                }
                (Excluded(high1), Excluded(high2)) => {
                    if high1 < high2 {
                        Some(Ordering::Less)
                    } else if high1 == high2 {
                        Some(Ordering::Equal)
                    } else {
                        Some(Ordering::Greater)
                    }
                }
                (Unbounded, Included(_)) => Some(Ordering::Greater),
                (Unbounded, Excluded(_)) => Some(Ordering::Greater),

                (Included(_), Unbounded) => Some(Ordering::Less),
                (Excluded(_), Unbounded) => Some(Ordering::Less),

                (Unbounded, Unbounded) => Some(Ordering::Equal),
            };
        }

        return result;
    }
}

impl<T: Ord> Ord for Interval<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn util_interval_valid_1() {
        assert!(Interval::valid(&Interval::new(Included(2), Included(2))));
        assert!(Interval::valid(&Interval::new(Excluded(2), Included(3))));
        assert!(Interval::valid(&Interval::new(Included(2), Excluded(3))));
        assert!(Interval::valid(&Interval::new(Excluded(2), Excluded(3))));

        assert!(Interval::valid(&Interval::new(Unbounded, Included(1))));
        assert!(Interval::valid(&Interval::new(Excluded(2), Unbounded)));
        assert!(Interval::<usize>::valid(&Interval::new(
            Unbounded, Unbounded
        )));
    }

    #[test]
    #[should_panic(expected = "Interval is not valid")]
    fn util_interval_panic_new_1() {
        assert!(!Interval::valid(&Interval::new(Included(2), Included(1))));
    }

    #[test]
    #[should_panic(expected = "Interval is not valid")]
    fn util_interval_panic_new_2() {
        assert!(!Interval::valid(&Interval::new(Excluded(2), Included(1))));
    }

    #[test]
    #[should_panic(expected = "Interval is not valid")]
    fn util_interval_panic_new_3() {
        assert!(!Interval::valid(&Interval::new(Included(2), Excluded(1))));
    }

    #[test]
    #[should_panic(expected = "Interval is not valid")]
    fn util_interval_panic_new_4() {
        assert!(!Interval::valid(&Interval::new(Excluded(2), Excluded(1))));
    }

    #[test]
    fn util_interval_compare_1() {
        let interval1 = Interval::new(Included(2), Included(3));
        let interval2 = Interval::new(Included(2), Excluded(3));
        let interval3 = Interval::new(Excluded(2), Included(3));
        let interval4 = Interval::new(Excluded(2), Excluded(3));

        let interval5 = Interval::new(Unbounded, Excluded(3));
        let interval6 = Interval::new(Excluded(2), Unbounded);
        let interval7 = Interval::new(Unbounded, Excluded(3));
        let interval8 = Interval::<usize>::new(Unbounded, Unbounded);

        assert!(interval1 == interval1);
        assert!(interval1 > interval2);
        assert!(interval2 < interval1);
        assert!(interval2 < interval3);
        assert!(interval3 > interval4);
        assert!(interval5 < interval6);
        assert!(interval7 < interval6);
        assert!(interval5 < interval8);
        assert!(interval6 > interval8);
    }

    #[test]
    fn util_interval_overlaps_1() {
        let interval1 = Interval::new(Included(1), Included(3));
        let interval2 = Interval::new(Included(2), Included(4));

        assert!(Interval::overlaps(&interval1, &interval2));
    }

    #[test]
    fn util_interval_overlaps_2() {
        let interval1 = Interval::new(Included(1), Included(3));
        let interval2 = Interval::new(Included(2), Excluded(3));

        assert!(Interval::overlaps(&interval1, &interval2));
    }

    #[test]
    fn util_interval_overlaps_3() {
        let interval1 = Interval::new(Included(1), Included(3));
        let interval2 = Interval::new(Excluded(1), Excluded(3));

        assert!(Interval::overlaps(&interval1, &interval2));
    }

    #[test]
    fn util_interval_overlaps_4() {
        let interval1 = Interval::new(Included(1), Included(3));
        let interval2 = Interval::new(Excluded(3), Excluded(4));

        assert!(!Interval::overlaps(&interval1, &interval2));
    }

    #[test]
    fn util_interval_overlaps_5() {
        let interval1 = Interval::new(Included(1), Included(3));
        let interval2 = Interval::new(Excluded(0), Excluded(1));

        assert!(!Interval::overlaps(&interval1, &interval2));
    }

    #[test]
    fn util_interval_overlaps_6() {
        let interval1 = Interval::new(Included(1), Included(3));
        let interval2 = Interval::new(Excluded(4), Excluded(5));

        assert!(!Interval::overlaps(&interval1, &interval2));
    }

    #[test]
    fn util_interval_overlaps_7() {
        let interval1 = Interval::new(Unbounded, Included(3));
        let interval2 = Interval::new(Excluded(1), Excluded(5));

        assert!(Interval::overlaps(&interval1, &interval2));
    }

    #[test]
    fn util_interval_overlaps_8() {
        let interval1 = Interval::new(Included(1), Included(3));
        let interval2 = Interval::new(Excluded(4), Unbounded);

        assert!(!Interval::overlaps(&interval1, &interval2));
    }

    #[test]
    fn until_interval_get_overlap_1() {
        let interval1 = Interval::new(Included(1), Included(3));
        let interval2 = Interval::new(Included(2), Included(4));

        assert!(
            Interval::get_overlap(&interval1, &interval2).unwrap()
                == Interval::new(Included(2), Included(3))
        );
    }

    #[test]
    fn until_interval_get_overlap_2() {
        let interval1 = Interval::new(Included(1), Excluded(3));
        let interval2 = Interval::new(Included(2), Included(4));

        assert!(
            Interval::get_overlap(&interval1, &interval2).unwrap()
                == Interval::new(Included(2), Excluded(3))
        );
    }

    #[test]
    fn until_interval_get_overlap_3() {
        let interval1 = Interval::new(Included(1), Excluded(3));
        let interval2 = Interval::new(Included(2), Included(3));

        assert!(
            Interval::get_overlap(&interval1, &interval2).unwrap()
                == Interval::new(Included(2), Excluded(3))
        );
    }

    #[test]
    fn until_interval_get_overlap_4() {
        let interval1 = Interval::new(Excluded(1), Excluded(3));
        let interval2 = Interval::new(Included(1), Included(4));

        assert!(
            Interval::get_overlap(&interval1, &interval2).unwrap()
                == Interval::new(Excluded(1), Excluded(3))
        );
    }

    #[test]
    fn until_interval_get_overlap_5() {
        let interval1 = Interval::new(Unbounded, Excluded(3));
        let interval2 = Interval::new(Included(1), Included(2));

        assert!(
            Interval::get_overlap(&interval1, &interval2).unwrap()
                == Interval::new(Included(1), Included(2))
        );
    }

    #[test]
    fn until_interval_get_overlap_6() {
        let interval1 = Interval::new(Unbounded, Excluded(3));
        let interval2 = Interval::new(Unbounded, Included(2));

        assert!(
            Interval::get_overlap(&interval1, &interval2).unwrap()
                == Interval::new(Unbounded, Included(2))
        );
    }

    #[test]
    fn until_interval_get_overlap_7() {
        let interval1 = Interval::new(Excluded(2), Excluded(3));
        let interval2 = Interval::new(Unbounded, Included(3));

        assert!(
            Interval::get_overlap(&interval1, &interval2).unwrap()
                == Interval::new(Excluded(2), Excluded(3))
        );
    }

    #[test]
    fn until_interval_get_overlap_8() {
        let interval1 = Interval::new(Excluded(2), Unbounded);
        let interval2 = Interval::new(Unbounded, Unbounded);

        assert!(
            Interval::get_overlap(&interval1, &interval2).unwrap()
                == Interval::new(Excluded(2), Unbounded)
        );
    }

    #[test]
    fn until_interval_get_overlap_9() {
        let interval1 = Interval::new(Excluded(2), Included(3));
        let interval2 = Interval::new(Excluded(3), Included(4));

        assert!(Interval::get_overlap(&interval1, &interval2).is_none());
    }

    #[test]
    fn until_interval_get_overlap_10() {
        let interval1 = Interval::new(Excluded(2), Included(3));
        let interval2 = Interval::new(Included(4), Included(4));

        assert!(Interval::get_overlap(&interval1, &interval2).is_none());
    }

    #[test]
    fn until_interval_get_overlap_11() {
        let interval1 = Interval::new(Included(3), Included(4));
        let interval2 = Interval::new(Included(2), Included(3));

        assert!(
            Interval::get_overlap(&interval1, &interval2).unwrap()
                == Interval::new(Included(3), Included(3))
        );
    }

    #[test]
    fn until_interval_contains_1() {
        let interval1 = Interval::new(Included(1), Included(4));
        let interval2 = Interval::new(Included(2), Included(3));

        assert!(Interval::contains(&interval1, &interval2));
    }

    #[test]
    fn until_interval_contains_2() {
        let interval1 = Interval::new(Included(1), Included(4));
        let interval2 = Interval::new(Excluded(1), Included(3));

        assert!(Interval::contains(&interval1, &interval2));
    }

    #[test]
    fn until_interval_contains_3() {
        let interval1 = Interval::new(Included(1), Included(4));
        let interval2 = Interval::new(Included(1), Included(3));

        assert!(Interval::contains(&interval1, &interval2));
    }

    #[test]
    fn until_interval_contains_4() {
        let interval1 = Interval::new(Included(1), Included(4));
        let interval2 = Interval::new(Included(2), Excluded(4));

        assert!(Interval::contains(&interval1, &interval2));
    }

    #[test]
    fn until_interval_contains_5() {
        let interval1 = Interval::new(Included(1), Included(4));
        let interval2 = Interval::new(Included(2), Included(4));

        assert!(Interval::contains(&interval1, &interval2));
    }
}
