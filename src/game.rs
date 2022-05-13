use self::rhb::RedHatBoy;
use crate::engine::{Game, Image, KeyState, Point, Rect, Renderer, Sheet};
use crate::{browser, engine};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use web_sys::HtmlImageElement;

mod rhb;

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
    background: Image,
    stone: Image,
    platform: Platform,
}

struct Platform {
    sheet: Sheet,
    image: HtmlImageElement,
    position: Point,
}
impl Platform {
    fn new(sheet: Sheet, image: HtmlImageElement, position: Point) -> Self {
        Platform {
            sheet,
            image,
            position,
        }
    }
    fn draw(&self, renderer: &Renderer) {
        let platform = self
            .sheet
            .frames
            .get("13.png")
            .expect("13.png does not exist");

        renderer.draw_image(
            &self.image,
            &Rect {
                x: platform.frame.x.into(),
                y: platform.frame.y.into(),
                width: (platform.frame.w * 3).into(),
                height: platform.frame.h.into(),
            },
            &self.bounding_box(),
        )
    }
    fn bounding_box(&self) -> Rect {
        let platform = self
            .sheet
            .frames
            .get("13.png")
            .expect("13.png does not exist");
        Rect {
            x: self.position.x.into(),
            y: self.position.y.into(),
            width: (platform.frame.w * 3).into(),
            height: platform.frame.h.into(),
        }
    }
    fn draw_bounding_box(&self, renderer: &Renderer) {
        renderer.draw_rect(&self.bounding_box())
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initalize(&self) -> Result<Box<dyn Game>> {
        match self {
            WalkTheDog::Loading => {
                let sheet = browser::fetch_json("rhb.json").await?.into_serde()?;

                let background = engine::load_image("BG.png").await?;
                let stone = engine::load_image("Stone.png").await?;
                let rhb = RedHatBoy::new(sheet, engine::load_image("rhb.png").await?);

                let platform_sheet = browser::fetch_json("tiles.json").await?;

                let platform = Platform::new(
                    platform_sheet.into_serde::<Sheet>()?,
                    engine::load_image("tiles.png").await?,
                    Point { x: 200, y: 400 },
                );

                Ok(Box::new(WalkTheDog::Loaded(Walk {
                    boy: rhb,
                    background: Image::new(background, Point { x: 0, y: 0 }),
                    stone: Image::new(stone, Point { x: 150, y: 546 }),
                    platform,
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
            if walk
                .boy
                .bounding_box()
                .intersects(&walk.platform.bounding_box())
            {
                if walk.boy.velocity_y() > 0 && walk.boy.pos_y() < walk.platform.position.y {
                    walk.boy.land_on(walk.platform.bounding_box().y as i16);
                } else {
                    walk.boy.knock_out();
                }
            }

            if walk
                .boy
                .bounding_box()
                .intersects(walk.stone.bounding_box())
            {
                //walk.boy.knock_out();
            }
        }
    }
    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Rect {
            x: 0.0,
            y: 0.0,
            width: 600.0,
            height: HEIGHT as f32,
        });

        if let WalkTheDog::Loaded(walk) = self {
            walk.background.draw(renderer);
            walk.boy.draw(renderer);
            walk.boy.draw_bounding_box(renderer);
            walk.stone.draw(renderer);
            walk.stone.draw_bounding_box(renderer);
            walk.platform.draw(renderer);
            walk.platform.draw_bounding_box(renderer);
        }
    }
}
