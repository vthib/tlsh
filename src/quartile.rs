use core::cmp::Ordering;

use crate::BUCKETS;

pub fn get_quartiles<const EFF_BUCKETS: usize>(bucket: &[u32; BUCKETS]) -> (u32, u32, u32) {
    let mut short_cut_left = [0; EFF_BUCKETS];
    let mut short_cut_right = [0; EFF_BUCKETS];
    let p1 = EFF_BUCKETS / 4 - 1;
    let p2 = EFF_BUCKETS / 2 - 1;
    let p3 = EFF_BUCKETS - EFF_BUCKETS / 4 - 1;
    let end = EFF_BUCKETS - 1;
    let mut q1 = 0;
    let q2;
    let mut q3 = 0;

    // Safety: this expect is eliminated at compile time, as the compiler can
    // trivially verify that EFF_BUCKETS <= BUCKETS.
    let mut bucket_copy: [u32; EFF_BUCKETS] = bucket[..EFF_BUCKETS]
        .try_into()
        .expect("EFF_BUCKETS is bigger than BUCKETS");

    let mut spl = 0;
    let mut spr = 0;
    let mut l = 0;
    let mut r = end;
    loop {
        let ret = partition(&mut bucket_copy, l, r);
        match ret.cmp(&p2) {
            Ordering::Greater => {
                r = ret - 1;
                short_cut_right[spr] = ret;
                spr += 1;
            }
            Ordering::Less => {
                l = ret + 1;
                short_cut_left[spl] = ret;
                spl += 1;
            }
            Ordering::Equal => {
                q2 = bucket_copy[p2];
                break;
            }
        }
    }

    short_cut_left[spl] = p2 - 1;
    short_cut_right[spr] = p2 + 1;

    let mut l = 0;
    for mut r in short_cut_left.iter().take(spl + 1).copied() {
        match r.cmp(&p1) {
            Ordering::Greater => {
                loop {
                    let ret = partition(&mut bucket_copy, l, r);
                    match ret.cmp(&p1) {
                        Ordering::Greater => r = ret - 1,
                        Ordering::Less => l = ret + 1,
                        Ordering::Equal => {
                            q1 = bucket_copy[p1];
                            break;
                        }
                    }
                }
                break;
            }
            Ordering::Less => {
                l = r;
            }
            Ordering::Equal => {
                q1 = bucket_copy[p1];
                break;
            }
        }
    }

    let mut r = end;
    for mut l in short_cut_right.iter().take(spr + 1).copied() {
        match l.cmp(&p3) {
            Ordering::Less => {
                loop {
                    let ret = partition(&mut bucket_copy, l, r);
                    match ret.cmp(&p3) {
                        Ordering::Greater => r = ret - 1,
                        Ordering::Less => l = ret + 1,
                        Ordering::Equal => {
                            q3 = bucket_copy[p3];
                            break;
                        }
                    }
                }
                break;
            }
            Ordering::Greater => {
                r = l;
            }
            Ordering::Equal => {
                q3 = bucket_copy[p3];
                break;
            }
        }
    }

    (q1, q2, q3)
}

fn partition(buf: &mut [u32], left: usize, right: usize) -> usize {
    if left == right {
        return left;
    }
    if left + 1 == right {
        if buf[left] > buf[right] {
            buf.swap(left, right);
        }
        return left;
    }

    let mut ret = left;
    let pivot = (left + right) >> 1;
    let val = buf[pivot];
    buf.swap(pivot, right);

    for i in left..right {
        if buf[i] < val {
            buf.swap(ret, i);
            ret += 1;
        }
    }
    buf[right] = buf[ret];
    buf[ret] = val;

    ret
}
