use super::*;

#[test]
fn test_parent_hash() {
    for ciphersuite in Config::supported_ciphersuites() {
        // Number of leaf nodes in the tree
        const NODES: usize = 31;

        // Build a list of nodes, for which we need credentials and key package bundles
        let mut nodes = vec![];
        let mut key_package_bundles = vec![];
        for i in 0..NODES {
            let credential_bundle = CredentialBundle::new(
                vec![i as u8],
                CredentialType::Basic,
                ciphersuite.signature_scheme(),
            )
            .unwrap();
            let key_package_bundle =
                KeyPackageBundle::new(&[ciphersuite.name()], &credential_bundle, vec![]).unwrap();

            // We build a leaf node from the key packages
            let leaf_node = Node::Leaf(key_package_bundle.key_package().clone());
            key_package_bundles.push(key_package_bundle);
            nodes.push(Some(leaf_node));
            // We skip the last parent node (trees should always end with a leaf node)
            if i != NODES - 1 {
                // We insert blank parent nodes to get a longer resolution list
                nodes.push(None);
            }
        }

        // The first key package bundle is used for the tree holder
        let key_package_bundle = key_package_bundles.remove(0);

        let mut tree =
            RatchetTree::new_from_nodes(&ciphersuite, key_package_bundle, &nodes).unwrap();

        assert!(tree.verify_parent_hashes());

        // Populate the parent nodes with fake values
        for index in 0..tree.tree_size().as_usize() {
            // Filter out leaf nodes
            if NodeIndex::from(index).is_parent() {
                let (_private_key, public_key) = ciphersuite
                    .derive_hpke_keypair(&Secret::random(ciphersuite.hash_length()))
                    .into_keys();
                let parent_node = ParentNode::new(public_key, &[], &[]);
                let node = Node::Parent(parent_node);
                tree.public_tree
                    .replace(&NodeIndex::from(index), Some(node))
                    .unwrap();
            }
        }

        // Compute the recursive parent_hash for the first member
        let original_parent_hash = tree.set_parent_hashes(LeafIndex::from(0usize));

        // Swap two leaf nodes in the left & right part of the tree
        let node_15 = tree
            .public_tree
            .replace(
                &NodeIndex::from(15 as usize),
                tree.public_tree
                    .node(&NodeIndex::from(47 as usize))
                    .unwrap()
                    .clone(),
            )
            .unwrap();
        tree.public_tree
            .replace(&NodeIndex::from(47 as usize), node_15);

        // Compute the parent hash again to verify it has changed
        let leaf_swap_parent_hash = tree.set_parent_hashes(LeafIndex::from(0usize));

        assert!(leaf_swap_parent_hash != original_parent_hash);
    }
}
