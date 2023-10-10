mod linear;
use linear::matrix;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;
    #[test]
    fn dot_test(){
        let mut arr0:Vec<Vec<f32>> = Vec::new();
        arr0.push(vec![1.0,2.0,3.0]);
        arr0.push(vec![4.0,5.0,6.0]);
        let mut arr1:Vec<Vec<f32>>=Vec::new();
        arr1.push(vec![1.0,2.0]);
        arr1.push(vec![3.0,4.0]);
        arr1.push(vec![5.0,6.0]);
        let mut m0 = matrix::matrix2D::new(arr0,[2,3]);
        let mut m1 = matrix::matrix2D::new(arr1,[3,2]);
        let mut r0 = matrix::dot2D(m0, m1);
        
        match r0 {
            Ok(re)=>{println!("成功: {:?}",re.array)},
            Err(msg)=>{println!("{}",msg)}
        }

        let mut arr2 :Vec<Vec<f32>>=Vec::new();
        let mut arr3:Vec<Vec<f32>>=Vec::new();
        arr2.push(vec![1.0,1.0]);
        arr2.push(vec![1.0,1.0]);
        
        arr3.push(vec![1.0,2.0]);
        arr3.push(vec![3.0,4.0]);
        
        let m2=matrix::matrix2D::new(arr2,[2,2]);
        let m3=matrix::matrix2D::new(arr3,[2,2]);
        let r1 = matrix::add2D(m2, m3);
        match r1{
            Ok(re)=>{println!("成功: {:?}",re.array)},
            Err(msg)=>{println!("{}",msg)}
        }
    }
}
