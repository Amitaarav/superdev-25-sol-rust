
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

fn main(){
    let r = Rect {
        width: 20,
        length: 40
    };

    let s = Square {
        side: 10
    };

    let rect = get_area_and_perimeter(r);
    println!("{:?}", rect);

    // println!("{}", s.area());
    println!("{}", s.perimeter());
}
