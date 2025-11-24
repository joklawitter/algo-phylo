use nexus_parser::parse_nexus_file;
use std::path::Path;

#[test]
fn test_single_tree() {
    let path = Path::new("tests").join("fixtures").join("nexus_t1_n10.trees");
    let result = parse_nexus_file(path.to_str().unwrap());
    if let Err(e) = &result {
        eprintln!("Error parsing single tree: {:?}", e);
    }
    assert!(result.is_ok());

    let (trees, leaf_map) = result.unwrap();
    assert_eq!(trees.len(), 1);
    assert_eq!(leaf_map.num_labels(), 10);

    let tree = &trees[0];
    assert_eq!(tree.num_leaves(), 10);
    assert!(tree.is_valid());
}

#[test]
fn test_multiple_trees_with_translate() {
    let path = Path::new("tests").join("fixtures").join("nexus_t11_n20_translate.trees");
    let result = parse_nexus_file(path.to_str().unwrap());
    if let Err(e) = &result {
        eprintln!("Error parsing multiple trees: {:?}", e);
    }
    assert!(result.is_ok());

    let (trees, leaf_map) = result.unwrap();
    assert_eq!(trees.len(), 11);
    assert_eq!(leaf_map.num_labels(), 20);

    for tree in &trees {
        assert_eq!(tree.num_leaves(), 20);
        assert!(tree.is_valid());
    }
}

#[test]
fn test_comments_and_unknown_blocks() {
    let path = Path::new("tests").join("fixtures").join("nexus_t3_n10_comments.trees");
    let result = parse_nexus_file(path.to_str().unwrap());
    if let Err(e) = &result {
        eprintln!("Error parsing trees with lots of comments: {:?}", e);
    }
    assert!(result.is_ok());

    let (trees, leaf_map) = result.unwrap();
    assert_eq!(trees.len(), 3);
    assert_eq!(leaf_map.num_labels(), 10);

    for tree in &trees {
        assert_eq!(tree.num_leaves(), 10);
        assert!(tree.is_valid());
    }
}
