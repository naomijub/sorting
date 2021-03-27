use std::collections::BTreeMap;

pub fn btree_sort<T: Ord>(vec: Vec<T>) -> BTreeMap<T, usize> {
    vec.into_iter().fold(BTreeMap::new(), |mut acc, e| {
        let b = acc.entry(e).or_insert(0);
        *b += 1;
        acc
    })
}

pub trait BTreeSort<T: Ord> {
    fn uniques(self) -> Vec<T>;
    fn sorted(self) -> Vec<T>;
    fn reverse_uniques(self) -> Vec<T>;
    fn reverse_sort(self) -> Vec<T>;
}

impl<T: Ord + Clone> BTreeSort<T> for BTreeMap<T, usize> {
    fn uniques(self) -> Vec<T> {
        self.into_iter().map(|(k, _)| k).collect::<Vec<T>>()
    }

    fn reverse_uniques(self) -> Vec<T> {
        let mut ord = self.into_iter().map(|(k, _)| k).collect::<Vec<T>>();
        ord.reverse();
        ord
    }

    fn sorted(self) -> Vec<T> {
        self.into_iter()
            .map(|(k, v)| vec![k; v])
            .flatten()
            .collect::<Vec<T>>()
    }

    fn reverse_sort(self) -> Vec<T> {
        let mut ord = self
            .into_iter()
            .map(|(k, v)| vec![k; v])
            .flatten()
            .collect::<Vec<T>>();
        ord.reverse();
        ord
    }
}

impl<T: Ord + Clone> BTreeSort<T> for Vec<T> {
    fn uniques(self) -> Vec<T> {
        btree_sort(self).into_iter().map(|(k, _)| k).collect::<Vec<T>>()
    }

    fn reverse_uniques(self) -> Vec<T> {
        let mut ord = btree_sort(self).into_iter().map(|(k, _)| k).collect::<Vec<T>>();
        ord.reverse();
        ord
    }

    fn sorted(self) -> Vec<T> {
        btree_sort(self).into_iter()
            .map(|(k, v)| vec![k; v])
            .flatten()
            .collect::<Vec<T>>()
    }

    fn reverse_sort(self) -> Vec<T> {
        let mut ord = btree_sort(self)
            .into_iter()
            .map(|(k, v)| vec![k; v])
            .flatten()
            .collect::<Vec<T>>();
        ord.reverse();
        ord
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn integers() {
        let vec = vec![7, 3, 4, 5, 6, 8, 3, 2, -4, 5, 7, 8, 0, 9];
        let sort = btree_sort(vec);

        assert_eq!(sort.clone().uniques(), vec![-4, 0, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(
            sort.sorted(),
            vec![-4, 0, 2, 3, 3, 4, 5, 5, 6, 7, 7, 8, 8, 9]
        );
    }

    #[test]
    fn usize() {
        let vec: Vec<usize> = vec![7, 3, 4, 5, 6, 8, 3, 2, 5, 7, 8, 0, 9];
        let sort = btree_sort(vec);

        assert_eq!(sort.clone().uniques(), vec![0, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(
            sort.clone().sorted(),
            vec![0, 2, 3, 3, 4, 5, 5, 6, 7, 7, 8, 8, 9]
        );
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

        assert_eq!(
            sort.clone().uniques(),
            vec!["12", "543", "as", "cd", "ga", "ha", "he", "pow"]
        );
        assert_eq!(
            sort.sorted(),
            vec!["12", "543", "as", "cd", "ga", "ga", "ha", "he", "he", "pow"]
        );
    }

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
}
