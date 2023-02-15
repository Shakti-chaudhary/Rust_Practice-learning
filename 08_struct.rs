struct ShippingBox {
    depth: i32,
    height: i32,
    width: i32,
}

fn main() {
    let my_box = ShippingBox {
        depth: 2,
        width: 3,
        height: 5,
    };

    let tall = my_box.height;
    let wide = my_box.width;
    println!("the box is {:?} units tall", tall);
    println!("the box is {:?} units wide", wide);
}
