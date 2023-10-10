//誤差逆伝播法を利用した高速な勾配計算
use super::matrix::*;

pub struct Affine{
    pub weight  :matrix2D,//重み
    pub bias    :matrix2D,//バイアス
    //初期値はnoneにしたい
    pub x       :matrix2D,
    pub d_weight:matrix2D,
    pub d_bias  :matrix2D
}

impl Affine{
    pub fn new(w:matrix2D,b:matrix2D)->Self{
        let weight_shape = w.shape;
        let bias_shape = b.shape;
        Self{
            weight: w,
            bias: b,
            //以下の初期化については検討しなければいけない
            x: zeros(weight_shape),
            d_weight: zeros(weight_shape),
            d_bias: zeros(bias_shape)
        }
    }
    pub fn forword(&mut self,x:matrix2D) -> matrix2D{
        //予想の流れ
        self.x = x.clone();
        //unwrapが多すぎるので、学習途中で予期せぬことが起きたときのために後で直す
        return add2D(dot2D(x, self.weight.clone()).unwrap(),self.bias.clone()).unwrap()
    }
    pub fn backword(&mut self,dout:matrix2D)->matrix2D{
        //トレーニングの流れ
        self.d_weight = dot2D(dout.clone(), self.weight.T().clone()).unwrap();
        
        dot2D(dout, self.weight.T().clone()).unwrap()//dx
    }
}


//活性化関数
struct Relu{
    mask:Vec<Vec<bool>>,
    shape:[usize;2]
}

impl Relu{
    fn new(shape:[usize;2])->Self{
        //back forwordで入力されるshapeをあらかじめ設定しておく
        let mut mask:Vec<Vec<bool>> = Vec::new();
        for i in 0..shape[0]{
            let mut row:Vec<bool> = Vec::new();
            for j in 0..shape[1]{
                row.push(false)
            }
            mask.push(row);
        }
        Self{
            mask:mask,
            shape:shape
        }
    }
    fn forword(&mut self,x:matrix2D)->matrix2D{
        let mut arr:Vec<Vec<f32>>=Vec::new();
        for i in 0..self.shape[0]{
            let mut row:Vec<f32> = Vec::new();
            for j in 0..self.shape[1]{
                if x.array[i][j]<=0.0{
                    self.mask[i][j]=false;
                    row.push(0.0);//0以下ならば0にする
                }else{
                    self.mask[i][j]=true;
                    row.push(x.array[i][j]);
                }
            }
        }
        return matrix2D::new(arr, self.shape);
    }

    fn backword(&mut self,dout:matrix2D)->matrix2D{
        let mut newarr:Vec<Vec<f32>> = Vec::new();
        for i in 0..self.shape[0]{
            let mut row :Vec<f32>= Vec::new();
            for j in 0..self.shape[1]{
                if self.mask[i][j]{
                    row.push(dout.array[i][j]);
                }else{
                    row.push(0.0);
                }
            }
            newarr.push(row);
        }
        return matrix2D::new(newarr, self.shape);
    }
}