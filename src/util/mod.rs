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
        println!("a:{} b:{}",a,b);

        let delta_error = a.abs();

        let signum; 
        if a > 0. {
            signum = 1;
        } else {
            signum = -1;
        }
        let mut error = -(y0.floor() - (a*x0.floor() + b)).abs();
        let mut error_end = (y1 - (a*x1.ceil() + b)).abs();
        println!("debut: error: {}",error);
        println!("error end : {}", error_end);

        let mut vec = Vec::new();
        let mut y = y0_i32;

        for x in x0_i32..x1_i32+1 {
            vec.push([x,y]);
            error += delta_error;
            println!("error: {}",error);
            while error >= 1.0 {
                y += signum;
                error -= 1.0;
                println!("error -= 1.0: {}",error);
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

#[test]
fn test_grid_raycast() {
    use rand;

    let x0 = 0.;
    let y0 = 0.;
    let x1 = 1.;
    let y1 = 1.;
    let line_vec = grid_raycast(x0,y0,x1,y1);
    //TODO
}

pub fn bounding_box_raycast(x: f64, y: f64, width: f64, height: f64, a: f64, b : f64) -> Option<(f64,f64,f64,f64)> {
    //println!("x:{}, y:{}, width:{}, height:{}, a:{}, b:{}",x,y,width,height,a,b);
    let y_proj = a*x + b;
    let x_proj = (y-b)/a;

    //println!("proj: {:?} | {:?}",x_proj,y_proj);
    // points noted from the top to the left

    let dx = height/2./a;
    //println!("dx: {:?}",dx);
    let x_i = x_proj + dx;
    let y_i = y + height/2.;
    let x_k = x_proj - dx;
    let y_k = y - height/2.;
    //println!("i: {:?} | {:?}",x_i,y_i);
    //println!("k: {:?} | {:?}",x_k,y_k);

    let dy = width/2. * a;
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

#[test]
fn test_bounding_box_raycast() {
    println!("{:?}",bounding_box_raycast( 5., 10., 10., 20., -2.0, 30.));
    assert!(false);
}

