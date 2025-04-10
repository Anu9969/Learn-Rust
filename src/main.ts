interface Shape{
  area() => number;
}

interface person{
  name:string;
  age:number
  gender: string;
}

type person2 = {
  name:string;
  age:number;
  gender: string;
}


class Reactangle implements Shape {
  constructor(){
    
  }

  area(){
    return 100;
  }
}


//Interview question
// //what is the difference between interface and type
//     -> interface can be extended
//       type cannot be extended
  