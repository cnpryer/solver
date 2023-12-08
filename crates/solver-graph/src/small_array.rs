use std::ops::{Add, Deref, DerefMut};

pub(crate) trait Reduce<V> {
    fn reduce(&self, reducer: Reducer<V>) -> SmallArray<V>;
}

/// The `Sort` trait defines implementations for sortable data structures.
pub(crate) trait Sort<V> {
    fn sorted(&mut self, sorting: Sorting) -> &mut Self;
}

#[derive(Debug, Clone)]
/// `SmallArray` is a compact array data structure for optimizing small graph problem search times.
/// The goal is to implement constraint-based sorting for `SmallArray`s.
///
/// ```rust
/// use solver_graph::SmallArray;
///
/// let arr = SmallArray::Five([0; 5]).sorted(Sorting::Ascend);
/// ```
pub(crate) enum SmallArray<V> {
    Empty,
    One([V; 1]),
    Two([V; 2]),
    Three([V; 3]),
    Four([V; 4]),
    Five([V; 5]),
    Six([V; 6]),
    Seven([V; 7]),
    Eight([V; 8]),
    Nine([V; 9]),
    Ten([V; 10]),
    Dynamic(Vec<V>),
}

impl<V> SmallArray<V> {
    pub(crate) fn push(&mut self, value: V) {
        match self {
            SmallArray::Empty => (),
            SmallArray::Dynamic(it) => it.push(value),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn pop(&mut self) -> Option<V> {
        match self {
            SmallArray::Empty => None,
            SmallArray::Dynamic(it) => it.pop(),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn as_slice(&self) -> &[V] {
        match self {
            SmallArray::Empty => &[],
            SmallArray::One(it) => it,
            SmallArray::Two(it) => it,
            SmallArray::Three(it) => it,
            SmallArray::Four(it) => it,
            SmallArray::Five(it) => it,
            SmallArray::Six(it) => it,
            SmallArray::Seven(it) => it,
            SmallArray::Eight(it) => it,
            SmallArray::Nine(it) => it,
            SmallArray::Ten(it) => it,
            SmallArray::Dynamic(it) => it,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        matches!(self, Self::Empty) || self.as_slice().is_empty()
    }
}

impl<V: PartialOrd + Ord> Sort<V> for SmallArray<V> {
    fn sorted(&mut self, sorting: Sorting) -> &mut Self {
        if self.is_empty() {
            self
        } else {
            sort_small_array(self, sorting)
        }
    }
}

impl<V: Copy + Add<Output = V>> Reduce<V> for SmallArray<V> {
    fn reduce(&self, reducer: Reducer<V>) -> SmallArray<V> {
        match (self, reducer) {
            (Self::Empty, Reducer::SumArray(Self::Empty)) => Self::Empty,
            (Self::Empty, Reducer::SumArray(SmallArray::One(b))) => SmallArray::One([b[0]]),
            (Self::One(a), Reducer::SumArray(Self::Empty)) => SmallArray::One([a[0]]),
            (Self::One(a), Reducer::SumArray(b)) if !a.is_empty() && !b.is_empty() => {
                SmallArray::One([a[0] + b[0]])
            }
            _ => unimplemented!(),
        }
    }
}

/// Sort a `SmallArray` with some `Sorting` variant. TODO:
fn sort_small_array<V: PartialOrd + Ord>(
    arr: &mut SmallArray<V>,
    sorting: Sorting,
) -> &mut SmallArray<V> {
    match arr {
        SmallArray::One(it) => sort(it, sorting),
        SmallArray::Two(it) => sort(it, sorting),
        SmallArray::Three(it) => sort(it, sorting),
        SmallArray::Four(it) => sort(it, sorting),
        SmallArray::Five(it) => sort(it, sorting),
        SmallArray::Six(it) => sort(it, sorting),
        SmallArray::Seven(it) => sort(it, sorting),
        SmallArray::Eight(it) => sort(it, sorting),
        SmallArray::Nine(it) => sort(it, sorting),
        SmallArray::Ten(it) => sort(it, sorting),
        SmallArray::Dynamic(it) => sort(it, sorting),
        SmallArray::Empty => (),
    }
    arr
}

fn sort<V: PartialOrd + Ord>(it: &mut [V], sorting: Sorting) {
    match sorting {
        Sorting::Ascend => it.sort(),
        Sorting::Descend => it.reverse(),
        _ => (),
    }
}

#[derive(Default)]
/// The `Sorting` enum provides different variants useful for describing how to sort an array.
/// TODO: Constraints(vec![Constraint])
pub(crate) enum Sorting {
    #[default]
    Ascend,
    Descend,
    Constraint(Constraint),
}

pub(crate) struct Constraint;
pub(crate) enum Reducer<'a, V> {
    Sum,
    SumArray(&'a SmallArray<V>),
    SumArrays(SmallArray<&'a SmallArray<V>>),
}

impl<V> Deref for SmallArray<V> {
    type Target = [V];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<V> DerefMut for SmallArray<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            SmallArray::Empty => &mut [],
            SmallArray::One(inner) => inner,
            SmallArray::Two(inner) => inner,
            SmallArray::Three(inner) => inner,
            SmallArray::Four(inner) => inner,
            SmallArray::Five(inner) => inner,
            SmallArray::Six(inner) => inner,
            SmallArray::Seven(inner) => inner,
            SmallArray::Eight(inner) => inner,
            SmallArray::Nine(inner) => inner,
            SmallArray::Ten(inner) => inner,
            SmallArray::Dynamic(inner) => inner.as_mut_slice(),
        }
    }
}

impl<'a, V> IntoIterator for &'a SmallArray<V> {
    type Item = &'a V;
    type IntoIter = std::slice::Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<V: PartialEq> PartialEq for SmallArray<V> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<V: Eq> Eq for SmallArray<V> {}

impl<V: PartialOrd + Ord> PartialOrd for SmallArray<V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<V: Ord> Ord for SmallArray<V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_array() {
        let arr = SmallArray::Two([0, 5]);
        assert_ne!(arr, SmallArray::Two([1, 5]));
    }

    #[test]
    fn test_sorted() {
        let mut arr = SmallArray::Five([1, 2, 3, 4, 5]);
        let mut desc = arr.clone();
        assert_eq!(
            arr.sorted(Sorting::Ascend),
            &mut SmallArray::Five([1, 2, 3, 4, 5])
        );
        assert_eq!(
            desc.sorted(Sorting::Descend),
            &mut SmallArray::Five([5, 4, 3, 2, 1])
        );
    }
}
