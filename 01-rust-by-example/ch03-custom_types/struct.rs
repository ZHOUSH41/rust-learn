#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// 单元结构体
struct Nil;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段（field）的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}
fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle{p1, p2} = rect;
    ((p1.x - p2.x) * (p1.y - p2.y)).abs()
}

fn square(point: Point, size: f32) -> Rectangle {
    Rectangle { 
        p1: Point{x: point.x, y: point.y},
        p2: Point{x: point.x+size, y:point.y+size}
    }
}
fn main() {
    // 使用简单的写法初始化字段，并创建结构体
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    
    // 以 Debug 方式打印结构体
    println!("{:?}", peter);
    
    // 实例化结构体 `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);
    
    // 使用结构体更新语法创建新的 point，这样可以用到之前的 point 的字段
    let new_point = Point { x: 0.1, ..point };
    
    // `new_point.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    println!("second point: ({}, {})", new_point.x, new_point.y);
    
    // 使用 `let` 绑定来解构 point
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // 实例化一个单元结构体
    let _nil = Nil;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let point = Point{ x: 3.0 , y: 2.0 };
    let rect = square(point, 4.0);
    println!("{}", rect_area(&rect));
}
