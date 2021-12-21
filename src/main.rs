use std::collections::HashMap;

fn  main (){

   
   //let v = vec![String::from("Marks"),];
      let mut initial_scores = vec![10, 50, 30, 60, 70];
      initial_scores.sort();
      let sum: i32 = initial_scores.iter().sum();

      let length = initial_scores.len();

      println!("The number of elements is {}", length);

     println!("The sum of the numbers is {}", sum);
    // println!("The mean of the elements is {}", sum/initial_scores.len());
  
     // let third: &i32 = &v[2];
   
 
    println!("The unsorted vector is {:?}", initial_scores);
   
   // println!("The  elements is {:?}", s);
    println!("The sorted vector is {:?}", initial_scores);
   // match v.get(2) {
   // Some(third) => println!("The third element is {}", ),
   // None => println!("There is no third element."),
  let median;
   if initial_scores.len() % 2 == 0 {

    median = length/2;
    let x: &i32 = &initial_scores[median - 1];
    let x2: &i32 = &initial_scores[median];
    let x3 = (x+x2)/2;
        
     println!("Median for even is {} ", x3);
   } else { 

      median = length/2;
      let x: &i32 = &initial_scores[median];
      println!("Median for odd is {} ", x);
   }

   let mut scores = HashMap::new();
  // 10, 50, 30, 60, 70
  /*the key is element of the vector and the value is the number of occurence the element appears in
  the vector*/
  scores.insert(i32::from(10), i32::from(1)); 
  scores.insert(i32::from(50), i32::from(1));
  scores.insert(i32::from(30), i32::from(1));
  scores.insert(i32::from(60), i32::from(1));
  scores.insert(i32::from(70), i32::from(1));

  for number in scores {
   let count = scores.entry(number).or_insert(0);
   *count += 1;
  }


  }
  


    
    



