fn buble(arr: &[i32]) -> Vec<i32> {
    let mut x = arr.to_vec();
    let len = x.len();

    for i in 0..len {
        for j in 0..(len - 1 - i) {
            if x[j] > x[j+1] {
                let tmp = x[j+1];
                x[j+1] = x[j];
                x[j] = tmp;
            }
        }
    }

    return x;
}

fn quicksort(a: &mut [i32], l: usize, r: usize) {
     if l < r {
        let q = partition(a, l, r);
        quicksort(a, l, q);
        quicksort(a, q + 1, r);
    }
}

fn partition(a: &mut [i32], l: usize, r: usize) -> usize {
    let v: i32 = a[(l + r) / 2];
    let mut i: usize = l;
    let mut j: usize = r;
    while i <= j {
        while a[i] < v {
            i += 1;
        }
        while a[j] > v {
           j -= 1;
        }
        if i >= j {
           break
        }
        a.swap(i, j);
        i += 1;
        j -= 1;
    }
    return j
}


fn main() {
    println!("Hello, world!");

    let arr = [3,324,4234,1,2312,432,5452,124,2,1];
    println!("{:?}", buble(&arr));

    let mut arr2 = [3,324,4234,1,2312,432,5452,124,2,1];
    let arr2_len = arr2.len();
    quicksort(&mut arr2, 0, arr2_len - 1);
    println!("{:?}", arr2);
}
