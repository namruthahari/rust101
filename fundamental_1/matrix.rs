use std::fmt;
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix{
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:?}\n{:?}",(self.0,self.1),(self.2,self.3))
    }
}
fn reverse(pair: (f32, f32)) -> (f32, f32) {
    let (a, b) = pair;
    (b, a)
}
fn transpose(mat: Matrix) -> Matrix {
    let (a, b) = reverse((mat.1, mat.2));
    Matrix(mat.0, a, b, mat.3)
}
fn main(){
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}