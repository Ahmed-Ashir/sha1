pub struct MySha1 {}
  
impl MySha1 {
    /// Takes an input and hashes it using self created Sha1 algorithm.
    /// First we process our input String into a 2D vector of chunks where
    /// each outside vector contains another vector that contains 16 chunks 
    /// of 32 bit words. Then we extend our vector to contain 80 words instead of 16.
    /// And finally, we compute our hash via a compression function that alters the values of 
    /// 5 fixed length initialized valuess using bitwise operations with words from our 2d Vector. 
    pub fn hash(text: &String) -> String {
        //each section is seperated for understandability, hence need not be public to the caller

        //take input convert to ascii, pad as needed, then store digest in a 2D vector:
        //Sha256 and a lot of other common hash functions also use this preprocess algorithm
        let digest: Vec<Vec<u32>> = MySha1::pre_process(&text);

        //2nd level of 'padding' (extending from 16 words to 80 words per chunk is specific to Sha1 
        //hence not part of pre_process function definition above)
        println!("\nStep 4:   Extend each of the 16 '512 bit chunks' to 80 (using 4 bit-wise XOR operations then 1 left rotation)");
        let digest = MySha1::extend80(digest);

        //finally, compute hash via compression algorithm:
        println!("\nStep 5:   Compression algorithm to compute hash");
        let hash = MySha1::compress(digest);

        hash
    }

    fn pre_process(text: &String) -> Vec<Vec<u32>> {
        //convert input to array represented as binary ASCII numbers:
        println!("\nStep 1:   convert String to binary array (vector)");
        let bin_vec = MySha1::binary_vec(&text);
        
        //1st level of padding:
        println!("\nStep 2:   1st padding...");
        let padded_string = MySha1::padding_1st(bin_vec);

        //represent digest so far as chunks:
        println!("\nStep 3:   divide input into array of 512 char chunks (of arrays of 16 (32-bit) 'words')");
        let chunks = MySha1::allocate_chunks(padded_string);
        
        chunks
    }

    //helper function
    fn binary_vec(text: &String) -> Vec<String>{
        //take input text and split it into an array of character ASCII codes.
        let chars = text.trim_ascii().as_bytes().to_vec();
        println!("input as ASCII codes: \n{chars:?}");
        
        //convert each ascii code to binary and pad zeros until each are 8 bits long:
        let mut binary8bit = Vec::new();
        for char in chars {
            let binary = format!("{:08b}", char);
            //println!("{binary8bit}");
            binary8bit.push(binary);
        }
        println!("ASCII codes in binary: \n{binary8bit:?}");
        binary8bit
    }


    fn padding_1st(binary_vec: Vec<String>) -> String {
        //now join all (8 bit) binary numbers into a string and append a 1
        let mut num_string = binary_vec.join("") + "1";
        println!("input so far (as a string with a padded 1): \n{num_string}");
        
        //get length of numstring (before padded on 1), convert to binary
        //pad this with 0s (behind it, not after it)  until it's length is 64 bit representation
        let len_num_string = format!("{:064b}",num_string.len() - 1);
        let base_10_len = num_string.len()-1;
        println!("length of input (minus padded 1): {base_10_len}");
        println!("length of input represented in 64 bit binary: \n{len_num_string}");
        
        //now pad num_String with 0s until length is 512 (mod 448) i.e. some multiple of 512 mod 448
        while num_string.len() % 512 != 448 { num_string += "0"; }
        
        let mut digest_len = num_string.len();
        println!("padded input step 1: (pad 0s until length is some multiple of 512 mod 448)\n{num_string}");
        println!("length so far: {digest_len}");
        
        //now append len_num_string to numString:
        num_string = format!("{num_string}{len_num_string}");
        println!("padded input step 2: (append 64 bit binary representation of length to string)\n{num_string}");
        
        //verify this is 512 chars in length
        digest_len = num_string.len();
        println!("length so far: {digest_len} (notice always a multiple of 512)");
        
        num_string
    }


    //from this point on, we don't need to do anymore padding, and also we will be doing a lot of bitwise 
    //operations, hence we implement a 2d vector that stores each '32-bit word' as an integer (u32)
    fn allocate_chunks(mut num_string: String) -> Vec<Vec<u32>>{
        //break the message into an array of 'chunks' of 512 characters
        //break each chunk into a subarray of 16 32-bit 'words'
        let mut chunks: Vec<Vec<u32>> = Vec::new();
        let mut remaining_len: usize = num_string.len();
        let mut index = 0; //current chunk (index updated per chunk) 
        while remaining_len != 0 { 
            let mut chunk = num_string[..512].to_string();
            chunks.push(vec![u32::from_str_radix(&chunk[..32],2).unwrap()]); //outside array stores each chunk
            chunk = chunk.split_off(32);
            let mut remaining_chunk_len = chunk.len(); //(always <= 512)
            while remaining_chunk_len !=0 {
                //inside array stores 16 32-bit length 'words'
                chunks[index].push(u32::from_str_radix(&chunk[..32],2).unwrap());
                chunk = chunk.split_off(32); //done with current 'word' in chunk, loop and do remaining
                remaining_chunk_len = chunk.len();
            }
            num_string = num_string.split_off(512); //done with current chunk, loop and do remaining
            remaining_len = num_string.len(); 
            index +=1; //update index to next chunk in outside array before looping
        }
        //chunks is a Vec<Vec<u32>> but I want it to print the numbers out in binary, so I made my own print:
        MySha1::pretty_print(&chunks);
        
        chunks
    }


    fn extend80(mut chunks: Vec<Vec<u32>>) -> Vec<Vec<u32>>{
    //2nd level of padding (entend to 80 'words'): loop through each chunk array of sixteen 
    //32-bit words and extend each array to 80 words using bitwise operations
        for chunk in &mut chunks {
            for i in 16..80 {
                //take 4 words from the chunk using current i in the loop:
                let word_a = &chunk[i-3];
                let word_b = &chunk[i-8];
                let word_c = &chunk[i-14];
                let word_d = &chunk[i-16];

                //perform consecutive XOR bitwise operations going through each word:
                let xor = word_a ^ word_b ^ word_c ^ word_d; //then left rotate by 1:
                let new_word = xor.rotate_left(1);
                chunk.push(new_word); //keep adding to list until we have 80
                //then if we have more chunks, continue to next chunk and repeat 
            }
        }
        MySha1::pretty_print(&chunks);
        
        chunks
    }

    //if uncommenting out all printlns, make sure to de-underscore this fn:
    fn pretty_print(chunks: &Vec<Vec<u32>>) {
        // Pretty-print the outer vector
        println!("[");
        for (_row_idx, row) in chunks.iter().enumerate() {
            print!("    [");
            // Format each u32 into a 32-character binary string with leading zeros
            let binary_row: Vec<String> = row
                .iter()
                .map(|num| format!("\n        {:032b}", num))
                .collect();
            // Join them with a comma and space for readability
            println!("{} ", binary_row.join(", "));
            println!("    ],");
        }
        println!("]");
    }    

    
    fn compress(chunks: Vec<Vec<u32>>) -> String {
    //the compression function that computes our hash
        //initialize fixed length variables who hold the initial hash:
        let mut h: Vec<u32> = vec![0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
        let mut a = h[0].clone(); //In binary h0 = 01100111010001010010001100000001
        let mut b = h[1].clone(); //In binary h1 = 11101111110011011010101110001001
        let mut c = h[2].clone(); //In binary h2 = 10011000101110101101110011111110
        let mut d = h[3].clone(); //In binary h3 = 00010000001100100101010001110110
        let mut e = h[4].clone(); //In binary h4 = 11000011110100101110000111110000

        println!("values of h[i]: (before compression) ");
        MySha1::print_current_state(&h);

        for i in 0..chunks.len() { //for each 80 'word' chunk:
            let mut f: u32; 
            let mut k: u32;
            //values of f and k depend on which stage we are in:
            for j in 0..80 {
                match j {
                    0..20 => { //stage 1 in compression function
                        f = (b & c) | ((!b) & d);
                        k = 0b01011010100000100111100110011001;
                    }
                    20..40 => { //stage 2 in compression function
                        f = b ^ c ^ d;
                        k = 0b01101110110110011110101110100001;
                    }
                    40..60 => { //stage  3 in compression function
                        f = (b & c) | (b & d) | (c & d);
                        k = 0b10001111000110111011110011011100;
                    }
                    _ /*60..80*/ => { //stage 4 in compression function
                        f = b ^ c ^ d;
                    k = 0b11001010011000101100000111010110;
                    }
                }
                //with all our variables calculated, add them up to the current 'word'
                //using .wrapping_add() to safely allow 32-bit hardware overflow
                let word = chunks[i][j];
                let new_a = a.rotate_left(5)
                                .wrapping_add(f)
                                .wrapping_add(e)
                                .wrapping_add(k)
                                .wrapping_add(word);
                //reassign variable for next iteration:
                (e,d,c,b,a) = (d,c,b.rotate_left(30),a,new_a);
            }
            //now update our initial hash values
            h[0] = h[0].wrapping_add(a);
            h[1] = h[1].wrapping_add(b);
            h[2] = h[2].wrapping_add(c);
            h[3] = h[3].wrapping_add(d);
            h[4] = h[4].wrapping_add(e);
            //then continue to next chunk if there are more
        }
        println!("values of h[i]: (after compression aka hash vector). These hexadecimal values will be concatenated:");
        MySha1::print_current_state(&h);
        
        //by this point hashing is complete, we now just need to return the digest
        //as a hexadecimal String by concatenating each h[i] to h[i+1]:
        let hash: String = h
        .iter()
        .map(|num| format!("{:08x}", num))
        .collect();
    //println!("SHA-1 Hash: {hash}");
    hash
    }

    fn print_current_state(h: &Vec<u32>) {
        for (i, hash_i) in h.iter().enumerate() {
            println!("h{i} = 0b{:032b} (binary) = 0x{:08x} (hexadecimal) ", hash_i, hash_i);
        }
    }

}

use sha1_checked::{Sha1, Digest};

//unit test: We would like to confirm that 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_hash() {
        //my hash:
        let input = String::from("A Test");
        let myhash =MySha1::hash(&input);

        println!("\nCompare hash computed to built in Sha1 to make sure it is indeed correct:");
        println!("my hash:       {myhash}");
        //built in Sha1:
        let mut hasher = Sha1::new();
        hasher.update(b"A Test");
        let digest = hasher.finalize();
        let result = format!("{:x}", digest);
        println!("built in sha1: {result}");

        //compare the values and make sure they are equal: 
        assert_eq!(myhash, result);

    }
}
