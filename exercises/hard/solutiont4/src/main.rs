//I AM NOT DONE 
//Calculated according to ISO8061 standard

mod calc_time;

fn main() {
    let day : String = calc_time::time_info("2023-10-10");
    println!("{}", day);
}

