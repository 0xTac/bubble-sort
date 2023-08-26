pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let size = arr.len();
    for i in 0..(size - 1) {
        let mut swapped = false;
        for j in 1..(size - i) {
            if arr[j - 1] > arr[j] { 
                arr.swap(j - 1, j);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

 fn main(){
        let mut vec = vec![-7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        bubble_sort(&mut vec);
        println!("排序: {:?}",vec);
  
    }
