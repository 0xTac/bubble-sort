fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main(){
    let mut order = vec![88, 4 , 29, 38, 21, 12, 2, 5, 7, 100, 30, 250];
    println!("排序前: {:?}",order);
  
    bubble_sort(&mut order);
    println!("排序后：{:?}",order);
}
