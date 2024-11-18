fn main() {
    let input:[i32; 8] = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO
/* 
    let mut max = 0;let mut min = 100;

    for i in input {
        if i < min {
            min = i;
        } 
        if i > max {
            max = i;
        }
    }
*/

    let max = input.iter().max().unwrap_or(&0);
    let min = input.iter().min().unwrap();

    println!("{:?} is largest and {} is smallest", max, min);
}
