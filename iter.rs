fn main(){
    let my_vector = vec![1,2,3];
    let doubled_vector: Vec<i32> = my_vector.into_iter().map(|x| x*2).collect();
    println!("{:?}",doubled_vector);   
}