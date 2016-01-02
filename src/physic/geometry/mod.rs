pub mod primitive;
pub mod shape;






















#[cfg(test)]
use std;
#[cfg(test)]
use self::primitive::Point;
#[cfg(test)]
use self::shape::Shape;

#[test]
fn point_angle() {
	use std::f64::consts::PI;

	let a = Point { x: 0., y: 0.};
	let b = Point { x: 1., y: 1.};
	let c = Point { x: 0., y: 1.};
	let pi = std::f64::consts::PI;
	assert_eq!(Point::angle(&a,&b,&c), -pi/4.);

	let a = Point { x: -10., y: -10.};
	let b = Point { x: 1., y: 1.};
	let c = Point { x: -1., y: 1.};
	let pi = std::f64::consts::PI;
	assert_eq!(Point::angle(&a,&b,&c), -pi/4.);

	let a = Point { x: 0., y: 0.};
	let b = Point { x: -1., y: 0.};
	let c = Point { x: -2., y: -1.};
	let pi = std::f64::consts::PI;
	assert_eq!(Point::angle(&a,&b,&c), -3./4.*pi);

	let a = Point { x: 0., y: 0.};
	let b = Point { x: -1., y: -1.};
	let c = Point { x: -1., y: -2.};
	let pi = std::f64::consts::PI;
	assert_eq!(Point::angle(&a,&b,&c), -3./4.*pi);

	let a = Point { x: 1., y: 1.};
	let b = Point { x: 1., y: 5.};
	let c = Point { x: 11., y: 5.};
	let pi = std::f64::consts::PI;
	assert_eq!(Point::angle(&a,&b,&c), pi/2.);

	let a = Point { x: 1., y: 1. };
	assert_eq!(a.angle_0x(),PI/4.);
}

#[test]
#[should_panic]
fn new_wrong_shape_1() {
	//one edge
	let p1 = Point{ x: 0.0, y: 0.0 };
	let _s = Shape::new(0.,0.,0.,vec![p1]).unwrap();
}
	
#[test]
#[should_panic]
fn new_wrong_shape_2() {
	//two edge
	let p1 = Point{ x: 0.0, y: 0.0 };
	let p2 = Point{ x: 0.0, y: 0.0 };
	let _s = Shape::new(0.,0.,0.,vec![p1,p2]).unwrap();
}

#[test]
#[should_panic]
fn new_wrong_shape_3() {
	//all edges aligned
	let p1 = Point{ x: 0., y: 0. };
	let p2 = Point{ x: 1., y: 1. };
	let p3 = Point{ x: 10., y: 10. };
	let p4 = Point{ x: -10., y: -10. };
	let p5 = Point{ x: 12., y: 12. };
	let p6 = Point{ x: 15., y: 15. };
	let _s = Shape::new(0.,0.,0.,vec![p1,p2,p3,p4,p5,p6]).unwrap();
}

#[test]
fn new_shape() {
	//standard
	let p1 = Point{ x: 0., y: 0. };
	let p2 = Point{ x: 1., y: 1. };
	let p3 = Point{ x: 0., y: 1. };
	let s = Shape::new(0.,0.,0.,vec![p1,p2,p3]).unwrap();

	assert_eq!(s.edges.len(), 3);
	assert_eq!(s.normals.len(), 3);
	assert_eq!(s.normals[0],-std::f64::consts::PI);
	assert_eq!(s.normals[1],-std::f64::consts::PI/4.);
	assert_eq!(s.normals[2],std::f64::consts::PI/2.);

	//with same edges
	let p0 = Point{ x: 0., y: 0. };
	let p1 = Point{ x: 0., y: 0. };
	let p2 = Point{ x: 1., y: 1. };
	let p3 = Point{ x: 1., y: 1. };
	let p4 = Point{ x: 1., y: 1. };
	let p5 = Point{ x: 0., y: 1. };
	let s = Shape::new(0.,0.,0.,vec![p0,p1,p2,p3,p4,p5]).unwrap();

	assert_eq!(s.edges.len(), 3);
}

#[test]
fn shape_overlap() {
	use std::f64::consts::PI;

	let p1 = Point{ x: 0., y: 0. };
	let p2 = Point{ x: 1., y: 1. };
	let p3 = Point{ x: 0., y: 1. };
	let s1 = Shape::new(0.,0.,0.,vec![p1.clone(),p2.clone(),p3.clone()]).unwrap();
	let s2 = Shape::new(2.,0.,0.,vec![p1.clone(),p2.clone(),p3.clone()]).unwrap();
	let s3 = Shape::new(0.5,0.,0.,vec![p1.clone(),p2.clone(),p3.clone()]).unwrap();
	let s4 = Shape::new(1.1,1.,PI,vec![p1.clone(),p2.clone(),p3.clone()]).unwrap();
	let s5 = Shape::new(0.9,1.,PI,vec![p1.clone(),p2.clone(),p3.clone()]).unwrap();

	let (o,_,_) = Shape::overlap(&s1,&s2);
	assert!(!o);

	let (o,_,_) = Shape::overlap(&s1,&s3);
	assert!(o);

	let (o,_,_) = Shape::overlap(&s1,&s4);
	assert!(!o);

	let (o,_,_) = Shape::overlap(&s1,&s5);
	assert!(o);

	//let (o,_,_) = Shape::overlap(&s1,2.,0.,0.,&s2);
	//let (o,_,_) = Shape::overlap(&s1,0.5,0.,0.,&s3);
	//let (o,_,_) = Shape::overlap(&s1,1.1,1.,PI,&s4);
	//let (o,_,_) = Shape::overlap(&s1,0.9,1.,PI,&s5);
}

#[test]
fn point_in_shape() {
	let p1 = Point{ x: 0., y: 0. };
	let p2 = Point{ x: 1., y: 1. };
	let p3 = Point{ x: 0., y: 1. };
	let s1 = Shape::new(0.,0.,0.,vec![p1,p2,p3]).unwrap();

	let p1 = Point { x: 0.1, y: 0.5 };
	assert_eq!(p1.in_shape(&s1),true);

	let p1 = Point { x: 0.5, y: 0.6 };
	assert_eq!(p1.in_shape(&s1),true);

	let p1 = Point { x: 0.6, y: 0.7 };
	assert_eq!(p1.in_shape(&s1),true);

	let p1 = Point { x: 0.5, y: 0.1 };
	assert_eq!(p1.in_shape(&s1),false);

	let p1 = Point { x: 0.9, y: 0.1 };
	assert_eq!(p1.in_shape(&s1),false);
}
