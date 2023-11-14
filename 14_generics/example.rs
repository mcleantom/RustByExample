struct A;

struct Single(A);

struct SingleGen<T>(T);

fn reg_fn(_s: Single) {
    println!("called reg_fn(_s: A)");
}

fn gen_spec_t(_s: SingleGen<A>) {
    println!("called gen_spec_t(_s: SingleGen<A>)");
}

fn gen_spec_i32(_s: SingleGen<i32>) {
    println!("called gen_spec_i32(_s: SingleGen<i32>)");
}

fn generic<T>(_s: SingleGen<T>) {
    println!("called generic(_s: SingleGen<{}>)", std::any::type_name::<T>());
}


fn main() {
    // Single is concrete and explicitly takes `A`.
    let _s = Single(A);

    // SingleGen is generic and can take any type
    // The type can be specified explicitly
    let _char: SingleGen<char> = SingleGen('a');
    // Or the type can be inferred
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');

    reg_fn(Single(A));
    gen_spec_t(SingleGen(A));
    gen_spec_i32(SingleGen(6));

    generic(SingleGen('a'));
    generic::<char>(SingleGen('b'));
    generic(SingleGen::<String>('c'.to_string()));
    generic(SingleGen('d'.to_string()));
}