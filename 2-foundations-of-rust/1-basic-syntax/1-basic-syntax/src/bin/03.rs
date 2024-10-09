fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];
    
    let mut mini = input[0];
    let mut maxi = input[0];

    for i in input{
        if i > maxi {
            maxi = i
        }
        if i < mini {
            mini = i
        }
    }
    // let mini = input.iter().min().unwrap_or(&0)
    // let maxi =input.iter().min().unwrap_or(&12)
    println!("{} is largest and {} is smallest", maxi, mini);
}
