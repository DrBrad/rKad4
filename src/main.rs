mod utils;

fn main() {
    //e5af5f 5134c1e664b6f8260e9d99d7a871926b b8
    //e5af5f 5134c1e664b6f8260e9d99d7a8719254 f8
    //11100101101011110101111101010001001101001100000111100110011001001011011011111000001001100000111010011101100110011101011110101000011100011001001001010100
    // 11111000
    let uid = crate::utils::uid::UID::new("e5af5f5134c1e664b6f8260e9d99d7a8719254c7").unwrap();
    //println!("Binary: {}", uid.get_binary());
    println!("Hex: {}", uid.to_string());

    let test = uid.generate_node_id_by_distance(10);
    println!("Test: {}", test.get_binary());
    println!("Test: {}", test.to_string());
}
