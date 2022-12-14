use crate::collider::Collider;
use crate::vec3::Vec3;
use crate::transform::Transform;

pub struct Sphere {
	pub pos: Vec3,
	pub radius: f32
}

impl Sphere {

    pub fn pos(&self) -> Vec3 {
        self.pos
    }

    pub fn radius(&self) -> f32 {
        self.radius 
    }

    pub fn transform(&mut self, transform: &Transform) {
        self.pos = Transform::apply(self.pos, transform, self.pos);
    }

    /*
    pub fn bounding_sphere(&self) -> Shape {
        self
    }
    */

	pub fn inv_inertia(&self, inv_m: f32) -> [[f32; 3]; 3] {
		let r2 = self.radius.powf(2.0);

		[[2.5*inv_m/r2, 0.0, 0.0],
		 [0.0, 2.5*inv_m/r2, 0.0],
		 [0.0, 0.0, 2.5*inv_m/r2]]
	}

    pub fn furthest_point(&self, direction: Vec3) -> Vec3 {
        self.pos + direction * self.radius
    }

}
