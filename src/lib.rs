mod linear_algebra;
mod map;

use std::ops;

use linear_algebra::{Matrix, Vector2D};
pub use map::*;

pub static SCALE: f64 = 30.;
pub static MAP_WIDTH: f64 = 800./SCALE;
pub static MAP_HEIGHT: f64 = 800./SCALE;
const CAR_WIDTH: f64 = 1.837;
const CAR_HEIGHT: f64 = 4.765;
const LOGO_WIDTH: f64 = 1.0;
const WHEEL_WIDTH: f64 = 0.215;
const WHEEL_HEIGHT: f64 = WHEEL_WIDTH*0.55*2.+1./39.37*17.;
const TURNING_RADIUS: f64 = 5.5;
const TURNING_COUNT: i32 = 4;
const TRACK_WIDTH: f64 = 1.58;
const FRONT_SUSPENSION: f64 = 0.92;
const REAR_SUSPENSION: f64 = 1.05;
const MIRROR_WIDTH: f64 = 0.08;
const MIRROR_HEIGHT: f64 = 0.35;
const MIRROR_ANGLE: f64 = 70./180.*std::f64::consts::PI;
const MIRROR_ORIGIN_TO_FRONT: f64 = 1.55-MIRROR_WIDTH/2.;


fn new_rotation_matrix(angle: f64) -> Matrix<2, 2> {
    Matrix::new([
        [f64::cos(angle), -f64::sin(angle)],
        [f64::sin(angle), f64::cos(angle)],
    ])
}

#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn rotate(&self, rotation: Rotation) -> Point {
        rotation.rotation_matrix * (*self-rotation.origin) + rotation.origin
    }

    fn forward(&self, distance: f64, rotation_matrix: Matrix<2, 2>) -> Point {
        point2(self.x, self.y+distance).rotate(Rotation {
            rotation_matrix,
            origin: *self,
        })
    }

    fn translate(&self, translation: Vector2D) -> Point {
        *self + translation
    }
}

impl From<(f64, f64)> for Point {
    fn from(p: (f64, f64)) -> Self {
        point2(p.0, p.1)
    }
}

impl From<Point> for (f64, f64) {
    fn from(p: Point) -> Self {
        (p.x, p.y)
    }
}

impl From<Vector2D> for Point {
    fn from(p: Vector2D) -> Self {
        point2(p.x(), p.y())
    }
}

impl From<Point> for Vector2D {
    fn from(p: Point) -> Self {
        Vector2D::new_from_x_and_y(p.x, p.y)
    }
}

impl ops::Sub<Point> for Point {
    type Output = Vector2D;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector2D::from(self) - Vector2D::from(rhs)
    }
}

impl ops::Add<Vector2D> for Point {
    type Output = Point;

    fn add(self, rhs: Vector2D) -> Self::Output {
        (Vector2D::from(self) + rhs).into()
    }
}

impl ops::Add<Point> for Vector2D {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        rhs + self
    }
}


fn point2(x: f64, y: f64) -> Point {
    Point {x, y}
}

fn distance_of(x: Point, y: Point) -> f64 {
    let a = x - y;
    f64::sqrt(a.x()*a.x() + a.y()*a.y())
}


#[derive(Clone, Copy)]
struct Rotation {
    rotation_matrix: Matrix<2, 2>,
    origin: Point,
}

impl Rotation {
    fn new(angle: f64, origin: Point) -> Rotation {
        Rotation {
            rotation_matrix: new_rotation_matrix(angle),
            origin
        }
    }
}

enum Source {
    Color(Color),
    Image(Vec<u32>),
}

pub struct Rect {
    pub origin: Point,
    pub width: f64,
    pub height: f64,
    rotation_matrix: Matrix<2,2>,
    source: Source,
}

impl Rect {
    fn new(origin: Point, width: f64, height: f64, source: Source) -> Rect {
        Rect {
            origin,
            width,
            height,
            rotation_matrix: Matrix::<2, 2>::eye(),
            source,
        }
    }

    pub fn lt(&self) -> Point {
        point2(self.origin.x - self.width/2., self.origin.y + self.height/2.)
            .rotate(Rotation { rotation_matrix: self.rotation_matrix, origin: self.origin })
    }

    pub fn rt(&self) -> Point {
        point2(self.origin.x + self.width/2., self.origin.y + self.height/2.)
            .rotate(Rotation { rotation_matrix: self.rotation_matrix, origin: self.origin })
    }

    pub fn lb(&self) -> Point {
        point2(self.origin.x - self.width/2., self.origin.y - self.height/2.)
            .rotate(Rotation { rotation_matrix: self.rotation_matrix, origin: self.origin })
    }

    pub fn rb(&self) -> Point {
        point2(self.origin.x + self.width/2., self.origin.y - self.height/2.)
            .rotate(Rotation { rotation_matrix: self.rotation_matrix, origin: self.origin })
    }

    fn rotate_self(&mut self, rotation_matrix: Matrix<2,2>) {
        self.rotation_matrix = rotation_matrix * self.rotation_matrix;
    }

    fn rotate(&mut self, rotation: Rotation) {
        self.origin = self.origin.rotate(rotation);
        self.rotate_self(rotation.rotation_matrix);
    }

    fn forward(&mut self, distance: f64, rotation_matrix: Matrix<2,2>) {
        self.origin = self.origin.forward(distance, rotation_matrix);
    }
}

fn new_logo(path: &std::path::Path, origin: Point, width: f64) -> Rect {
    let svg = usvg::Tree::from_data(&std::fs::read(path).unwrap(), &usvg::Options::default().to_ref()).unwrap();
    let (svg_ori_width, svg_ori_height) = (svg.svg_node().size.width(), svg.svg_node().size.height());
    let height = (svg_ori_height/svg_ori_width) as f64 * width;
    let mut pixmap = tiny_skia::Pixmap::new((width*SCALE) as u32, (height*SCALE) as u32).unwrap();
    resvg::render(&svg, usvg::FitTo::Width((width*SCALE) as u32), pixmap.as_mut()).unwrap();
    let mut data = vec![];
    for chunk in pixmap.data().chunks(4) {
        if let &[r, g, b, a] = chunk {
            data.push(u32::from_be_bytes([a, r, g, b]));
        }
    }
    Rect::new(origin, width, height, Source::Image(data))
}

pub struct Car {
    pub lt: Rect,
    pub rt: Rect,
    pub lb: Rect,
    pub rb: Rect,
    pub body: Rect,
    steer_angle: i32,
    logo: Rect,
    left_mirror: Rect,
    right_mirror: Rect,
}

impl Car {
    fn new(body_origin: Point, angle: f64) -> Car {
        let body_color = Color {r: 24, g: 174, b: 219};
        let wheel_color = Color {r: 0, g: 0, b: 0};
        let mut body = Rect::new(body_origin, CAR_WIDTH, CAR_HEIGHT, Source::Color(body_color));
        let mut lt = Rect::new(point2(body.origin.x-TRACK_WIDTH/2., CAR_HEIGHT/2.+body.origin.y-FRONT_SUSPENSION),
        WHEEL_WIDTH, WHEEL_HEIGHT, Source::Color(wheel_color));
        let mut rt = Rect::new(point2(body.origin.x+TRACK_WIDTH/2., CAR_HEIGHT/2.+body.origin.y-FRONT_SUSPENSION),
        WHEEL_WIDTH, WHEEL_HEIGHT, Source::Color(wheel_color));
        let mut lb = Rect::new(point2(body.origin.x-TRACK_WIDTH/2., -CAR_HEIGHT/2.+body.origin.y+REAR_SUSPENSION),
        WHEEL_WIDTH, WHEEL_HEIGHT, Source::Color(wheel_color));
        let mut rb = Rect::new(point2(body.origin.x+TRACK_WIDTH/2., -CAR_HEIGHT/2.+body.origin.y+REAR_SUSPENSION),
        WHEEL_WIDTH, WHEEL_HEIGHT, Source::Color(wheel_color));
        let mut logo = new_logo(
            std::path::Path::new("res/tesla.svg"),
            point2(body_origin.x, body_origin.y+CAR_HEIGHT/2.-0.2),
            LOGO_WIDTH,
        );
        logo.origin.y -= logo.height/2.;
        let mut left_mirror = Rect::new(
            point2(
                body_origin.x-CAR_WIDTH/2.-MIRROR_HEIGHT/2.,
                body_origin.y+CAR_HEIGHT/2.-MIRROR_ORIGIN_TO_FRONT,
            ), MIRROR_WIDTH, MIRROR_HEIGHT, Source::Color(body_color));
        let mut right_mirror = Rect::new(
            point2(
                body_origin.x+CAR_WIDTH/2.+MIRROR_HEIGHT/2.,
                body_origin.y+CAR_HEIGHT/2.-MIRROR_ORIGIN_TO_FRONT,
            ), MIRROR_WIDTH, MIRROR_HEIGHT, Source::Color(body_color));
        left_mirror.rotate_self(new_rotation_matrix(std::f64::consts::PI/2.));
        right_mirror.rotate_self(new_rotation_matrix(std::f64::consts::PI/2.));
        left_mirror.rotate(Rotation::new(std::f64::consts::PI/2.-MIRROR_ANGLE, left_mirror.rb()));
        right_mirror.rotate(Rotation::new(-(std::f64::consts::PI/2.-MIRROR_ANGLE), right_mirror.rt()));
        let rotation = Rotation::new(angle, body_origin);
        body.rotate(rotation);
        lt.rotate(rotation);
        rt.rotate(rotation);
        lb.rotate(rotation);
        rb.rotate(rotation);
        logo.rotate(rotation);
        left_mirror.rotate(rotation);
        right_mirror.rotate(rotation);
        Car {
            lt, rt, lb, rb, body, steer_angle: 0, logo, left_mirror, right_mirror
        }
    }

    fn angle_matrix(&self, r: f64) -> Matrix<2, 2> {
        let c = f64::sqrt(r*r + self.L()*self.L());
        Matrix { inner: [
            [r/c, -self.L()/c],
            [self.L()/c, r/c],
        ] }
    }

    fn small_angle_matrix(&self, r: f64) -> Matrix<2,2> {
        self.angle_matrix(r+self.T()/2.)
    }

    fn big_angle_matrix(&self, r: f64) -> Matrix<2,2> {
        self.angle_matrix(r-self.T()/2.)
    }

    fn top2_angle_matrix(&self, o: Option<Point>) -> (Matrix<2,2>, Matrix<2,2>) {
        match o {
            Some(o) => {
                let r = distance_of(self.back_origin(), o);
                if distance_of(self.lt.origin, o) < distance_of(self.rt.origin, o) {
                    (self.big_angle_matrix(r), self.small_angle_matrix(r))
                } else {
                    (self.small_angle_matrix(r).inverse().unwrap(), self.big_angle_matrix(r).inverse().unwrap())
                }
            },
            None => {
                (Matrix::<2, 2>::eye(), Matrix::<2, 2>::eye())
            }
        }
    }

    fn steer(&mut self) {
        let o_new = self.angle2origin(self.steer_angle);
        let (lt, rt) = self.top2_angle_matrix(o_new);
        self.lt.rotation_matrix = lt * self.body.rotation_matrix;
        self.rt.rotation_matrix = rt * self.body.rotation_matrix;
    }

    fn forward(&mut self, distance: f64) {
        let o = self.angle2origin(self.steer_angle);
        if let Some(o) = o {
            let angle = distance/distance_of(self.top_origin(), o) 
                * (if self.steer_angle > 0 {1.} else {-1.});
            let rotation = Rotation::new(angle, o);
            self.lt.rotate(rotation);
            self.rt.rotate(rotation);
            self.lb.rotate(rotation);
            self.rb.rotate(rotation);
            self.body.rotate(rotation);
            self.logo.rotate(rotation);
            self.left_mirror.rotate(rotation);
            self.right_mirror.rotate(rotation);
        } else {
            let rotation_matrix = self.body.rotation_matrix;
            self.lt.forward(distance, rotation_matrix);
            self.rt.forward(distance, rotation_matrix);
            self.lb.forward(distance, rotation_matrix);
            self.rb.forward(distance, rotation_matrix);
            self.body.forward(distance, rotation_matrix);
            self.logo.forward(distance, rotation_matrix);
            self.left_mirror.forward(distance, rotation_matrix);
            self.right_mirror.forward(distance, rotation_matrix);
        }
    }

    fn L(&self) -> f64 {
        distance_of(self.lt.origin, self.lb.origin)
    }

    fn T(&self) -> f64 {
        distance_of(self.lb.origin, self.rb.origin)
    }

    fn back_origin(&self) -> Point {
        point2((self.lb.origin.x+self.rb.origin.x)/2., (self.lb.origin.y+self.rb.origin.y)/2.)
    }

    fn top_origin(&self) -> Point {
        point2((self.lt.origin.x+self.rt.origin.x)/2., (self.lt.origin.y+self.rt.origin.y)/2.)
    }

    fn angle2origin(&self, angle: i32) -> Option<Point> {
        match self.angle2r(angle) {
            Some(r) => {
                let back_origin = self.back_origin();
                let origin_before_trans = point2(back_origin.x-r, back_origin.y);
                Some(origin_before_trans.rotate(Rotation {
                    rotation_matrix: self.body.rotation_matrix,
                    origin: back_origin,
                }))
            },
            None => None
        }
    }

    fn angle2r(&self, angle: i32) -> Option<f64> {
        // angle>0: 向左转, r>0; angle<0: 向右转, r<0;
        if angle == 0 {
            None
        } else {
            Some(
                (TURNING_COUNT as f64)*(f64::sqrt(TURNING_RADIUS*TURNING_RADIUS-self.L()*self.L())-self.T()/2.)
                    /(angle as f64)
            )
        }
    }

    fn left_steer(&mut self) {
        if self.steer_angle < TURNING_COUNT {
            self.steer_angle += 1;
            self.steer();
        }
    }

    fn right_steer(&mut self) {
        if self.steer_angle > -TURNING_COUNT {
            self.steer_angle -= 1;
            self.steer();
        }
    }
}
