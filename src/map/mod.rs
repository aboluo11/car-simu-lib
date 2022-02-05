use crate::Car;
pub use parallel_parking::ParallelParking;

mod parallel_parking;

trait Map {
    fn car(&self) -> Car;
}
