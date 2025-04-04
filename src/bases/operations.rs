use super::conversion;

pub fn sum(num1:&str,base1:u8,num2:&str,base2:u8) -> f64 {
   return conversion::to_base_10(num1,base1) + conversion::to_base_10(num2,base2); 
}

pub fn prod(num1:&str,base1:u8,num2:&str,base2:u8) -> f64 {
   return conversion::to_base_10(num1,base1) * conversion::to_base_10(num2,base2); 
}
