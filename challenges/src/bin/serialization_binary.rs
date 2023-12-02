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

trait Serializable {
    fn custom_serialize(&self) -> Vec<u8>;
    fn custom_deserialize(data: &[u8]) -> Option<Self>
    where
        Self: Sized;
}

macro_rules! impl_serializable_for_primitive {
    ($t:ty) => {
        impl Serializable for $t {
            fn custom_serialize(&self) -> Vec<u8> {
                self.to_ne_bytes().to_vec()
            }

            fn custom_deserialize(data: &[u8]) -> Option<Self> {
                if data.len() == std::mem::size_of::<$t>() {
                    let arr = data.try_into().ok()?;
                    Some(<$t>::from_ne_bytes(arr))
                } else {
                    None
                }
            }
        }
    };
}

// Implementation of Serializable Trait for primitive types
impl_serializable_for_primitive!(i8);
impl_serializable_for_primitive!(u8);
impl_serializable_for_primitive!(i16);
impl_serializable_for_primitive!(u16);
impl_serializable_for_primitive!(i32);
impl_serializable_for_primitive!(u32);
impl_serializable_for_primitive!(i64);
impl_serializable_for_primitive!(u64);
impl_serializable_for_primitive!(f32);
impl_serializable_for_primitive!(f64);

impl Serializable for String {
    fn custom_serialize(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn custom_deserialize(data: &[u8]) -> Option<String> {
        String::from_utf8(data.to_vec()).ok()
    }
}

impl<T> Serializable for Vec<T>
where
    T: Serializable + Sized,
{
    fn custom_serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // Serialize the length of the vector
        bytes.extend(&(self.len() as u32).to_ne_bytes());
        for item in self {
            let item_bytes = item.custom_serialize();
            // Optionally serialize the length of each item for variable-sized types
            // Required for something like String
            bytes.extend(&(item_bytes.len() as u32).to_ne_bytes());
            bytes.extend(item_bytes);
        }
        bytes
    }

    fn custom_deserialize(data: &[u8]) -> Option<Vec<T>> {
        let (len_bytes, mut remaining) = data.split_at(std::mem::size_of::<u32>());
        let num_elements = u32::from_ne_bytes(len_bytes.try_into().unwrap()) as usize;

        let mut vec = Vec::with_capacity(num_elements);

        while !remaining.is_empty() {
            if remaining.len() < std::mem::size_of::<u32>() {
                return None; // Not enough data for item length
            }

            let (len_bytes, rest) = remaining.split_at(std::mem::size_of::<u32>());
            let len = u32::from_ne_bytes(len_bytes.try_into().unwrap()) as usize;

            if rest.len() < len {
                return None; // Not enough data for item
            }

            let (item_data, rest_remaining) = rest.split_at(len);
            if let Some(item) = T::custom_deserialize(item_data) {
                vec.push(item);
            } else {
                return None; // Deserialization failed
            }

            remaining = rest_remaining
        }

        Some(vec)
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

    // Custom implementation
    let my_string = String::from("Hello, world!");
    let serialized_string = my_string.custom_serialize();
    let deserialized_string = String::custom_deserialize(&serialized_string).unwrap();
    println!("\nOriginal: {}", my_string);
    println!("Serialized: {:?}", serialized_string);
    println!("Deserialized: {}", deserialized_string);

    let my_vec = vec![1, 2, 3, 4];
    let serialized_vec = my_vec.custom_serialize();
    let deserialized_vec = Vec::<i32>::custom_deserialize(&serialized_vec).unwrap();
    println!("\nOriginal: {:?}", my_vec);
    println!("Serialized: {:?}", serialized_vec);
    println!("Deserialized: {:?}", deserialized_vec);

    let my_string_vec: Vec<String> = vec![
        "Hello".to_string(),
        "How are you?".to_string(),
        "I am fine!".to_string(),
    ];
    let serialized_string_vec = my_string_vec.custom_serialize();
    let deserialized_string_vec =
        Vec::<String>::custom_deserialize(&serialized_string_vec).unwrap();
    println!("\nOriginal: {:?}", my_string_vec);
    println!("Serialized: {:?}", serialized_string_vec);
    println!("Deserialized: {:?}", deserialized_string_vec);
}
