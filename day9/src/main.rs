fn main() {
    let r = (0..10).map(|x| x * x)
        .inspect(|x| {println!("{}", *x) })
        .filter(|x| *x<5)
        .filter_map(|x| Some(x))
        .inspect(|x| {println!("   {}", *x) })
        .fold(0, |acc, x| acc + x);
    println!("Hello, world! {}",r);
}
