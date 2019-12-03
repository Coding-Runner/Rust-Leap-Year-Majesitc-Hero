fn main() {
    let n = 2000;
    
    if n % 100 == 0 {
        if n % 400 == 0{
            print!("Leap Year!");
        } else {
            print!("Common Year!");
        }
    } else if n % 4 == 0{
        print!("Leap Year!");
    } else {
        print!("Common Year!");
    }
}
