use derive_more::{Add, Display, From, Into};

#[derive(PartialEq, From, Add, Debug, Display, Clone, Copy, Into)]
struct MyInt(i32);

#[derive(PartialEq, From, Into)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(PartialEq, From, Add, Display, Debug)]
enum MyEnum {
    #[display(fmt = "int: {_0}")]
    Int(i32),
    Uint(u32),
    #[display(fmt = "nothing")]
    Nothing,
}

fn main() -> anyhow::Result<()> {
    let my_int: MyInt = 10.into();
    let v = my_int + 20.into();
    let v1: i32 = v.into();

    println!("my_int: {}, v: {}, v1: {}", my_int, v, v1);

    let e: MyEnum = 10i32.into();
    let e1: MyEnum = 20u32.into();
    let e2 = MyEnum::Nothing;
    println!("e: {:?}, e1: {:?}, e2: {:?}", e, e1, e2);

    Ok(())
}
