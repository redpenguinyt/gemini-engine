pub fn interpolate(i0: isize, d0: f64, i1: isize, d1: f64) -> Vec<isize> {
    if i0 == i1 {
        return vec![d0.round() as isize];
    }
    let mut values = vec![];

    let a = (d1 - d0) / (i1 - i0) as f64;
    let mut d = d0;
    for _i in i0..(i1 + 1) {
        values.push(d.clone().round() as isize);
        d += a;
    }
    values
}