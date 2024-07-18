mod sds;

fn exec_sds() {
    let mut sds = sds::SDS::new();
    println!("Initial SDS: {:?}", sds);

    sds.sdscat("Hello");
    println!("After append 'Hello': {:?}", sds);
    println!("SDS length: {}", sds.sdslen());
    println!("SDS is empty: {}", sds.sdsempty());

    sds.sdscat(", world!");
    println!("After append ', world!': {:?}", sds);
    println!("SDS content: {}", sds.sds_to_string());

    let range = sds.sdsrange(1, 4);
    println!("SDS range from 1 to 4: {:?}", range);
    println!("SDS content: {:?}", sds.sds_to_string());

    sds.sdsclear();
    println!("After clear: {:?}", sds);
    println!("SDS is empty: {}", sds.sdsempty());
}

fn exec_zskiplist() {
    use curly_giggle::collection::skiplist::zskiplist::ZSkipList;
    let mut list: ZSkipList<i32> = ZSkipList::zsl_create();
    println!("Initial ZSkipList: {:?}", list);

    list.zsl_insert(1.0, 1);
    println!("After insert 1:\n{:?}", list);
    println!("ZSkipList length: {}", list.get_len());
    println!("ZSkipList contains 1: {}", list.contains(&1));
    println!("ZSkipList contains 2: {}", list.contains(&2));

    list.zsl_insert(2.0, 2);
    println!("After insert 2:\n{:?}", list);
    list.zsl_insert(3.0, 3);
    println!("After insert 3:\n{:?}", list);
    println!("ZSkipList contains 2: {}", list.contains(&2));
    println!("ZSkipList contains 3: {}", list.contains(&3));

    list.zsl_delete(&2);
    println!("After delete 2:\n{:?}", list);

    // Loop
    let mut loop_list: ZSkipList<i32> = ZSkipList::zsl_create();
    for i in 0..20 {
        loop_list.zsl_insert(i as f64, i);
    }
    println!("Loop insert:\n{:?}", loop_list);

    println!("ZSkipList get_rank {:?}", loop_list.zsl_get_rank(1.0, 3));
    
    for i in 0..20 {
        println!("ZSkipList get_element_by_rank {:?} -> {:?}", i, loop_list.zsl_get_element_by_rank(i));
    }

    print!("ZSkipList is in range: {:?}\n", loop_list.zsl_is_in_range(1.0, 15.0));

    print!("Zskiplist before delete range by score:\n{:?}", loop_list);
    print!("Delete range by score: {:?}\n", loop_list.zsl_delete_range_by_score(1.0, 3.0));
    print!("Zskiplist after delete range by score:\n{:?}", loop_list);
    print!("Delete range by score: {:?}\n", loop_list.zsl_delete_range_by_score(2.0, 10.0));
    print!("Zskiplist after delete range by score:\n{:?}", loop_list);
    print!("Delete range by score: {:?}\n", loop_list.zsl_delete_range_by_score(21.0, 23.0));
    print!("Zskiplist after delete range by score:\n{:?}", loop_list);

    let mut loop_list: ZSkipList<i32> = ZSkipList::zsl_create();
    for i in 0..20 {
        loop_list.zsl_insert(i as f64, i);
    }
    print!("Zskiplist before delete range by rank:\n{:?}", loop_list);
    print!("Delete range by rank: {:?}\n", loop_list.zsl_delete_range_by_rank(1, 3));
    print!("Zskiplist after delete range by rank:\n{:?}", loop_list);
    print!("Delete range by rank: {:?}\n", loop_list.zsl_delete_range_by_rank(2, 10));
    print!("Zskiplist after delete range by rank:\n{:?}", loop_list);
    print!("Delete range by rank: {:?}\n", loop_list.zsl_delete_range_by_rank(21, 23));
    print!("Zskiplist after delete range by rank:\n{:?}", loop_list);

}

fn main() {

    exec_sds();
    exec_zskiplist()
}
