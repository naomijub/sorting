# sorting-vec

A simple crate to sort a vector using a BTreeMap. It makes easier to remove duplicates and there is no real performance advantage against algorithms like mergesort, but it fun and pretty to implement. This crate contains one trait called `BTreeSort` that contains the functions:

* `uniques -> Vec<T>` - returns ascending sorted unique values.
* `sorted -> Vec<T>` - returns ascending sorted values.
* `reverse_uniques -> Vec<T>` - returns descending sorted unique values.
* `reverse_sort -> Vec<T>` - returns descending sorted values.

And the function `btree_sort` that provides an intermediary structure to support this sortings.

## Using function `btree_sort` and trait `BTreeSort`

```rust
use sorting_vec::{btree_sort, BTreeSort};

#[test]
fn integers() {
    let vec = vec![7, 3, 4,5, 6,8,3,2, -4, 5, 7,8, 0,9];
    let sort = btree_sort(vec);

    assert_eq!(sort.clone().uniques(), vec![-4, 0, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(sort.sorted(), vec![-4, 0, 2, 3, 3, 4, 5, 5, 6, 7, 7, 8, 8, 9]);
}

#[test]
fn usize() {
    let vec: Vec<usize> = vec![7, 3, 4,5, 6,8,3,2, 5, 7,8, 0,9];
    let sort = btree_sort(vec);

    assert_eq!(sort.clone().uniques(), vec![0, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(sort.clone().sorted(), vec![0, 2, 3, 3, 4, 5, 5, 6, 7, 7, 8, 8, 9]);
    assert_eq!(sort.reverse_uniques(), vec![9, 8, 7, 6, 5, 4, 3, 2, 0]);
}

#[test]
fn chars() {
    let vec = vec!['h', 'g', 'p', 'a', 'c', 'g'];
    let sort = btree_sort(vec);

    assert_eq!(sort.clone().uniques(), vec!['a', 'c', 'g', 'h', 'p']);
    assert_eq!(sort.clone().sorted(), vec!['a', 'c', 'g', 'g', 'h', 'p']);
    assert_eq!(sort.reverse_sort(), vec!['p', 'h', 'g', 'g', 'c', 'a']);
}

#[test]
fn string() {
    let vec = vec!["ha", "he", "ga", "12", "pow", "he", "543", "as", "cd", "ga"];
    let sort = btree_sort(vec);

    assert_eq!(sort.clone().uniques(), vec!["12", "543", "as", "cd", "ga", "ha", "he", "pow"]);
    assert_eq!(sort.sorted(), vec!["12", "543", "as", "cd", "ga", "ga", "ha", "he", "he", "pow"]);
}
```

## Using trait `BTreeSort`

```rust
use sorting_vec::{BTreeSort};

#[test]
fn vec_usize() {
    let vec: Vec<usize> = vec![7, 3, 4, 5, 6, 8, 3, 2, 5, 7, 8, 0, 9];

    assert_eq!(vec.clone().uniques(), vec![0, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(
        vec.clone().sorted(),
        vec![0, 2, 3, 3, 4, 5, 5, 6, 7, 7, 8, 8, 9]
    );
    assert_eq!(vec.reverse_uniques(), vec![9, 8, 7, 6, 5, 4, 3, 2, 0]);
}
```