#[derive(Debug)]
struct Foo(u32);

const CONST_FOO: Foo = Foo(1);
// const mut CONST_MUT_FOO: Foo = Foo(3) - cant have const mut
static STATIC_FOO: Foo = Foo(3);
static mut STATIC_MUT_FOO: Foo = Foo(4);


fn main() {
    CONST_FOO.0 = 2;
    println!("{CONST_FOO:?}");  // This still prints Foo(1)

    let mut x = CONST_FOO;
    x.0 = 2;
    println!("{x:?}");  // This has made a copy, which is now mutable so prints Foo(2)
    
    unsafe {
        // CONST_FOO = Foo(1);  // illegal
        CONST_FOO.0 = 2; // legal, but makes a new tempoary variable, original CONST_FOO is unchanged
        // STATIC_FOO = Foo(1); // illegal
        // STATIC_FOO.0 = 2; // illegal
        STATIC_MUT_FOO = Foo(2);
        STATIC_MUT_FOO.0 = 2;
    }
}