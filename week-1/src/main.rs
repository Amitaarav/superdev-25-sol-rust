
struct Rect {
    width: u32,
    length: u32,
}

struct Square {
    side: u32,
}

trait Shape{
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

impl Shape for Square {
    fn area(&self)-> u32{
        return self.side * self.side;
    }

    fn perimeter(&self) -> u32 {
        return 4 * self.side;
    }
}

impl Shape for Rect {
    fn area(&self) -> u32{
        return self.width * self.length;
    }

    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.length);
    }
}

fn get_area_and_perimeter(s: impl Shape) -> (u32, u32){
    return (s.area(), s.perimeter());
}

#[derive(Debug)]
struct User{
    name: String,
    age: u32,
}

// impl std::fmt::Debug for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}, {}", self.name, self.age)
//     }
// }

fn main(){
    let r = Rect {
        width: 20,
        length: 40
    };

    let s = Square {
        side: 10
    };

    let (area, perimeter) = get_area_and_perimeter(r);
    // println!("{}", rect);

    // println!("{}", s.area());
    println!("{}", s.perimeter());

    // 

    let u = User{
        name: String::from("Amit"),
        age: 24
    };

    println!("{:?}", u)
}
