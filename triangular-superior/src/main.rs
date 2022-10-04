// sistema triangular superior

fn triangular_superior() {
    let a = [[1.0, 0.0, 0.0], [2.0, 3.0, 0.0], [4.0, 5.0, 6.0]];
    let b = [1.0, 2.0, 3.0];
    let mut x = [0.0, 0.0, 0.0];
    let n = 3;
    let mut i = 0;
    let mut j = 0;
    while i < n {
        let mut soma = 0.0;
        j = 0;
        while j < i {
            soma = soma + a[i][j] * x[j];
            j = j + 1;
        }
        x[i] = (b[i] - soma) / a[i][i];
        i = i + 1;
    }
    println!("x = {:?}", x);
}

fn triangular_inferior() {
    let a = [[1.0, 2.0, 4.0], [0.0, 3.0, 5.0], [0.0, 0.0, 6.0]];
    let b = [1.0, 2.0, 3.0];
    let mut x = [0.0, 0.0, 0.0];
    let n = 3;
    x[0] = b[0] / a[0][0];

    for i in 2..n {
        let mut soma = b[i];
        for j in (1..i) {
            soma = soma - a[i][j] * x[j];
        }
        x[i] = soma / a[i][i];
    }
    println!("x = {:?}", x);
}

fn main() {
    triangular_superior();
    triangular_inferior();

}
