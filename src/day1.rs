use std::path::PathBuf;
use rocket::get;

#[get("/<nums..>")]
pub fn xorcube<'a>(nums: PathBuf) -> String{
   let x = nums.iter().rfold(0i64, |a,b| a ^ b.to_str().unwrap().parse::<i64>().unwrap());
   format!("{}", x.pow(3))
}