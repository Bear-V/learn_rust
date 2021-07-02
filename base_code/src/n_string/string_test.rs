#[test]
fn string_test() {
    let _s = String::new();
    let data = "init content";
    let _s = data.to_string();
    let _s = "init content too".to_string();
    let _s = String::from("init content third");

    // haiyou byte
    for c in "list".chars() {
        println!("{}", c);
    }

    let hello = "hello word";

    let s = &hello[0..4];
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}
