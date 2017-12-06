
fn reverse(pair: (i32, bool)) -> (bool, i32) { 
    let (interger, boolean) = pair;
    (boolean, interger)
}

fn main() {
    let pair = (1, true);
    let rev = reverse(pair);
    println!("{:?}", rev);
}