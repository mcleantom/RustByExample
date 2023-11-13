mod my {
    pub struct OpenBox<T> {
        pub contents: T
    }

    pub struct ClosedBox<T> {
        contents: T
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { 
                contents: contents,
            }
        }
    }
}

fn main() {
    let open_box = my::OpenBox { contents: "Public information " };

    println!("The open box contains {}", open_box.contents);

    // ClosedBox's internals are private, cant construct
    // let _closed_box = my::ClosedBox { contents: "classified" };
    let _closed_box = my::ClosedBox::new("classified information");
    // println!("The closed box contains {}", _closed_box.contents);
}