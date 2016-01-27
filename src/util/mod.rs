#[macro_use]
mod delegate;

#[macro_use]
mod drawer;

pub fn grid_raycast(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<[i32;2]> {
    if x0 == x1 {
        let x0_i32 = x0 as i32;
        let y0_i32 = y0 as i32;
        let y1_i32 = y1 as i32;
        let mut vec = Vec::new();

        if y0 > y1 {
            for y in y1_i32..y0_i32+1 {
                vec.push([x0_i32,y]);
            }
            vec.reverse();
        } else {
            for y in y0_i32..y1_i32+1 {
                vec.push([x0_i32,y]);
            }
        }

        vec
    } else if x0 > x1 {
        let mut vec = grid_raycast(x1,y1,x0,y0);
        vec.reverse();
        vec
    } else {
        let x0_i32 = x0 as i32;
        let y0_i32 = y0 as i32;
        let x1_i32 = x1 as i32;

        // equation y = ax + b
        let a = (y1 - y0)/(x1 - x0);
        let b = y0 -a*x0;
        //println!("a:{} b:{}",a,b);

        let delta_error = a.abs();

        let signum; 
        if a > 0. {
            signum = 1;
        } else {
            signum = -1;
        }
        let mut error = -(y0.floor() - (a*x0.floor() + b)).abs();
        let mut error_end = (y1 - (a*x1.ceil() + b)).abs();
        //println!("debut: error: {}",error);
        //println!("error end : {}", error_end);

        let mut vec = Vec::new();
        let mut y = y0_i32;

        for x in x0_i32..x1_i32+1 {
            vec.push([x,y]);
            error += delta_error;
            //println!("error: {}",error);
            while error >= 1.0 {
                y += signum;
                error -= 1.0;
                //println!("error -= 1.0: {}",error);
                vec.push([x,y]);
            }
        }
        while error_end >= 0. {
            vec.pop();
            error_end -= 1.0;
        }

        vec
    }
}

//#[test]
//fn test_grid_raycast() {
//    use rand;
//
//    let x0 = 0.;
//    let y0 = 0.;
//    let x1 = 1.;
//    let y1 = 1.;
//    let line_vec = grid_raycast(x0,y0,x1,y1);
//    //TODO
//}

/// the coordinate of the intersections (if some) of a rectangle of center (x,y) width and height,
/// and the line of equation ax+by+c=0
pub fn bounding_box_raycast(x: f64, y: f64, width: f64, height: f64, a: f64, b: f64, c: f64) -> Option<(f64,f64,f64,f64)> {
    if a == 0. && b == 0. {
        None
    } else if a == 0. {
        let y_proj = -c/b;
        if y - height/2. <= y_proj && y_proj <= y + height/2. {
            Some((x-width/2.,y_proj,x+width/2.,y_proj))
        } else {
            None
        }
    } else if b == 0. {
        let x_proj = -c/a;
        if x - width/2. <= x_proj && x_proj <= x + width/2. {
            Some((x_proj,y-height/2.,x_proj,y+height/2.))
        } else {
            None
        }
    } else {
        //println!("x:{}, y:{}, width:{}, height:{}, a:{}, b:{}, c:{}",x,y,width,height,a,b,c);
        // the ordonate of the point that is on the line(a,b) and the horizontal line that cut (x,y)
        let y_proj = -(a*x + c)/b; 
        // the abscisse of the point that is on the line(a,b) and the vertical line that cut (x,y)
        let x_proj = -(b*y+c)/a; 

        //println!("proj: {:?} | {:?}",x_proj,y_proj);
        // i,j,k,l are three point:
        // * i represent the point on the horizontal line on the top of the bounding box
        // and on the line(a,b)
        // * j represent the point on the vertical line on the right of the bounding box
        // and on the line(a,b)
        // * k represent the point on the horizontal line on the bottom of the bounding box
        // and on the line(a,b)
        // * l represent the point on the vertical line on the left of the bounding box
        // and on the line(a,b)

        // dy = -a/b * dx

        let dx = -height/2. * b/a;
        //println!("dx: {:?}",dx);
        let x_i = x_proj + dx;
        let y_i = y + height/2.;
        let x_k = x_proj - dx;
        let y_k = y - height/2.;
        //println!("i: {:?} | {:?}",x_i,y_i);
        //println!("k: {:?} | {:?}",x_k,y_k);

        let dy = -width/2. * a/b;
        //println!("dy: {:?}",dy);
        let x_j = x + width/2.;
        let y_j = y_proj + dy;
        let x_l = x - width/2.;
        let y_l = y_proj - dy;
        //println!("j: {:?} | {:?}",x_j,y_j);
        //println!("l: {:?} | {:?}",x_l,y_l);


        let cond_i = x-width/2. < x_i && x_i < x+width/2.;
        let cond_k = x-width/2. < x_k && x_k < x+width/2.;
        let cond_j = y-width/2. < y_j && y_j < y+width/2.;
        let cond_l = y-width/2. < y_l && y_l < y+width/2.;

        if cond_i && cond_k {
            if x_i < x_k {
                Some((x_i,y_i,x_k,y_k))
            } else {
                Some((x_k,y_k,x_i,y_i))
            }
        } else if cond_i && cond_j {
            if x_i < x_j {
                Some((x_i,y_i,x_j,y_j))
            } else {
                Some((x_j,y_j,x_i,y_i))
            }
        } else if cond_i && cond_l {
            if x_i < x_l {
                Some((x_i,y_i,x_l,y_l))
            } else {
                Some((x_l,y_l,x_i,y_i))
            }
        } else if cond_j && cond_k {
            if x_j < x_k {
                Some((x_j,y_j,x_k,y_k))
            } else {
                Some((x_k,y_k,x_j,y_j))
            }
        } else if cond_j && cond_l {
            if x_j < x_l {
                Some((x_j,y_j,x_l,y_l))
            } else {
                Some((x_l,y_l,x_j,y_j))
            }
        } else if cond_k && cond_l {
            if x_k < x_l {
                Some((x_k,y_k,x_l,y_l))
            } else {
                Some((x_l,y_l,x_k,y_k))
            }
        } else {
            None
        }
    }
}

#[test]
fn test_bounding_box_raycast() {
    // for a == 0
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., 0., 0., -1.));
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., 0., 1., -1.));
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., 0., -1./0.5, -1.));
    assert_eq!(Some((-4.,-1.,2.,-1.)),bounding_box_raycast( -1., -2., 6., 2., 0., -1./1., -1.));
    assert_eq!(Some((-4.,-2.,2.,-2.)),bounding_box_raycast( -1., -2., 6., 2., 0., -1./2., -1.));
    assert_eq!(Some((-4.,-3.,2.,-3.)),bounding_box_raycast( -1., -2., 6., 2., 0., -1./3., -1.));
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., 0., -1./3.5, -1.));
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., 0., -1./4., -1.));
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., 0., -1./4.5, -1.));

    // for b == 0
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., -1./4.5, 0., -1.));
    assert_eq!(Some((-4.,-3.,-4.,-1.)),bounding_box_raycast( -1., -2., 6., 2., -1./4., 0., -1.));
    assert_eq!(Some((0.,-3.,0.,-1.)),bounding_box_raycast( -1., -2., 6., 2., 1., 0., 0.));
    assert_eq!(Some((2.,-3.,2.,-1.)),bounding_box_raycast( -1., -2., 6., 2., 1./2., 0., -1.));
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., 1./2.5, 0., -1.));

    // for b != 0 && a != 0
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., -1., -1., 9.));
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., 1., 1., 7.));
    assert_eq!(Some((-4.,-2.,-3.,-3.)),bounding_box_raycast( -1., -2., 6., 2., 1., 1., 6.));
}

