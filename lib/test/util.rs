pub fn from_slice_16(bytes: &[f64]) -> [f64; 16] {
    let mut array = [0.; 16];
    let bytes = &bytes[..array.len()];
    array.copy_from_slice(bytes);
    array
}
pub fn from_slice_9(bytes: &[f64]) -> [f64; 9] {
    let mut array = [0.; 9];
    let bytes = &bytes[..array.len()];
    array.copy_from_slice(bytes);
    array
}
pub fn from_slice_3(bytes: &[f64]) -> [f64; 3] {
    let mut array = [0.; 3];
    let bytes = &bytes[..array.len()];
    array.copy_from_slice(bytes);
    array
}
pub fn from_slice_4(bytes: &[f64]) -> [f64; 4] {
    let mut array = [0.; 4];
    let bytes = &bytes[..array.len()];
    array.copy_from_slice(bytes);
    array
}
