mod sds;

fn main() {
    let mut sds = sds::SDS::new();
    println!("Initial SDS: {:?}", sds);

    sds.sdscat("Hello");
    println!("After append 'Hello': {:?}", sds);
    println!("SDS length: {}", sds.sdslen());
    println!("SDS is empty: {}", sds.sdsempty());

    sds.sdscat(", world!");
    println!("After append ', world!': {:?}", sds);
    println!("SDS content: {}", sds.to_string());

    sds.sdsclear();
    println!("After clear: {:?}", sds);
    println!("SDS is empty: {}", sds.sdsempty());
}
