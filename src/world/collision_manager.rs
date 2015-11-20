use std::f64::consts::PI;
use world::body::{ 
	Body, 
	BodyCollision,
	CollisionType,
	OverlapInformation,
};

pub fn collision(a: &Body, b: &Body, info: OverlapInformation) -> (BodyCollision, BodyCollision) {
	let mut a_col = BodyCollision::new();
	let mut b_col = BodyCollision::new();

	let delta_x = info.length*info.angle.cos();
	let delta_y = info.length*info.angle.sin();

	let mut rate = a.weight()/(a.weight()+b.weight());
	if rate.is_nan() {
		rate = 1.;
	}

	a_col.delta_x = -(1.-rate)*delta_x;
	a_col.delta_y = -(1.-rate)*delta_y;

	b_col.delta_x = rate*delta_x;
	b_col.delta_y = rate*delta_y;

	match a.collision_type {
		CollisionType::Bounce => {
			a_col.delta_angle = 2.*info.angle - 2.*a.angle() + PI;
		},
		CollisionType::Stop => {
			a_col.delta_velocity = -a.velocity();
		},
		CollisionType::Persist => (),
	}

	match b.collision_type {
		CollisionType::Bounce => {
			b_col.delta_angle = 2.*info.angle - 2.*b.angle() + PI 
		},
		CollisionType::Stop => {
			b_col.delta_velocity = -b.velocity();
		},
		CollisionType::Persist => (),
	}

	(a_col,b_col)
}
