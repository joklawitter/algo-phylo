#![allow(unused)]
use nexus_parser::model::vertex::{BranchLength, Vertex};

#[test]
fn test_branch_lengths() {
    let test_length = 1.234;
    let vertex = Vertex::new_internal(5, (1, 2), Some(BranchLength::new(test_length)));
    assert_eq!(*vertex.branch_length().unwrap(), test_length);
}

#[test]
#[should_panic]
fn test_negative_branch_length() {
    let negative_length = BranchLength::new(-1.0);
}

#[test]
fn test_is_x() {
    let leaf = Vertex::new_leaf(0, Some(BranchLength::new(0.5)), 10);
    assert!(leaf.is_leaf());

    let vertex = Vertex::new_internal(0, (1, 2), Some(BranchLength::new(0.5)));
    assert!(vertex.is_internal());

    let root = Vertex::new_root(2, (42, 42));
    assert!(root.is_root());
}

#[test]
fn test_internal_vertex_has_no_label() {
    let vertex = Vertex::new_internal(0, (1, 2), Some(BranchLength::new(0.5)));
    assert_eq!(vertex.label_index(), None);
}

#[test]
fn test_parent_unset() {
    let vertex = Vertex::new_internal(0, (1, 2), Some(BranchLength::new(0.5)));
    assert_eq!(vertex.parent_index(), None);
    assert!(!vertex.has_parent());

    let leaf = Vertex::new_leaf(0, Some(BranchLength::new(0.5)), 0);
    assert_eq!(leaf.parent_index(), None);
    assert!(!leaf.has_parent());

    let root = Vertex::new_root(2, (42, 42));
    assert_eq!(root.parent_index(), None);
}

#[test]
fn test_leaf_has_no_children() {
    let vertex = Vertex::new_leaf(0, Some(BranchLength::new(0.5)), 42);
    assert_eq!(vertex.children(), None);
}


