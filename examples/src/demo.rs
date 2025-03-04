use hora::core::ann_index::{ANNIndex, SerializableIndex};
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Normal};

pub fn demo() {
    let n = 1000;
    let dimension = 64;

    // make sample points
    println!("Making samples");
    let mut samples = Vec::with_capacity(n);
    let normal = Normal::new(0.0, 10.0).unwrap();
    for _i in 0..n {
        let mut sample = Vec::with_capacity(dimension);
        for _j in 0..dimension {
            sample.push(normal.sample(&mut rand::thread_rng()));
        }
        samples.push(sample);
    }
    println!("Samples made");

    println!("samples: {:?}", samples.len());
    // init index
    println!("Init index");
    let mut index = hora::index::hnsw_idx::HNSWIndex::<f32, usize>::new(
        dimension,
        &hora::index::hnsw_params::HNSWParams::<f32>::default(),
    );
    println!("Index inited");
    // add points
    println!("Adding points");
    for (i, sample) in samples.iter().enumerate().take(n) {
        // add point
        index.add(sample, i).unwrap();
    }
    println!("Points added");
    println!("Building index");
    index.build(hora::core::metrics::Metric::CosineSimilarity).unwrap();
    println!("Index built");
    let mut rng = thread_rng();
    let target: usize = rng.gen_range(0..n);
    // 523 has neighbors: [523, 762, 364, 268, 561, 231, 380, 817, 331, 246]
    println!(
        "{:?} has neighbors: {:?}",
        target,
        index.search(&samples[target], 10) // search for k nearest neighbors
    );

    // save index
    println!("Saving index");
    let vec = index.dump_bin("hnsw_index").unwrap();
    let mut indexTwo = hora::index::hnsw_idx::HNSWIndex::<f32, usize>::load_bin(&vec).unwrap();
}
