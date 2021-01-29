fn main(){
    let mut arr:[i32;9] = [1,0,-3,2,10,9,-18,22,89];
    for i in 1..arr.len(){
        let mut t = 0;
        for j in 0..arr.len()-i{
            if arr[j] > arr[j+1] {
                t = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = t;
            }
        }
    }
    for el in arr.iter(){
        println!("{}",el)
    }
}
