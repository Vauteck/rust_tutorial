fn main() {
    let mut vec_test = vec![0, 10, 50, 100, 30, 40];

    vec_test.sort();

    println!("median element is : {}", vec_test[vec_test.len()/2]);
}



