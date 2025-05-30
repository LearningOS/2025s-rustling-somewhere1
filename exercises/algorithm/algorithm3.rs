/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn sort<T: Ord + Copy>(array: &mut [T]){
	//TODO
    let len = array.len();
    for i in 0..len {
        // 假设当前索引的元素是最小值
        let mut min_index = i;
        // 在剩余的数组中找到最小值的索引
        for j in (i + 1)..len {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }
        // 如果找到的最小值索引不是当前索引，交换它们
        if min_index != i {
            array.swap(i, min_index);
        }
    }
    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}