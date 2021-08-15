#[cfg(test)]
mod iterator_test {
    use crate::u_iterator::example::{shoes_in_my_size, Counter, Shoe};

    #[test]
    fn one() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        for val in v1_iter {
            println!("{}", &val);
        }
    }

    #[test]
    fn two() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None)
    }

    #[test]
    fn third() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6)
    }

    #[test]
    fn four() {
        let v1 = vec![1, 2, 3];

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        println!("{:?}", v1);
        assert_eq!(v2, vec![2, 3, 4])
    }

    #[test]
    fn five() {
        let shoes: Vec<Shoe> = vec![
            Shoe {
                size: 10,
                style: "sneaker".to_string(),
            },
            Shoe {
                size: 23,
                style: "sneaker".to_string(),
            },
            Shoe {
                size: 34,
                style: "sneaker".to_string(),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 23);

        println!("{:?}", in_my_size);
    }

    #[test]
    fn six() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn seven() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        println!("sum is :{}", sum);

        let new_sum = Counter::new();
        println!("{:?}", &new_sum);
        let other_counter = Counter::new().skip(1);
        println!("{:?}", &other_counter);
        let zip_sum = new_sum.zip(other_counter);
        println!("{:?}", &zip_sum);
        let map_sum = zip_sum.map(|(a, b)| a * b);
        println!("{:?}", &map_sum);
        let filter_sum = map_sum.filter(|x| x % 3 == 0);
        println!("{:?}", filter_sum);
        let sum_result: u32 = filter_sum.sum();
        println!("{}", sum_result);

        let n_sum = vec![4, 4, 4];
        let o_sum = vec![2, 2, 4];
        let ot = o_sum.into_iter().skip(1);
        println!("{:?}", ot);
        let zip_s = n_sum.into_iter().zip(ot);
        let result: i32 = zip_s.map(|(a, b)| a * b).filter(|x| x % 3 == 0).sum();

        println!("{}", result);
    }
}

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
