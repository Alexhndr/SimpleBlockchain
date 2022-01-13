use sha256::digest;

const STRING_TO_VALIDATE: &str = "0000";

pub fn proof_of_work(previous_proof: i64) -> i64 {
    let mut proof: i64 = 0;
    
    while !proof_is_valid(proof, previous_proof) {
        proof += 1;
    }
    
    proof
}

pub fn proof_is_valid(proof: i64, previous_proof: i64) -> bool {
    let string = format!("{}|{}", previous_proof, proof);
    let hash_string = digest(string);
    let length_of_hash_string = hash_string.len();
    let length_of_string_to_validate = STRING_TO_VALIDATE.len();
    
    if length_of_hash_string < length_of_string_to_validate {
        return false;
    }
    
    &hash_string[length_of_hash_string - length_of_string_to_validate..length_of_hash_string] == STRING_TO_VALIDATE
}
