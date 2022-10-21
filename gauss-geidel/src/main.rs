fn main() {
    let a: [[f64; 3]; 3] = [[4.0, 0.24, -0.08], [0.09, 3.0, -0.15], [0.04, -0.08, 4.0]]; // equações
    let b: [f64; 3] = [8.0, 9.0, 20.0]; // termo independente
    let mut vet_inicial: [f64; 3] = [0.0, 0.0, 0.0]; // vetor inicial (chute)
    const interacao_max: usize = 5; // número de iterações

    loop {
        for i in 0..b.len() {
            x = b[i];
            for j in 0..b.len() {
                if i != j {
                    x -= a[i][j] * vet_inicial[j];
                }
            }
        }
    }
}
