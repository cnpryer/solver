use std::ops::Deref;

/// The `Sort` trait defines implementations for sortable data strucutres.
trait Sort<T> {
    fn sorted(&mut self, sorting: Sorting) -> &mut Self;
}

#[derive(Default)]
/// The `Sorting` enum provides different variants useful for describing how to sort an array.
/// TODO: Constraints(vec![Constraint])
enum Sorting {
    #[default]
    Ascend,
    Descend,
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
pub enum SmallArray<T> {
    Empty,
    One([T; 1]),
    Five([T; 5]),
    Ten([T; 10]),
    Dynamic(Vec<T>),
}

impl<T> SmallArray<T> {
    fn as_slice(&self) -> &[T] {
        match self {
            Self::Empty => &[],
            Self::One(it) => it,
            Self::Five(it) => it,
            Self::Ten(it) => it,
            Self::Dynamic(it) => it,
        }
    }

    fn empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

impl<T: Copy + Default + PartialOrd + Ord> Sort<T> for SmallArray<T> {
    fn sorted(&mut self, sorting: Sorting) -> &mut Self {
        if self.empty() {
            self
        } else {
            sort_small_array(self, sorting)
        }
    }
}

/// Sort a `SmallArray` with some `Sorting` variant. TODO:
fn sort_small_array<T: PartialOrd + Ord>(
    arr: &mut SmallArray<T>,
    sorting: Sorting,
) -> &mut SmallArray<T> {
    match arr {
        SmallArray::Five(it) => sort_five(it, sorting),
        SmallArray::Ten(it) => sort_ten(it, sorting),
        SmallArray::Dynamic(it) => sort_dynamic(it, sorting),
        _ => (),
    }
    arr
}

fn sort_five<T: PartialOrd + Ord>(it: &mut [T; 5], sorting: Sorting) {
    match sorting {
        Sorting::Ascend => it.sort(),
        Sorting::Descend => it.reverse(),
    }
}

fn sort_ten<T: PartialOrd + Ord>(it: &mut [T; 10], sorting: Sorting) {
    match sorting {
        Sorting::Ascend => it.sort(),
        Sorting::Descend => it.reverse(),
    }
}

fn sort_dynamic<T: PartialOrd + Ord>(it: &mut Vec<T>, sorting: Sorting) {
    match sorting {
        Sorting::Ascend => it.sort(),
        Sorting::Descend => it.reverse(),
    }
}

impl<T> Deref for SmallArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a, T> IntoIterator for &'a SmallArray<T> {
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Eq> Eq for SmallArray<T> {}

impl<T: PartialEq> PartialEq for SmallArray<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_array() {
        let arr = SmallArray::Five([0; 5]);
        assert_ne!(arr, SmallArray::Five([1; 5]))
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
