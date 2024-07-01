pub mod vec3;
pub mod color;
pub mod ray;
pub mod camera;
pub mod hit;
pub mod random;
pub mod materials;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
