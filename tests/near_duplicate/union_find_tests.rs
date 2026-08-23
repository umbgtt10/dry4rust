// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::near_duplicate::union_find::UnionFind;

#[test]
fn find_after_a_chain_of_unions_reports_one_root_for_the_whole_chain() {
    // Arrange
    let mut forest = UnionFind::new(5);
    forest.union(0, 1);
    forest.union(1, 2);
    forest.union(2, 3);

    // Act
    let roots: Vec<usize> = (0..4).map(|i| forest.find(i)).collect();

    // Assert
    assert!(
        roots.windows(2).all(|w| w[0] == w[1]),
        "a transitive chain collapses to a single root, got {roots:?}"
    );
    assert_ne!(forest.find(4), forest.find(0), "4 was never joined");
}

#[test]
fn find_on_a_fresh_forest_returns_the_element_itself() {
    // Arrange
    let mut forest = UnionFind::new(3);

    // Act
    let roots: Vec<usize> = (0..3).map(|i| forest.find(i)).collect();

    // Assert
    assert_eq!(roots, vec![0, 1, 2]);
}

#[test]
fn groups_on_a_fresh_forest_returns_one_singleton_each() {
    // Arrange
    let mut forest = UnionFind::new(3);

    // Act
    let groups = forest.groups();

    // Assert
    assert_eq!(groups, vec![vec![0], vec![1], vec![2]]);
}

#[test]
fn groups_orders_members_ascending_and_groups_by_lowest_member() {
    // Arrange
    let mut forest = UnionFind::new(6);
    forest.union(4, 1);
    forest.union(5, 2);
    forest.union(3, 0);

    // Act
    let groups = forest.groups();

    // Assert
    assert_eq!(
        groups,
        vec![vec![0, 3], vec![1, 4], vec![2, 5]],
        "the same input must always produce the same output"
    );
}

#[test]
fn groups_over_an_empty_forest_returns_nothing() {
    // Arrange
    let mut forest = UnionFind::new(0);

    // Act
    let groups = forest.groups();

    // Assert
    assert!(groups.is_empty());
}

#[test]
fn is_empty_distinguishes_a_zero_sized_forest_from_a_populated_one() {
    // Arrange & Act
    let empty = UnionFind::new(0);
    let populated = UnionFind::new(1);

    // Assert
    assert!(empty.is_empty());
    assert!(!populated.is_empty());
}

#[test]
fn len_reports_the_size_the_forest_was_built_with() {
    // Arrange & Act
    let forest = UnionFind::new(7);

    // Assert
    assert_eq!(forest.len(), 7);
}

#[test]
fn new_creates_a_forest_in_which_nothing_is_joined() {
    // Arrange & Act
    let mut forest = UnionFind::new(4);

    // Assert
    assert_eq!(forest.groups().len(), 4);
}

#[test]
fn union_of_an_element_with_itself_leaves_the_groups_unchanged() {
    // Arrange
    let mut forest = UnionFind::new(3);

    // Act
    forest.union(1, 1);

    // Assert
    assert_eq!(forest.groups(), vec![vec![0], vec![1], vec![2]]);
}

#[test]
fn union_repeated_over_the_same_pair_is_idempotent() {
    // Arrange
    let mut forest = UnionFind::new(3);

    // Act
    forest.union(0, 1);
    forest.union(0, 1);
    forest.union(1, 0);

    // Assert
    assert_eq!(forest.groups(), vec![vec![0, 1], vec![2]]);
}
