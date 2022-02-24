fn main() {
    let data = std::fs::read_to_string("input").unwrap();

    let desired_pre = "00000";



    // 
    // MD5
    // peter => "51dc30ddc473d43a6011e9ebba6ca770"
    let test = "peter".to_string();
    let mut test_bits = String::new();

    println!(":::Converting to bytes:::");
    let tb = test.into_bytes();
    println!(":::Converting to bits:::");
    for c in tb {
        test_bits += &format!("{:08b}", c);
    }
    let original_bit_len = test_bits.len() as u128;
    println!(":::Bits representation: :::");
    println!("bit len: {original_bit_len}, bits: {:#?}", test_bits);

    println!(":::Padding to div 512:::");

    let bits_to_add = 512-(original_bit_len+1)%512-64;
    println!("Required to add {bits_to_add} bits...");

    test_bits += "1";
    test_bits += &"0".repeat(bits_to_add.try_into().unwrap());

    let padded_bit_len = test_bits.len();
    println!("bit len: {padded_bit_len}, bits: {:#?}", test_bits);

    let final_pad = original_bit_len % u128::pow(2, 64);

    test_bits += &format!("{final_pad:064b}");

    let final_bit_len = test_bits.len();
    println!("final bit len: {final_bit_len}, bits: {:#?}", test_bits);

    test_bits.as_str().chars().ch


    // use something smarter like: https://stackoverflow.com/questions/57029974/how-to-split-string-into-chunks-in-rust-to-insert-spaces
    
}
