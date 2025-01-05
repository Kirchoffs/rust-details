#[cfg(test)]
mod tests {
    use serde::Serialize;

    #[test]
    fn serialize_simple_enum() {
        #[derive(Debug, Serialize)]
        enum Direction {
            North,
            East,
            South,
            West,
        }

        println!("{:?}", bincode::serialize(&Direction::North).unwrap());
        println!("{:?}", bincode::serialize(&Direction::East).unwrap());
        println!("{:?}", bincode::serialize(&Direction::South).unwrap());
        println!("{:?}", bincode::serialize(&Direction::West).unwrap());
    }

    #[test]
    fn serialize_tuple_struct_enum() {
        #[derive(Debug, Serialize)]
        enum Scores {
            Chinese(i32),
            Math(i32),
            English(i32),
        }

        println!("{:?}", bincode::serialize(&Scores::Chinese(99)).unwrap());
        println!("{:?}", bincode::serialize(&Scores::Math(88)).unwrap());
        println!("{:?}", bincode::serialize(&Scores::English(77)).unwrap());
    }

    #[test]
    fn serialize_class_struct_enum() {
        #[derive(Debug, Serialize)]
        enum Shape {
            Circle { radius: f64 },
            Rectangle { width: f64, height: f64 },
            Square { side: f64 },
        }

        println!("{:?}", bincode::serialize(&Shape::Circle { radius: 3.141 }).unwrap());
        println!("{:?}", bincode::serialize(&Shape::Rectangle { width: 1.618, height: 0.618 }).unwrap());
        println!("{:?}", bincode::serialize(&Shape::Square { side: 2.718 }).unwrap());
    }
}
