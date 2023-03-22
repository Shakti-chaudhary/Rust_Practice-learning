fn main() {
    // let mut x: u8 = 10;
    // dbg!(x);

    // let y: i8 = 13;

    // let decimal = 02_55;
    // let hex = 0xff;
    // let octal = 0o377;
    // let binary = 0b1111_1111;

    // println!("{}", decimal);
    // println!("{}", hex);
    // println!("{}", octal);
    // println!("{}", binary);

    // let byte = b'A';
    // println!("{}", byte);

    // ===================================================
    //? Tuples

    // let tup = (500, "hi", true);
    // println!("{} is tup value ", tup.1);
    // let (x, y, z) = tup;
    // println!("{}", x);

    //  =============================================
    //? Arrays

    // let arr = [22, 22, 11, 1, 53];
    // println!("{:?} arr 2", arr[2]);

    // let arr1: [i32; 5] = [2, 33, 1, 4, 33];
    // println!("{:?} arr1 1", arr1[1]);

    // let val = arr[1];
    // println!("{} arr 1 == val", arr[1]);
    // println!("{} val ", val);

    // ==================================================
    //? Vectors

    // let mut vec = vec![2, 4, 11, 65];
    // vec.push(5);
    // vec.sort();
    // println!("vec : {:?} \n", vec);

    // vec.pop();
    // println!("vec : {:?} \n", vec);

    // println!("vec length : {:?} \n", vec.len());
    // println!("vec capacity : {:?} \n", vec.capacity());
    // println!("vec : {:?} \n", vec.iter());

    // let v: Vec<i32> = (0..5).collect();
    // println!("vec : {:?} ", v);

    // ==========================================
    //? Slices

    // let v: Vec<i32> = (0..5).collect();
    // // let sv: &[i32] = &v;
    // let sv: &[i32] = &v[2..4];
    // println!("vec : {:?} ", sv);

    // ==========================================
    //? String and &Str

    let name = String::from("Tyler");
    let course = "Rust".to_string();
    let new_name = name.replace("Tyler", "Tylr");

    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name);

    // &str = "String slices" or "stir"
    let str1 = "hello "; // str
    let str2 = str1.to_string();
    let str3 = &str2;

    println!("{}", str1);
    println!("{}", str2);
    println!("{}", str3);

    // compare string == != (does not equal)
    println!("{}", "ONE".to_lowercase() == "one");
}
