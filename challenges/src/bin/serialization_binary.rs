use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};
use std::f32;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct LidarPoint {
    x: f32,
    y: f32,
    z: f32,
}

type LidarCloud = Vec<LidarPoint>;

fn generate_random_point(rng: &mut ThreadRng) -> LidarPoint {
    let dist = Uniform::new(-100.0f32, 100.0f32);
    LidarPoint {
        x: dist.sample(rng),
        y: dist.sample(rng),
        z: dist.sample(rng),
    }
}

fn main() {
    let mut lidar_cloud: LidarCloud = Vec::new();
    let mut rng = rand::thread_rng();

    const NUMBER_OF_POINTS: u32 = 100_000;

    for _ in 0..NUMBER_OF_POINTS {
        lidar_cloud.push(generate_random_point(&mut rng));
    }

    // Serialize data to bytes
    let serialized_data: Vec<u8> = match bincode::serialize(&lidar_cloud) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Serialization error: {}", e);
            return;
        }
    };

    // Deserialize data from bytes
    let deserialized_cloud: LidarCloud = match bincode::deserialize(&serialized_data) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Deserialization error: {}", e);
            return;
        }
    };

    // Compare with original version
    println!(
        "Original size: {} | Deserialized size: {}",
        lidar_cloud.len(),
        deserialized_cloud.len()
    );

    // Compare all elements
    // 1. iter() - creates an iterator over the items in each vector
    // 2. zip(deserialized_cloud.iter()) - takes a pair of iterators and creates
    // a new iterator that yields pairs of elements (one from each iterator).
    // 3. all(|(p1, p2)| p1 == p2) This is a closure that takes each pair of points
    // and checks if they are equal. The all condition checks it for each pair in the zipped iterator.
    let clouds_equal = lidar_cloud
        .iter()
        .zip(deserialized_cloud.iter())
        .all(|(p1, p2)| p1 == p2);

    println!("Clouds equal: {}", clouds_equal);
}
