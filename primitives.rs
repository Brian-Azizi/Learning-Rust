use std::fmt;
use std::mem;


fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) ->  Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let _logical: bool = true;
    let _a_float: f64 = 1.0;
    let _an_integer = 5i32;

    let _default_float = 3.0;
    let _default_integer = 7;

    let mut _inferred_type = 12;
    _inferred_type = 4294967296i64;

    let mut _mutable = 12;
    _mutable = 21;

    // mutable = true;

    let _mutable = true;


    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    println!("{} {} {}", true && false, true || false, !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);
    

    let long_tuple = (
        1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true
    );
    
    println!("long tuple first and second {} {}", long_tuple.0, long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("pair {:?} reverse {:?}", pair, reverse(pair));

    println!("tuple {:?} and not {:?}", (5u32,), (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a,b,c,d) = tuple;
    println!("{:?} {:?} {:?} {:?}", a,b,c,d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("{}", transpose(matrix));



    let xs: [i32; 5] = [1,2,3,4,5];
    let ys: [i32; 500] = [0; 500];
    println!("0 {} 1 {}", ys[0], ys[1]);
    println!("size {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    println!("array occupies {} bytes", mem::size_of_val(&ys));

    // Arrays can be automatically borrows as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);
    analyze_slice(&ys[1 .. 4]);

    println!("{} bytes", mem::size_of_val(&ys[1 .. 4]));
    // println!("{}", xs[5]);
}
