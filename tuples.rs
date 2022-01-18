// tuples can be used as function args and as return values
// A tuple is a collection of values of different types

fn reverse(pair: (i32, bool)) -> (bool, i32){
    // let to bind
    let (integer, boolean) = pair;

    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main(){
    let pair = (1, false);
    println!("Reverse of pair {:?} is {:?}", pair, reverse(pair));
}