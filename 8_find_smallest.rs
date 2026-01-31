// A program to find and print the smallest of three numbers inputted

fn way1(a: &u32, b: &u32, c: &u32) -> u32 {
    let mut m = if a < b { a } else { b };
    if c < m { m = c; }
    *m
}

#[test]
fn test_way1() {
    assert_eq!(way1(&5, &6, &7), 5);
    assert_eq!(way1(&6, &5, &7), 5);
    assert_eq!(way1(&6, &7, &5), 5);
    assert_eq!(way1(&7, &6, &5), 5);
}
