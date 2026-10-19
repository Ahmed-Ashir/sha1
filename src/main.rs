use sha1::MySha1;

fn main() {
    let input = String::from("Enter your input here to hash");
    println!("input to hash: {input}");
    let hash =MySha1::hash(&input);
    println!("Sha1 Hash: {hash} \n");
} 
