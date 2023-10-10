pub struct matrix2D{
    pub array:Vec<Vec<f32>>,
    pub shape:[usize;2]
}

impl matrix2D {
    pub fn new(arr:Vec<Vec<f32>>,shape:[usize;2])->Self{
        Self{
            array:arr,
            shape:shape
        }
    }
    pub fn T(&mut self)->matrix2D{
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


pub fn dot2D(a:matrix2D,b:matrix2D)->Result<matrix2D,&'static str>{
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

pub fn add2D(a:matrix2D,b:matrix2D)->Result<matrix2D,&'static str>{
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

pub fn inner_product2D(a:Vec<f32>,b:Vec<f32>)->Result<f32,&'static str>{
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
