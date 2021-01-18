
fn main() {
    
    let hotels: [i32;7] = [0,50,75,200,399,600,2000];
    println!("Your array of hotels by distance: {:?}",hotels);
    let vec = opt_travel(&hotels);
    println!("The optimal sequence of hotels with minimum penalty is: {:?}",vec);
}


pub fn opt_travel(hotels: &[i32]) -> Vec<i32> {

    let mut opt_seq: Vec<i32> = vec![];

    //i to track current hotel
    let mut i = 0;
    let mut j = 0;
    let mut s = 0;

    //start vector at a1, end vector at an
    opt_seq.push(0);
    
  
    while i < hotels.len() - 1 {
         s = s + 1;
         j = i + 1;

         //start min penalty at a1 to an
        let mut min = (200 - (hotels[hotels.len()-1] - hotels[0])).pow(2);

        //iterate through i->j looking for min distance
        let mut flag = false;
        for j in j..hotels.len() {
            
            println!("{},{}",i,j);

            let temp = (200 - (hotels[j] - hotels[i])).pow(2);

            if min > temp && flag == false{
                min = temp;
                opt_seq.push(j as i32);
                println!("Pushed: {:?}\n",opt_seq);
                flag = true;
                
            }else if min > temp{
                min = temp;
                opt_seq.pop();
                opt_seq.push(j as i32);
                println!("Pushed: {:?}\n",opt_seq);
            }
              
        }
        i = opt_seq[s] as usize;
    }

    //return the vector
    opt_seq
}