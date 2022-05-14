use self::obstacles::Obstacle;
use self::rhb::RedHatBoy;
use crate::engine::{Game, Image, KeyState, Point, Rect, Renderer, Sheet, SpriteSheet};
use crate::{browser, engine};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::rc::Rc;

mod obstacles;
mod rhb;
mod segments;

pub const HEIGHT: i16 = 600;

pub enum WalkTheDog {
    Loading,
    Loaded(Walk),
}
impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog::Loading
    }
}

pub struct Walk {
    boy: RedHatBoy,
    backgrounds: [Image; 2],
    obstacles: Vec<Box<dyn Obstacle>>,
    obstacle_sheet: Rc<SpriteSheet>,
}
impl Walk {
    fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initalize(&self) -> Result<Box<dyn Game>> {
        match self {
            WalkTheDog::Loading => {
                let sheet = browser::fetch_json("rhb.json").await?.into_serde()?;

                let background = engine::load_image("BG.png").await?;
                let background_width = background.width() as i16;
                let stone = engine::load_image("Stone.png").await?;
                let rhb = RedHatBoy::new(sheet, engine::load_image("rhb.png").await?);

                let tiles = browser::fetch_json("tiles.json").await?;

                let sprite_sheet = Rc::new(SpriteSheet::new(
                    tiles.into_serde::<Sheet>()?,
                    engine::load_image("tiles.png").await?,
                ));

                Ok(Box::new(WalkTheDog::Loaded(Walk {
                    boy: rhb,
                    backgrounds: [
                        Image::new(background.clone(), Point { x: 0, y: 0 }),
                        Image::new(
                            background,
                            Point {
                                x: background_width,
                                y: 0,
                            },
                        ),
                    ],
                    obstacles: segments::stone_and_platform(stone, sprite_sheet.clone(), 0),
                    obstacle_sheet: sprite_sheet,
                })))
            }
            WalkTheDog::Loaded(_) => Err(anyhow!("Error: Game is already initalized!")),
        }
    }
    fn update(&mut self, keystate: &KeyState) {
        let mut velocity = Point { x: 0, y: 0 };

        if let WalkTheDog::Loaded(walk) = self {
            if keystate.is_pressed("ArrowRight") {
                velocity.x += 3;
                walk.boy.run_right();
            }

            if keystate.is_pressed("ArrowDown") {
                walk.boy.slide();
            }

            if keystate.is_pressed("Space") {
                walk.boy.jump();
            }

            walk.boy.update();

            let velocity = walk.velocity();

            let [first_background, second_background] = &mut walk.backgrounds;
            first_background.move_horizontally(velocity / 3);
            second_background.move_horizontally(velocity / 3);

            if first_background.right() < 0 {
                first_background.set_x(second_background.right());
            }
            if second_background.right() < 0 {
                second_background.set_x(second_background.right());
            }

            walk.obstacles.retain(|obstacle| obstacle.right() > 0);

            walk.obstacles.iter_mut().for_each(|obstacle| {
                obstacle.move_horizontally(velocity);
                obstacle.check_intersection(&mut walk.boy)
            });
        }
    }
    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Rect::new_from_x_y(0, 0, 600, HEIGHT));

        if let WalkTheDog::Loaded(walk) = self {
            walk.backgrounds.iter().for_each(|bg| bg.draw(renderer));
            walk.boy.draw(renderer);
            walk.boy.draw_bounding_box(renderer);
            walk.obstacles.iter().for_each(|obstacle| {
                obstacle.draw(renderer);
                obstacle.draw_bounding_box(renderer);
            });
        }
    }
}
