use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::Bound;
use std::ops::Bound::*;
use std::rc::Rc;

pub struct Interval<T: Ord> {
    low: Rc<Bound<T>>,
    high: Rc<Bound<T>>,
}

impl<T: Ord> Interval<T> {
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

    pub fn valid(interval: &Interval<T>) -> bool {
        match (&interval.low(), &interval.high()) {
            (Included(low), Included(high)) => low <= high,

            (Included(low), Excluded(high))
            | (Excluded(low), Included(high))
            | (Excluded(low), Excluded(high)) => low < high,

            _ => true,
        }
    }
    pub fn low(&self) -> &Bound<T> {
        self.low.as_ref()
    }

    pub fn high(&self) -> &Bound<T> {
        self.high.as_ref()
    }

    pub fn overlaps(first: &Interval<T>, second: &Interval<T>) -> bool {
        
        let high: &Bound<T>;
        let low: &Bound<T>;

        if *first == *second {
            return true;
        }
        else if first < second {
            low = second.low();
            high = first.high();
        }
        else {
            low = first.low();
            high = second.high();
        }

        match (low, high) {
            (Included(_low), Included(_high)) => _high >= _low,
            (Included(_low), Excluded(_high)) => _high > _low,
            (Excluded(_low), Included(_high)) => _high > _low,
            (Excluded(_low), Excluded(_high)) => _high > _low,
            _ => true
        }
    }

    pub fn contains(first: &Interval<T>, second: &Interval<T>) -> bool {
        if Interval::overlaps(first, second) {
            let overlap = Interval::get_overlap(first, second).unwrap();

            overlap == *second
        }
        else {
            false
        }
    }

    pub fn get_overlap(first: &Interval<T>, second: &Interval<T>) -> Option<Interval<T>> {

        if !Interval::overlaps(first, second) {
            return None;
        }

        let low = match (&first.low(), &second.low()) {
            (Included(low1), Included(low2))
            | (Included(low1), Excluded(low2))
            | (Excluded(low1), Excluded(low2)) => {
                if low1 <= low2 {
                    Rc::clone(&first.low)
                } else {
                    Rc::clone(&second.low)
                }
            }
            (Excluded(low1), Included(low2)) => {
                if low1 < low2 {
                    Rc::clone(&first.low)
                } else {
                    Rc::clone(&second.low)
                }
            }
            (Unbounded, _) | (_, Unbounded) => Rc::new(Unbounded),
        };

        let high = match (&first.high(), &second.high()) {
            (Included(high1), Excluded(high2)) => {
                if high1 < high2 {
                    Rc::clone(&first.low)
                } else {
                    Rc::clone(&second.low)
                }
            }
            (Included(high1), Included(high2))
            | (Excluded(high1), Included(high2))
            | (Excluded(high1), Excluded(high2)) => {
                if high1 <= high2 {
                    Rc::clone(&first.low)
                } else {
                    Rc::clone(&second.low)
                }
            }
            (Unbounded, _) | (_, Unbounded) => Rc::new(Unbounded),
        };

        Some(Interval { low, high })
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
                (Unbounded, Included(_)) => Some(Ordering::Less),
                (Unbounded, Excluded(_)) => Some(Ordering::Less),
                (Included(_), Unbounded) => Some(Ordering::Greater),
                (Excluded(_), Unbounded) => Some(Ordering::Greater),
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
    fn test_bound_1() {
        let bound1 = Bound::Included(1);
        let bound2 = Bound::Excluded(1);
        let bound3 = Bound::Included(1);
        let bound4 = Bound::Included(2);

        assert_eq!(bound1 == bound2, false);
        assert_eq!(bound1 == bound3, true);
        assert_eq!(bound3 == bound4, false);
    }

    #[test]
    fn test_bound_2() {
        let bound1 = Bound::Included(String::from("amin"));
        let bound2 = Bound::Excluded(String::from("amin"));
        let bound3 = Bound::Included(String::from("amin"));
        let bound4 = Bound::Included(String::from("amin2"));

        assert_eq!(bound1 == bound2, false);
        assert_eq!(bound1 == bound3, true);
        assert_eq!(bound3 == bound4, false);
    }
}
