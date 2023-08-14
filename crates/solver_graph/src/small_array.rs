use std::ops::Deref;

use crate::Value;

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
    fn as_slice(&self) -> &[V] {
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

    fn empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

impl<V: Value + PartialOrd + Ord> Sort<V> for SmallArray<V> {
    fn sorted(&mut self, sorting: Sorting) -> &mut Self {
        if self.empty() {
            self
        } else {
            sort_small_array(self, sorting)
        }
    }
}

/// Sort a `SmallArray` with some `Sorting` variant. TODO:
fn sort_small_array<V: Value + PartialOrd + Ord>(
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

fn sort<V: Value + PartialOrd + Ord>(it: &mut [V], sorting: Sorting) {
    match sorting {
        Sorting::Ascend => it.sort(),
        Sorting::Descend => it.reverse(),
        _ => (),
    }
}

/// The `Sort` trait defines implementations for sortable data structures.
trait Sort<V> {
    fn sorted(&mut self, sorting: Sorting) -> &mut Self;
}

#[derive(Default)]
/// The `Sorting` enum provides different variants useful for describing how to sort an array.
/// TODO: Constraints(vec![Constraint])
enum Sorting {
    #[default]
    Ascend,
    Descend,
    Constraint(Constraint),
}

struct Constraint;

impl<V> Deref for SmallArray<V> {
    type Target = [V];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a, V> IntoIterator for &'a SmallArray<V> {
    type Item = &'a V;

    type IntoIter = std::slice::Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<V: PartialEq + Value> PartialEq for SmallArray<V> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<V: Eq + Value> Eq for SmallArray<V> {}

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
