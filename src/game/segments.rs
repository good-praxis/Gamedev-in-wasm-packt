use crate::engine::{Image, Point, Rect, SpriteSheet};
use crate::game::obstacles::{Barrier, Obstacle, Platform};
use std::rc::Rc;
use web_sys::HtmlImageElement;

const LOW_PLATFORM: i16 = 420;
const HIGH_PLATFORM: i16 = 375;
const INITIAL_OBSTACLE_OFFSET: i16 = 150;
const FIRST_PLATFORM: i16 = 400;
const STONE_HEIGHT: i16 = 54;
const STONE_ON_GROUND: i16 = 546;
const STONE_ON_LOW_PLATFORM: i16 = LOW_PLATFORM - STONE_HEIGHT;

const FLOATING_PLATFORM_SPRITES: [&str; 3] = ["13.png", "14.png", "15.png"];
const FLOATING_PLATFORM_BOUNDING_BOXES: [Rect; 3] = [
    Rect {
        position: Point { x: 0, y: 0 },
        width: 60,
        height: 54,
    },
    Rect {
        position: Point { x: 60, y: 0 },
        width: 384 - (60 * 2),
        height: 93,
    },
    Rect {
        position: Point { x: 384 - 60, y: 0 },
        width: 60,
        height: 54,
    },
];

pub fn stone_and_platform(
    stone: HtmlImageElement,
    sprite_sheet: Rc<SpriteSheet>,
    offset_x: i16,
) -> Vec<Box<dyn Obstacle>> {
    vec![
        Box::new(Barrier::new(Image::new(
            stone,
            Point {
                x: offset_x + INITIAL_OBSTACLE_OFFSET,
                y: STONE_ON_GROUND,
            },
        ))),
        Box::new(create_floating_platform(
            sprite_sheet,
            Point {
                x: offset_x + FIRST_PLATFORM,
                y: LOW_PLATFORM,
            },
        )),
    ]
}

pub fn platform_and_stone(
    stone: HtmlImageElement,
    sprite_sheet: Rc<SpriteSheet>,
    offset_x: i16,
) -> Vec<Box<dyn Obstacle>> {
    vec![
        Box::new(create_floating_platform(
            sprite_sheet,
            Point {
                x: offset_x + INITIAL_OBSTACLE_OFFSET,
                y: HIGH_PLATFORM,
            },
        )),
        Box::new(Barrier::new(Image::new(
            stone,
            Point {
                x: offset_x + FIRST_PLATFORM,
                y: STONE_ON_GROUND,
            },
        ))),
    ]
}

pub fn stone_on_low_platform(
    stone: HtmlImageElement,
    sprite_sheet: Rc<SpriteSheet>,
    offset_x: i16,
) -> Vec<Box<dyn Obstacle>> {
    vec![
        Box::new(create_floating_platform(
            sprite_sheet,
            Point {
                x: offset_x + INITIAL_OBSTACLE_OFFSET,
                y: LOW_PLATFORM,
            },
        )),
        Box::new(Barrier::new(Image::new(
            stone,
            Point {
                x: offset_x + INITIAL_OBSTACLE_OFFSET + 160,
                y: STONE_ON_LOW_PLATFORM,
            },
        ))),
    ]
}

fn create_floating_platform(sprite_sheet: Rc<SpriteSheet>, position: Point) -> Platform {
    Platform::new(
        sprite_sheet,
        position,
        &FLOATING_PLATFORM_SPRITES,
        &FLOATING_PLATFORM_BOUNDING_BOXES,
    )
}
