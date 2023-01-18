/**
 * File: array.rs
 * Created Time: 2023-01-15
 * Author: xBLACICEx (xBLACKICEx@outlook.com)
*/

fn random_access(nums: &[i32]) ->i32 {
    let random_index = rand::random::<usize>() % nums.len();
    let random_num = nums[random_index];
    random_num
}

fn extend(nums: Vec<i32>, enlarge: usize) -> Vec<i32> {
    // 创建一个长度为 nums.len() + enlarge 的新 Vec
    let mut res: Vec<i32> = vec![0; nums.len() + enlarge];

    // 将原数组中的所有元素复制到新 
    for i in 0..nums.len() {
        res[i]  = nums[i];
    }

    res
}

/* 在数组的索引 index 处插入元素 num */
fn insert(nums: &mut Vec<i32>, num: i32, index: usize) {
    // 把索引 index 以及之后的所有元素向后移动一位
    for i in (index + 1..nums.len()).rev() {
        nums[i] = nums[i - 1];
    }

    // 将 num 赋给 index 处元素
    nums[index] = num;
}

/* 删除索引 index 处元素 */
fn remove(nums: &mut Vec<i32>, index: usize) {
    // 把索引 index 之后的所有元素向前移动一位
    for i in index..nums.len() - 1 {
        nums[i] = nums[i + 1];
    }
}

#[allow(unused_variables)]
/* 遍历数组 */
fn traverse(nums: &[i32]) {
    let mut count = 0;

    // 通过索引遍历数组
    for _ in 0..nums.len() {
        count += 1;
    }

    // 直接遍历数组
    for _ in nums {
        count += 1;
    }
}

fn find(nums: &[i32],  target: i32) -> Option<usize> {
    for i in 0..nums.len() {
        if nums[i] == target {
            return Some(i);
        }
    }
    None
}

/* Driver Code */
fn main() {
    let arr = [0; 5];
    println!("数组 arr = {:?}", arr);
    // 在 Rust 中，指定长度时（[i32; 5]）为数组
	// 由于 Rust 的数组被设计为在编译期确定长度，因此只能使用常量来指定长度
	// 为了方便实现扩容 extend() 方法，以下将(Vec) 看作数组（Array）也是rust一般情况下使用动态数组的类型
    let nums = vec![1, 3, 2, 5, 4];
    println!("数组 nums = {:?}", nums);

    let random_num = random_access(&nums);
    println!("在 nums 中获取随机元素 {}", random_num);
    
    let mut nums = extend(nums, 3);
    println!("将数组长度扩展至 8 ，得到 nums = {:?}", nums);

    /* 插入元素 */
    insert(&mut nums, 6, 3);
    println!("在索引 3 处插入数字 6 ，得到 nums = {:?}", nums);

    /* 删除元素 */
    remove(&mut nums, 2);
    println!("删除索引 2 处的元素，得到 nums = {:?}", nums);

    /* 遍历数组 */
    traverse(&nums);

    /* 查找元素 */
    let index = find(&nums, 3);
    println!("在 nums 中查找元素 3 ，得到索引 = {:?}", index);
}