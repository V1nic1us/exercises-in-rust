pub fn functions() {
    let width: i32 = 4;
    let height:i32 = 7;
    let depth:i32 = 10;

    {
        let area = area_of(width, height);
        println!("Area is {}", area);
    }

    print!("Volume is {}", volume_of(width, height, depth));

}

pub fn area_of(x:i32, y:i32)-> i32 {
    x * y
}

pub fn volume_of(x:i32, y:i32, z:i32)-> i32 {
    x * y * z
}