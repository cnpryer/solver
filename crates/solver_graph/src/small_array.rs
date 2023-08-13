use std::ops::Deref;

/// The `Sort` trait defines implementations for sortable data strucutres.
trait Sort<T> {
    fn sorted(&self, sorting: Sorting) -> Self;
}

#[derive(Default)]
/// The `Sorting` enum provides different variants useful for describing how to sort an array.
enum Sorting {
    #[default]
    Ascend,
    Descend,
}

/// The `Replace` trait defines implementations for replacing data in arrays.
trait Replace<T> {
    fn replace(&self, index: usize, value: T) -> Self; // TODO: Might need to be &self -> Self
}

#[derive(Debug, Clone)]
/// `SmallArray` is a compact array data structure for optimizing small graph problem search times.
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
    Mutable(Vec<T>),
}

impl<T> SmallArray<T> {
    fn as_slice(&self) -> &[T] {
        match self {
            Self::Empty => &[],
            Self::One(v) => v,
            Self::Five(v) => v,
            Self::Ten(v) => v,
            Self::Mutable(v) => v,
        }
    }
}

impl<T: Copy + Default> Sort<T> for SmallArray<T> {
    fn sorted(&self, _sorting: Sorting) -> Self {
        unimplemented!()
    }
}

impl<T: Copy + Default> Replace<T> for SmallArray<T> {
    fn replace(&self, _index: usize, _value: T) -> Self {
        unimplemented!()
    }
}

impl<T> Default for SmallArray<T> {
    fn default() -> Self {
        Self::Empty
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
        let arr = SmallArray::Five([0; 5]).sorted(Sorting::Ascend);
        assert_ne!(arr, SmallArray::Five([0; 5]))
    }
}
