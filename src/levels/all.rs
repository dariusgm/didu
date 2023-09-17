use super::level_1::level_1;
use super::level_2::level_2;
use super::level_3::level_3;
use super::level_4::level_4;
use super::level_5::level_5;
use crate::utils::level::Level;

pub fn levels() -> Vec<Level> {
    vec![level_1(), level_2(), level_3(), level_4(), level_5()]
}
