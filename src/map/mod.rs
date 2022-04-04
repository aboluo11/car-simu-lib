use crate::{Car, point2, SCALE, MAP_HEIGHT};
pub use parallel_parking::ParallelParking;
use usvg::{Node, NodeKind, Tree};

mod parallel_parking;

pub trait Map {
    fn car(&self) -> Car;
}

pub struct RightAngleTurn {
    pub svg: &'static str,
}

impl RightAngleTurn {
    pub fn new() -> Self {
        Self {
            svg: include_str!("../../res/map/直角转弯.svg")
        }
    }
}

impl Map for RightAngleTurn {
    fn car(&self) -> Car {
        let tree = usvg::Tree::from_str(
            self.svg,
            &usvg::Options::default().to_ref(),
        ).unwrap();
        let car_svg = tree.node_by_id("car").unwrap();
        let origin = match &*car_svg.borrow() {
            NodeKind::Image(image) => {
                let rect = image.view_box.rect;
                point2(
                    (rect.x() + rect.width()/2.)/SCALE,
                    MAP_HEIGHT - (rect.y() + rect.height()/2.)/SCALE,
                )
            }
            _ => {unreachable!()}
        };
        Car::new(origin, 0.)
    }
}
