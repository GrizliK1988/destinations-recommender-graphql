pub(crate) fn vector_norm(vector: &[f64]) -> f64 {
    let mut norm = 0.0f64;

    for el in vector.iter() {
        norm += el * el;
    }

    norm.sqrt()
}

pub(crate) fn normalize_vector(vector: &mut [f64]) -> () {
    let norm = vector_norm(vector);

    for key in 0..vector.len() {
        vector[key] = vector[key] / norm;
    }
}

pub(crate) fn dot_product(v1: &[f64], v2: &[f64]) -> f64 {
    let mut sum = 0.0f64;

    for key in 0..v1.len() {
        sum += v1[key] * v2[key];
    }

    sum
}
