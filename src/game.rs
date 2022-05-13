use self::rhb::RedHatBoy;
use crate::engine::{Game, Image, KeyState, Point, Rect, Renderer};
use crate::{browser, engine};
use anyhow::{anyhow, Result};
use async_trait::async_trait;

mod rhb;

pub enum WalkTheDog {
    Loading,
    Loaded(Walk),
}

pub struct Walk {
    boy: RedHatBoy,
    background: Image,
}

impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog::Loading
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initalize(&self) -> Result<Box<dyn Game>> {
        match self {
            WalkTheDog::Loading => {
                let sheet = browser::fetch_json("rhb.json").await?.into_serde()?;

                let background = engine::load_image("BG.png").await?;
                let rhb = RedHatBoy::new(sheet, engine::load_image("rhb.png").await?);

                Ok(Box::new(WalkTheDog::Loaded(Walk {
                    boy: rhb,
                    background: Image::new(background, Point { x: 0, y: 0 }),
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
        }
    }
    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Rect {
            x: 0.0,
            y: 0.0,
            width: 600.0,
            height: 600.0,
        });

        if let WalkTheDog::Loaded(walk) = self {
            walk.background.draw(renderer);
            walk.boy.draw(renderer);
        }
    }
}
