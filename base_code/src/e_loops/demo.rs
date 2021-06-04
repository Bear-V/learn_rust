pub fn fei_bo(num: isize) -> isize {
    if num < 1 {
        return 0;
    }
    if num <= 3 {
        return 1;
    }
    return fei_bo(num - 1) + fei_bo(num - 2);
}
