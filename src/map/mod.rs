use crate::Car;

pub mod parallel_parking;

trait Map {
    fn car(&self) -> Car;
}
