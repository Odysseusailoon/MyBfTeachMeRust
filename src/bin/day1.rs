/*
*//*
0. mut, let, fn, in, un, fxx, println!
1. Vec::with_capacity, String::new
2. struct & derive(Debug)
3. references &B
4. Clone & Copy
5. &mut (mutation)
6. Ref operator overload (String -> &str, Vec<T> -> &[T])
*/

fn transfer(vector: Vec<char>) -> String {
    transfer2(&vector)
}

fn transfer2(vector: &[char]) -> String {
    vector.iter().collect()
}

fn main() {
    println!("{:?}", transfer(vec!['a', 'b', 'c']));
}

/*
#[derive(Debug, Clone, Copy)]
struct B {
    pub size: i32,
    pub size2: i32,
}

#[derive(Debug, Clone, Copy)]
struct A {
    pub size: i32,
    pub bb: B,
}

fn main() {
    let s = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let s_ref: &[i32] = &s;
    let mut s = String::new();
    s.push('c');
    let r = s.to_ascii_lowercase();
    let s_ref: &str = &s;
    println!("{:?}", s_ref);
}

fn bla(a: &mut A) -> &mut B {
    a.size = 123;
    &mut a.bb
}

fn blabla(s: &str) {}

*//*
fn main() {
    let mut s = String::new();
    blabla(&s);
    let s = "123";
    blabla(s);
}
*//*

*//*
fn main() {
    let mut aa = A {
        size: 0,
        bb: B { size: 0, size2: 0 },
    };
    let b = bla(&mut aa);
    b.size = 1;
    let b2 = bla(&mut aa);
    println!("main: {:?}", aa);
}
*//*
*/
