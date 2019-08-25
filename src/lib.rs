#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
  use rand::Rng;
  use test::Bencher;

  #[bench]
  fn bench_build_id_tree (b : &mut Bencher) {
    let mut rng = rand::thread_rng();
    let values = (0..999).map(|_| rng.gen_range(-100i64, 101i64))
      .collect::<Vec<_>>();
    let offsets = (0..999).map(|i| rng.gen_range(0, i+1)).collect::<Vec<_>>();
    b.iter(|| {
      let mut tree = id_tree::TreeBuilder::new()
        .with_root (id_tree::Node::new (0i64)).build();
      let root_id = tree.root_node_id().unwrap().clone();
      for (v, o) in values.iter().zip(offsets.iter()) {
        let id = tree.traverse_pre_order_ids (&root_id).unwrap().skip (*o)
          .next().unwrap();
        let _ = tree.insert (
          id_tree::Node::new (*v),
          id_tree::InsertBehavior::UnderNode (&id)
        ).unwrap();
      }
      //assert_eq!(tree.traverse_pre_order(&root_id).unwrap().count(), 1000);
    });
  }
  #[bench]
  fn bench_traverse_id_tree (b : &mut Bencher) {
    let mut rng = rand::thread_rng();
    let values = (0..999).map(|_| rng.gen_range(-100i64, 101i64))
      .collect::<Vec<_>>();
    let vsum = values.iter().sum();
    let offsets = (0..999).map(|i| rng.gen_range(0, i+1)).collect::<Vec<_>>();
    let mut tree = id_tree::TreeBuilder::new()
      .with_root (id_tree::Node::new (0i64)).build();
    let root_id = tree.root_node_id().unwrap().clone();
    for (v, o) in values.iter().zip(offsets.iter()) {
      let id = tree.traverse_pre_order_ids (&root_id).unwrap().skip (*o)
        .next().unwrap();
      let _ = tree.insert (
        id_tree::Node::new (*v),
        id_tree::InsertBehavior::UnderNode (&id)
      ).unwrap();
    }
    b.iter(|| {
      let mut sum = 0;
      for n in tree.traverse_pre_order (&root_id).unwrap() {
        sum += n.data()
      }
      assert_eq!(sum, vsum);
    });
  }
  #[bench]
  fn bench_combined_id_tree (b : &mut Bencher) {
    let mut rng = rand::thread_rng();
    let values = (0..999).map(|_| rng.gen_range(-100i64, 101i64))
      .collect::<Vec<_>>();
    let vsum = values.iter().sum();
    let offsets = (0..999).map(|i| rng.gen_range(0, i+1)).collect::<Vec<_>>();
    b.iter(|| {
      let mut tree = id_tree::TreeBuilder::new()
        .with_root (id_tree::Node::new (0i64)).build();
      let root_id = tree.root_node_id().unwrap().clone();
      for (v, o) in values.iter().zip(offsets.iter()) {
        let id = tree.traverse_pre_order_ids (&root_id).unwrap().skip (*o)
          .next().unwrap();
        let _ = tree.insert (
          id_tree::Node::new (*v),
          id_tree::InsertBehavior::UnderNode (&id)
        ).unwrap();
      }
      let mut sum = 0;
      for n in tree.traverse_pre_order (&root_id).unwrap() {
        sum += n.data()
      }
      assert_eq!(sum, vsum);
    });
  }

  #[bench]
  fn bench_build_slab_tree (b : &mut Bencher) {
    let mut rng = rand::thread_rng();
    let values = (0..999).map(|_| rng.gen_range(-100i64, 101i64))
      .collect::<Vec<_>>();
    let offsets = (0..999).map(|i| rng.gen_range(0, i+1)).collect::<Vec<_>>();
    b.iter(|| {
      let mut tree = slab_tree::TreeBuilder::new().with_root(0i64).build();
      for (v, o) in values.iter().zip(offsets.iter()) {
        let at_id = tree.root().unwrap().traverse_pre_order().skip (*o).next()
          .unwrap().node_id();
        let mut at = tree.get_mut(at_id).unwrap();
        let _ = at.append(*v);
      }
      //assert_eq!(tree.root().unwrap().traverse_pre_order().count(), 1000);
    });
  }
  #[bench]
  fn bench_traverse_slab_tree (b : &mut Bencher) {
    let mut rng = rand::thread_rng();
    let values = (0..999).map(|_| rng.gen_range(-100i64, 101i64))
      .collect::<Vec<_>>();
    let vsum = values.iter().sum();
    let offsets = (0..999).map(|i| rng.gen_range(0, i+1)).collect::<Vec<_>>();
    let mut tree = slab_tree::TreeBuilder::new().with_root(0i64).build();
    for (v, o) in values.iter().zip(offsets.iter()) {
      let at_id = tree.root().unwrap().traverse_pre_order().skip (*o).next()
        .unwrap().node_id();
      let mut at = tree.get_mut(at_id).unwrap();
      let _ = at.append(*v);
    }
    b.iter(|| {
      let mut sum = 0;
      for n in tree.root().unwrap().traverse_pre_order() {
        sum += n.data()
      }
      assert_eq!(sum, vsum);
    });
  }
  #[bench]
  fn bench_combined_slab_tree (b : &mut Bencher) {
    let mut rng = rand::thread_rng();
    let values = (0..999).map(|_| rng.gen_range(-100i64, 101i64))
      .collect::<Vec<_>>();
    let vsum = values.iter().sum();
    let offsets = (0..999).map(|i| rng.gen_range(0, i+1)).collect::<Vec<_>>();
    b.iter(|| {
      let mut tree = slab_tree::TreeBuilder::new().with_root(0i64).build();
      for (v, o) in values.iter().zip(offsets.iter()) {
        let at_id = tree.root().unwrap().traverse_pre_order().skip (*o).next()
          .unwrap().node_id();
        let mut at = tree.get_mut(at_id).unwrap();
        let _ = at.append(*v);
      }
      let mut sum = 0;
      for n in tree.root().unwrap().traverse_pre_order() {
        sum += n.data()
      }
      assert_eq!(sum, vsum);
    });
  }

  #[bench]
  fn bench_build_ego_tree (b : &mut Bencher) {
    let mut rng = rand::thread_rng();
    let values = (0..999).map(|_| rng.gen_range(-100i64, 101i64))
      .collect::<Vec<_>>();
    let offsets = (0..999).map(|i| rng.gen_range(0, i+1)).collect::<Vec<_>>();
    b.iter(|| {
      let mut tree = ego_tree::Tree::new(0i64);
      for (v, o) in values.iter().zip(offsets.iter()) {
        let node_id = tree.root().descendants().skip (*o).next().unwrap().id();
        let mut node = tree.get_mut (node_id).unwrap();
        let _ = node.append (*v);
      }
      //assert_eq!(tree.nodes().count(), 1000);
    });
  }
  #[bench]
  fn bench_traverse_ego_tree (b : &mut Bencher) {
    let mut rng = rand::thread_rng();
    let values = (0..999).map(|_| rng.gen_range(-100i64, 101i64))
      .collect::<Vec<_>>();
    let vsum = values.iter().sum();
    let offsets = (0..999).map(|i| rng.gen_range(0, i+1)).collect::<Vec<_>>();
    let mut tree = ego_tree::Tree::new(0i64);
    for (v, o) in values.iter().zip(offsets.iter()) {
      let node_id = tree.root().descendants().skip (*o).next().unwrap().id();
      let mut node = tree.get_mut (node_id).unwrap();
      let _ = node.append (*v);
    }
    b.iter(|| {
      let mut sum = 0;
      for node in tree.root().descendants() {
        sum += node.value()
      }
      assert_eq!(sum, vsum);
    });
  }
  #[bench]
  fn bench_combined_ego_tree (b : &mut Bencher) {
    let mut rng = rand::thread_rng();
    let values = (0..999).map(|_| rng.gen_range(-100i64, 101i64))
      .collect::<Vec<_>>();
    let vsum = values.iter().sum();
    let offsets = (0..999).map(|i| rng.gen_range(0, i+1)).collect::<Vec<_>>();
    b.iter(|| {
      let mut tree = ego_tree::Tree::new(0i64);
      for (v, o) in values.iter().zip(offsets.iter()) {
        let node_id = tree.root().descendants().skip (*o).next().unwrap().id();
        let mut node = tree.get_mut (node_id).unwrap();
        let _ = node.append (*v);
      }
      let mut sum = 0;
      for node in tree.root().descendants() {
        sum += node.value()
      }
      assert_eq!(sum, vsum);
    });
  }
  #[bench]
  fn bench_combined_ego_tree_insert_order (b : &mut Bencher) {
    let mut rng = rand::thread_rng();
    let values = (0..999).map(|_| rng.gen_range(-100i64, 101i64))
      .collect::<Vec<_>>();
    let vsum = values.iter().sum();
    let offsets = (0..999).map(|i| rng.gen_range(0, i+1)).collect::<Vec<_>>();
    b.iter(|| {
      let mut tree = ego_tree::Tree::new(0i64);
      for (v, o) in values.iter().zip(offsets.iter()) {
        let node_id = tree.nodes().skip (*o).next().unwrap().id();
        let mut node = tree.get_mut (node_id).unwrap();
        let _ = node.append (*v);
      }
      let mut sum = 0;
      for node in tree.nodes() {
        sum += node.value()
      }
      assert_eq!(sum, vsum);
    });
  }
}
