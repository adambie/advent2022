
fn main() {

    let max = count(1, 3);
    println!("Hello, world! {}", max);
}

fn count(num: i32, times: i32) -> i32 {
    let arr:Vec<i32>=vec![1,2,3,4];
    let mut maks: i32 = 0;

    if (times-1)>0 {
        for (idx, x) in arr.iter().enumerate(){
            let mut result = count((num*x), (times-1));
            maks = maks.max(result);        
        }
    } else {
        maks = maks.max(num);
    }
    //println!("ret max: {}", maks);
    maks
}
