


pub fn main() {
    let vals = sp1_zkvm::io::read::<[u32; 9]>();
    let sum = sp1_zkvm::io::read::<u32>();
    
    let sum_ok = 
        vals[0] + vals[1] + vals[2] == sum &&
        vals[3] + vals[4] + vals[5] == sum &&
        vals[6] + vals[7] + vals[8] == sum &&
        
        vals[0] + vals[3] + vals[6] == sum &&
        vals[1] + vals[4] + vals[7] == sum &&
        vals[2] + vals[5] + vals[8] == sum &&
        
        vals[0] + vals[4] + vals[8] == sum &&
        vals[2] + vals[4] + vals[6] == sum;

    assert!(sum_ok);
    sp1_zkvm::io::commit(&sum_ok);
}
