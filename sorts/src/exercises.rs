use std::fmt::Debug;

#[allow(dead_code)]
fn ex_1(players: &mut Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    //сортировка как O(kln(k)) и все
    players.sort_unstable();
    let (left, rigth) = players.split_at(players.len() / 2);
    (left.to_vec().to_owned(), rigth.to_vec().to_owned())
}

#[allow(dead_code)]
//просто вектор буду использовать, а не какое-либо библиотечное множество.
//Если набор несортирован, то сортировка nln(n), так что решение 8б за n достаточно.
//Работает, но стэковерфлоу при значениях от-но больших, хвостовой оптимизации то нет))
fn ex_8(set: &Vec<f64>, value: f64, min: usize, max: usize, counter: i32) -> Option<(f64, f64)> {
    if !(min < max) {
        println!("count:{}", counter);
        None
    } else {
        let maybe = set[min] + set[max];
        if maybe == value {
            println!("count:{}", counter);
            println!("Success:{} + {} = {}", set[min], set[max], maybe);
            Some((set[min], set[max]))
        } else {
            if maybe > value {
                ex_8(set, value, min, max - 1, counter + 1)
            } else {
                ex_8(set, value, min + 1, max, counter + 1)
            }
        }
    }
}

#[allow(dead_code)]
fn ex_16(array: &mut Vec<i32>) -> i32 {
    fn partition(array: &mut Vec<i32>, l: usize, h: usize) -> usize {
        let p = h;
        let mut firsthigh = l;
        for i in l..h {
            if array[i] < array[p] {
                array.swap(i, firsthigh);
                firsthigh += 1;
            }
        }
        array.swap(p, firsthigh);
        firsthigh
    }
    fn quick_sort(array: &mut Vec<i32>, l: usize, h: usize) {
        if h > l {
            let p = partition(array, l, h);
            quick_sort(array, l, p - 1);
            quick_sort(array, p + 1, h);
        }
    }

    fn find_median(array: &mut Vec<i32>) -> i32 {
        let idx = array.len() / 2;
        find_median_go(array, 0, array.len() - 1, idx)
    }

    fn find_median_go(array: &mut Vec<i32>, l: usize, h: usize, idx: usize) -> i32 {
        match partition(array, l, h) {
            x if x == idx => array[x],
            x if x > idx => find_median_go(array, l, x - 1, idx),
            x if x < idx => find_median_go(array, x + 1, h, idx),
            _ => unreachable!()
        }
    }

    find_median(array)
}

#[allow(dead_code)]
fn ex_18() {
    #[derive(Copy, PartialEq, PartialOrd, Clone, Debug)]
    pub enum RGB {
        Red,
        Green,
        Blue,
    }

    let mut array = (1..100).collect::<Vec<i32>>().iter().map(|x| {
        match x % 3 {
            0 => RGB::Red,
            1 => RGB::Green,
            2 => RGB::Blue,
            _ => unreachable!()
        }
    }).collect::<Vec<RGB>>();

    fn examine(array: &Vec<RGB>, i: usize) -> RGB {
        array[i]
    }
    fn swap(array: Vec<RGB>, i: usize, j: usize) -> Vec<RGB> {
        let mut new_array = array.clone();
        new_array.swap(i, j);
        new_array
    }

    let mut border: usize = 0;
    for idx in 0..array.len() {
        let examine = array[idx];
        match examine {
            RGB::Red => {
                array.swap(border, idx);
                border += 1;
            }
            _ => {}
        }
    };
    border = array.len() - 1;
    for idx in 0..array.len() {
        let idx = array.len() - 1 - idx;
        let examine = array[idx];
        match examine {
            RGB::Blue => {
                array.swap(border, idx);
                border -= 1;
            }
            _ => {}
        }
    }

    array.iter().fold(array[0], |acc, x| {
        assert!(acc <= *x);
        *x
    });
}

#[allow(dead_code)]
fn ex_24<T: PartialEq + PartialOrd + Debug + Clone + Ord>(array: Vec<T>, n: usize) -> Vec<T> {
    fn merge<T: PartialEq + PartialOrd + Debug + Clone + Ord>(left: &[T], right: &[T]) -> Vec<T> {
        let mut result = Vec::new();
        let (mut i, mut j) = (0, 0);
        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                result.push(left[i].clone());
                i += 1;
            } else {
                result.push(right[j].clone());
                j += 1;
            }
        }
        if i < left.len() {
            for c in i..left.len() {
                result.push(left[c].clone());
            }
        }
        if j < right.len() {
            for c in j..right.len() {
                result.push(right[c].clone());
            }
        }
        result
    }

    let (left, right) = array.split_at(n);
    let mut right_sorted = right.to_vec(); //k*lnk = примерно корень из n на ln(n)
    right_sorted.sort_unstable();
    merge(left, &right_sorted[..])
}

#[cfg(test)]
mod tests {
    #[test]
    fn ex1_test() {
        use super::ex_1;
        let mut players = vec![11, 23, 51, 65, 87, 12];
        assert_eq!(ex_1(&mut players), (vec![11, 12, 23], vec![51, 65, 87]))
    }

    #[test]
    fn ex8_test() {
        use super::ex_8;
        let array = vec![11.0, 12.0, 23.0, 51.0, 65.0, 87.0];

        assert_eq!(ex_8(&array, 116 as f64, 0, array.len() - 1, 0), Some((51 as f64, 65 as f64)));
        assert_eq!(ex_8(&array, (87 + 65) as f64, 0, array.len() - 1, 0), Some((65 as f64, 87 as f64)));

        let number: Vec<f64> = (0..100).map(|x| x as f64).collect();
        println!("{:?}", ex_8(&number, 197 as f64, 0, number.len() - 1, 0));
    }

    #[test]
    fn ex16_test() {
        use super::ex_16;
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        let mut init = (1..100).collect::<Vec<i32>>();
        init.shuffle(&mut rng);
        let median = ex_16(&mut init);
        assert_eq!(median, 50);
    }

    #[test]
    fn ex18_test() {
        use super::ex_18;
        ex_18();
    }

    #[test]
    fn ex24_test() {
        use super::ex_24;
        let mut left_init = (6..105).collect::<Vec<i32>>();
        let mut right_init = vec![1,109,108,2, 3, 107, 106, 110,4,5];
        left_init.append(&mut right_init);


        let result = ex_24(left_init, 99);
        result.iter().fold(result[0],|accum, value|{

            assert!(accum <= *value);
            *value
        });


    }
}
