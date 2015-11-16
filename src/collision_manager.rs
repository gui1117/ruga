use body::{ 
	Body, 
	BodyCollision,
	OverlapInformation,
};
use quadtree::Identifiable;

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

	(a_col,b_col)
}
