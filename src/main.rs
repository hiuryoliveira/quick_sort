fn main() {
    let mut data = [4, 2, 3, 1, 189, 178, 17, 48, 16, 63];
    quick_sort(&mut data);
    println!("{:?}", data);
}

pub(crate) fn quick_sort(data: &mut [i32]) {
    if data.len() <= 1 {
        return;
    }
    let pivot = data.len() / 2;
    let mut left = 0;
    let right = data.len() - 1;
    data.swap(pivot, right);
    for i in 0..data.len() {
        if data[i] < data[right] {
            data.swap(i, left);
            left += 1;
        }
    }
    data.swap(left, right);
    quick_sort(&mut data[0..left]);
    quick_sort(&mut data[left + 1..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut data = [4, 2, 3, 1];
        quick_sort(&mut data);
        assert_eq!(data, [1, 2, 3, 4]);

        let mut data = [5, 1, 9, 3, 7];
        quick_sort(&mut data);
        assert_eq!(data, [1, 3, 5, 7, 9]);

        let mut data = [10, 8, 6, 4, 2];
        quick_sort(&mut data);
        assert_eq!(data, [2, 4, 6, 8, 10]);
    }
}