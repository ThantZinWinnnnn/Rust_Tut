 pub fn string_example ()-> () {
    let s = String::from("hello");
    let len = s.len();
    let _slice = &s[1..len];

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("Slice: {:?}", slice);
    assert_eq!(slice, &[2, 3])
}