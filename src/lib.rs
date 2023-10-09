struct matrix2D{
    array:Vec<Vec<f32>>,
    shape:[usize;2]
}

impl matrix2D {
    fn new(arr:Vec<Vec<f32>>,shape:[usize;2])->Self{
        Self{
            array:arr,
            shape:shape
        }
    }
    fn T(&mut self)->matrix2D{
        let newshape=[self.shape[1],self.shape[0]];
        let mut newarr:Vec<Vec<f32>> = Vec::new();
        for i in 0..self.shape[1]{
            let mut group:Vec<f32> = Vec::new();
            for j in 0..self.shape[0]{
                group.push(self.array[j][i]);
            }
            newarr.push(group);
        }
        matrix2D {
            array: newarr,
            shape: newshape
        }
    }
}


fn dot2D(a:matrix2D,b:matrix2D)->Result<matrix2D,&'static str>{
    //行列積
    if a.shape[1]!=b.shape[0]{
        Err("計算できない次元同士です")
    }else{
        let shape = [a.shape[0],b.shape[1]];
        let mut arr = Vec::new();
        for i in 0..b.shape[1]{
            let mut b_vec = Vec::new();
            for j in b.array.iter(){
                b_vec.push(j[i]);
            }
            let mut new_col:Vec<f32> = Vec::new();
            for j in a.array.iter(){
                new_col.push(inner_product2D(j.to_vec(), b_vec.clone()).unwrap())
            }
            arr.push(new_col);
        }
        Ok(matrix2D::new(arr, shape).T())
    }
}

fn add2D(a:matrix2D,b:matrix2D)->Result<matrix2D,&'static str>{
    //行列和
    if a.shape==b.shape{
        let mut newarr:Vec<Vec<f32>> = Vec::new();
        for (i,row) in a.array.iter().enumerate(){
            let mut newrow :Vec<f32>= Vec::new();
            for (j,num) in row.iter().enumerate(){
                newrow.push(num + b.array[i][j]);
            }
            newarr.push(newrow);
        }
        Ok(matrix2D { array: newarr, shape: a.shape })
    }else{
        Err("同じ次元にしてください")
    }
}

fn inner_product2D(a:Vec<f32>,b:Vec<f32>)->Result<f32,&'static str>{
    let mut i:f32=0.0;
    if a.len()==b.len(){
        for (index,elem) in a.iter().enumerate(){
            i+=elem*b[index];
        }
        return Ok(i);
    }else{
        Err("同じ次元にしてください")
    }
}

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
        let mut m0 = matrix2D::new(arr0,[2,3]);
        let mut m1 = matrix2D::new(arr1,[3,2]);
        let mut r0 = dot2D(m0, m1);
        
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
        
        let m2=matrix2D::new(arr2,[2,2]);
        let m3=matrix2D::new(arr3,[2,2]);
        let r1 = add2D(m2, m3);
        match r1{
            Ok(re)=>{println!("成功: {:?}",re.array)},
            Err(msg)=>{println!("{}",msg)}
        }
    }
}
