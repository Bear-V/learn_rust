mod demo;
pub fn fb(num: isize) -> isize {
    return demo::fei_bo(num);
}
pub fn loop_fun() {
    let mut num = 1;
    loop {
        num += 1;
        println!("loop num :{}", num);
        if num > 5 {
            break;
        }
    }
}

pub fn while_fun() {
    let mut num = 3;
    while num != 0 {
        println!("while num:{}", num);
        num -= 1;
    }
}

pub fn for_in_fun() {
    let arr = [2, 3, 4, 5, 6];
    // 再循环arr 数组时候需要 .iter() 返回可迭代的切片
    for item in arr.iter().rev() {
        println!("value in arr:{}", item)
    }
}

pub fn for_in_rev_fun() {
    // 反向迭代,包前不包后
    for number in (1..4).rev() {
        println!("rev for number:{}", number)
    }
    // 尝试
    // range类型是可以直接遍历的
    let tup_num = 1..8;
    for item in tup_num {
        println!("item:{}", item)
    }
}
