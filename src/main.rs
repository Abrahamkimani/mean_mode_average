fn  main (){

    let mut v = vec![2, 4, 3, 1, 5,7 ];
   // let third: &i32 = &v[2];
   let length = v.len();
    println!("The number of elements is {}", length);
    println!("The unsorted vector is {:?}", v);
   v.sort();
   // println!("The  elements is {:?}", s);
    println!("The sorted vector is {:?}", v);
   // match v.get(2) {
   // Some(third) => println!("The third element is {}", ),
   // None => println!("There is no third element."),
  let median;
   if v.len() % 2 == 0 {

    median = length/2;
    let x: &i32 = &v[median - 1];
    let x2: &i32 = &v[median];
    let x3 = (x+x2)/2;
        
     println!("Median for even is {} ", x3);
   } else { 

      median = length/2;
      let x: &i32 = &v[median];
      println!("Median for odd is {} ", x);
   }


    }
    



