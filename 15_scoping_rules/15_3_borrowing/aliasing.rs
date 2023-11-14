struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    println!("Point has coordinates: ({}, {}, {})",
             borrowed_point.x, another_borrow.y, point.z);

    // let mutable_borrow = &mut point;

    println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);
    
    // borrowed_point, another_borrow and point are 
    // no longer used after this point
    // so we can now borrow point as mutable

    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // the mutable_borrow goes out of scope here
    // so we can make an immutable borrow again
    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             new_borrowed_point.x, new_borrowed_point.y,
             new_borrowed_point.z);
}