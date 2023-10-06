pub fn multiply(a: f64) -> f64 {
    if(a == 0.0){
        println!("no calc made");
        return 0.0;
    }   
   return  a * a
}

pub fn check_grade(a:f64)->Option<f64>
{
    if a>10.0{
        return None;
    }
    return Some(a);
}

//fun return result
pub fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        return Err("Division by zero");
    }
    Ok(a / b)
}