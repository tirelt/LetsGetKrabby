fn merge_sort(v: Vec<i32>){
    match v.len() {
        0 => (),
        1 => (),
        2 => {
            if v[0]>v[1]{
                let temp = v[1];
                v[1] = v[0];
                v[0] = temp;
            }
        },
        n => {
            let v1 = &v[0..n/2].copied();
            let v2 = v[n/2..n].copy();
            println!("{n}");
            v
        }
    }
}

fn main() {
    let my_vec = vec![1,3,5,3,7,3,7,2,9];
    let sorted_vec = merge_sort(my_vec);
    println!("{sorted_vec:?}")
}
