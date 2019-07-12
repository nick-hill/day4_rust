struct Numbers<T> {
    x: T,
    y: T,
}

struct Points<T, U> {
    x: T,
    y : U,
}

pub fn generic() {

    //single type
    let integer = Numbers { x: 5, y: 10 };
    let float = Numbers{ x: 1.0, y: 4.0 };

    //multiple types
    let integer = Points { x: 5, y: 10 };
    let float = Points{ x: 1.0, y: 4.0 };
    let float_and_integer = Points{ x: 1, y: 4.0 };

}