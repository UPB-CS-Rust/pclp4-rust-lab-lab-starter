fn main() {
    let input:[i32; 8] = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO

    let mut max = 0;let mut min = 100;

    for i in 0..8 {
        if input[i] < min {
            min = input[i];
        } 
        if input[i] > max {
            max = input[i];
        }
    }

    println!("{} is largest and {} is smallest", max, min);
}
