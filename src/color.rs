use crate::random::clamp;
use crate::vec3::Vec3;

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0. {
        return linear_component.sqrt();
    };
    0.
}

fn f64_to_u8_component(component: f64, apply_gamma: bool) -> u8 {
    let component = if apply_gamma {
        linear_to_gamma(component)
    } else {
        component
    };
    let intensity = 0.000..0.999;
    (256. * clamp(component, &intensity)) as u8
}

pub fn write_color(color: &Color) {
    let r = f64_to_u8_component(color.x, true);
    let g = f64_to_u8_component(color.y, true);
    let b = f64_to_u8_component(color.z, true);
    println!("{} {} {}", r, g, b);
}
