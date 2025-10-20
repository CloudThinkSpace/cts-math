use std::{cmp::Ordering, ops::AddAssign};
use num::{Float, NumCast};

/// 计算数据的格式
pub fn count<T>(data: &[T]) -> usize {
    data.iter().count()
}

/// 计算数组的最小值
pub fn min<T>(data: &[T]) -> Option<T>
where
    T: PartialOrd + Copy,
{
    if data.is_empty() {
        return None;
    }

    let mut min_result = data[0];

    for &item in data.iter().skip(1) {
        if min_result > item {
            min_result = item;
        }
    }

    Some(min_result)
}


/// 计算数组的最大值
pub fn max<T>(data: &[T]) -> Option<T>
where
    T: PartialOrd + Copy,
{
    if data.is_empty() {
        return None;
    }

    let mut max_result = data[0];

    for &item in data.iter().skip(1) {
        if max_result < item {
            max_result = item;
        }
    }

    Some(max_result)
}

/// 计算数组的平均值
pub fn avg<T>(data: &[T]) -> Option<f64>
where
    T: std::convert::Into<f64> + Copy,
{
    if data.is_empty() {
        return None;
    }

    let mut result = 0.0;
    let mut count = 0;

    for &item in data {
        let value: f64 = item.into();
        result += value;
        count += 1;
    }

    Some(result / count as f64)
}

/// 计算数组的中位数
pub fn median<T>(numbers: &[T]) -> Option<T>
where
    T: PartialOrd + Clone,
{
    if numbers.is_empty() {
        return None;
    }

    let len = numbers.len();
    let mid = len / 2;

    // 如果只有一个元素，直接返回
    if len == 1 {
        return Some(numbers[0].clone());
    }

    // 创建可变副本进行排序
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort_by(|a, b| {
        a.partial_cmp(b).unwrap_or_else(|| {
            // 处理无法比较的情况，这里我们选择保持顺序
            Ordering::Equal
        })
    });

    if len % 2 == 0 {
        // 偶数个元素，取中间两个数的平均值
        // 注意：这里需要 T 支持加法和除法
        // 如果 T 不支持这些运算，可能需要不同的实现
        Some(sorted_numbers[mid - 1].clone())
    } else {
        // 奇数个元素，取中间的数
        Some(sorted_numbers[mid].clone())
    }
}

///计算数组的标准差
pub fn std_dev<T, U>(data: &[T]) -> U
where
    T: NumCast + Copy,
    U: Float + AddAssign + NumCast,
{
    if data.len() < 2 {
        return U::zero();
    }

    let n = U::from(data.len()).unwrap();
    let n_minus_one = U::from(data.len() - 1).unwrap();

    // 使用 fold 计算总和
    let sum = data.iter()
        .map(|&x| U::from(x).unwrap())
        .fold(U::zero(), |acc, x| acc + x);

    let mean = sum / n;

    // 使用 fold 计算方差
    let variance = data.iter()
        .map(|&x| {
            let val = U::from(x).unwrap();
            let diff = val - mean;
            diff * diff
        })
        .fold(U::zero(), |acc, x| acc + x) / n_minus_one;

    variance.sqrt()
}
