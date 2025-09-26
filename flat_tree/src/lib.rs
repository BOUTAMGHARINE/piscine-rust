 use std::collections::BTreeSet;

pub fn flatten_tree<T: ToOwned<Owned = T>+ std::clone::Clone>(tree: &BTreeSet<T>) -> Vec<T> {

    tree.iter().cloned().collect()

}