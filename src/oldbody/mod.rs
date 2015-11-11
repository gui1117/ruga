mod geometry;

/// mask and types must be seen as arrays of bits, 
/// two body a and b can collide if (a.mask && b.types) || (b.mask && a.types)  == 0
struct body {
	mask: u32,
	types: u32,

	position: geometry::Point,
	velocity: f64,
	angle: f64,
	shape: geometry::Shape,
}

impl body {
	/// return whether the two body are colliding
	/// it consider mask.
	fn collide(&self, b: body) -> bool {
		if (self.mask & b.types) | (b.mask & self.types)  != 0 {
			return false;
		}
		self.overlapping(b)
	}

	/// return whether the two body are overlapping
	fn overlapping(&self, b: body) -> bool {
		true
	}

	/// update the position of the body considering its angle and velocity
	fn update(&mut self, dt: f64) {
		let ca = self.angle.cos();
		let sa = self.angle.sin();

		self.position.x += ca*self.velocity*dt;
		self.position.y += sa*self.velocity*dt;
	}
}
