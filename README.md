# Introduction

- Some simple implement for learning Redis

# Command

## Test

```
cargo test --test test_sds
```

## Benchmark

```
cargo bench --bench bm_sds
```

# List

## SDS

- [x] sdsnew
- [x] sdsempty
- [x] sdsfree
- [x] sdslen
- [x] sdsavail 
- [x] sdsdup 
- [x] sdsclear  
- [x] sdscat
- [x] sdscatsds 
- [x] sdscpy
- [x] sdsgrowzero
- [x] sdsrange
- [x] sdstrim
- [x] sdscmp

## ZSkipList

- [] zsl_create
- [] zsl_free
- [] zsl_insert
- [] zsl_delete
- [] zsl_get_rank
- [] zsl_get_element_by_rank
- [] zsl_is_in_range
- [] zsl_first_in_range
- [] zsl_last_in_range
- [] zsl_delete_range_by_score
- [] zsl_delete_range_by_rank




# Note 

- Type of buf

# Ref

- https://github.com/redis/redis
- https://rust-unofficial.github.io/too-many-lists/index.html#learn-rust-with-entirely-too-many-linked-lists
- The Design And Implementation Of Redis