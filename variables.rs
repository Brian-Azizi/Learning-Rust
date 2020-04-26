fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning

    // =====

    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("Before: {}", mutable_binding);

    mutable_binding += 1;
    println!("After: {}", mutable_binding);

    // _immutable_binding += 1;

    // =====

    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;
        println!("Inner short: {}", short_lived_binding);

        // shadow
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }

    // println!("Outer short: {}", short_lived_binding);

    println!("Outer long: {}", long_lived_binding);

    // shadow
    let long_lived_binding = 'a';
    println!("Outer long: {}", long_lived_binding);
    

    // ====
    let a_binding;

    {
        let x = 2;

        // init
        a_binding = x * x;
    }

    println!("a binding {}", a_binding);

    let another_binding;
    // println!("another binding: {}", another_binding);

    another_binding = 1;
    println!("another binding: {}", another_binding);

}
