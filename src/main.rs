
trait Shape{
  fn area(&self) -> f32;
}

struct Circle{
  radius: f32
}

struct Reactangle{
  width: u32,
  height: u32
}

impl Shape for Circle{
  fn area(&self) -> f32{
    3.14 *( self.radius * self.radius)
  }
}


fn get_area(aaakaar: impl Shape) -> f32{
    return aaakaar.area()
}

fn main() {
    let c:Circle = Circle{
        radius: 3.0
    };

  let r:Reactangle = Rectangle{
    width:3,
    height: 4
  };
    get_area(c);
  
}


//traits are similar to interfaces in java/javascript