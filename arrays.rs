fn analyze_slice(slice: &[i32]){
    println!("First element of slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main(){
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // borrowing - access data without taking ownership over it (referencing)
    analyze_slice(&xs);
}1