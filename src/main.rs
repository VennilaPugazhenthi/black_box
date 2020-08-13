fn black_box_function(x: Vec<i32>) -> Vec<i32>{
    println!("Made it to black box");
    let mut z= vec![];
    for i in &x{
        z.push(i+2);        
    }
    z
}

fn main() {
    println!("Hello, world!");
    let x = vec![1, 2, 3];

    let z=black_box_function(x);
    let mut largest =0;
    let mut largest_index=0;
    let mut index=0;
    for i in &z{
        if(i>&largest){
            largest=*i;
            largest_index=index;
        }
        println!("The value of z in {}", i);
        index= index+1;
    }
    println!("The largest value is {}", largest);
    println!("The index of it is {}", largest_index);
    
}
