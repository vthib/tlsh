pub fn get_quartiles<const EFF_BUCKETS: usize>(bucket: &[u32; EFF_BUCKETS]) -> (u32, u32, u32) {
    let p1 = EFF_BUCKETS / 4 - 1;
    let p2 = EFF_BUCKETS / 2 - 1;
    let p3 = EFF_BUCKETS - EFF_BUCKETS / 4 - 1;

    let mut bucket_copy: [u32; EFF_BUCKETS] = *bucket;

    // XXX: this code replaces the C++ version with a Rust core equivalent.
    // See <https://github.com/vthib/tlsh/pull/17>.
    let (lesser, &mut q2, greater) = bucket_copy.select_nth_unstable(p2);
    let (_, &mut q1, _) = lesser.select_nth_unstable(p1);
    let (_, &mut q3, _) = greater.select_nth_unstable(p3 - p2 - 1);

    (q1, q2, q3)
}
