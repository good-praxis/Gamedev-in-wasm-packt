use self::rhb::RedHatBoy;
use crate::engine::{Game, Image, KeyState, Point, Rect, Renderer, Sheet};
use crate::{browser, engine};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use web_sys::HtmlImageElement;

mod rhb;

pub const HEIGHT: i16 = 600;
const LOW_PLATFORM: i16 = 420;
const HIGH_PLATFORM: i16 = 375;

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
}
impl Walk {
    fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }
}

pub trait Obstacle {
    fn check_intersection(&self, boy: &mut RedHatBoy);
    fn draw(&self, renderer: &Renderer);
    fn move_horizontally(&mut self, x: i16);
    fn draw_bounding_box(&self, renderer: &Renderer);
    fn right(&self) -> i16;
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
    fn destination_box(&self) -> Rect {
        let platform = self
            .sheet
            .frames
            .get("13.png")
            .expect("13.png does not exist");
        Rect::new(self.position, platform.frame.w * 3, platform.frame.h)
    }
    fn bounding_boxes(&self) -> Vec<Rect> {
        const X_OFFSET: i16 = 60;
        const END_HEIGHT: i16 = 54;
        let destination_box = self.destination_box();
        let bounding_box_one = Rect::new(destination_box.position, X_OFFSET, END_HEIGHT);

        let bounding_box_two = Rect::new_from_x_y(
            destination_box.x() + X_OFFSET,
            destination_box.y(),
            destination_box.width - (X_OFFSET * 2),
            destination_box.height,
        );

        let bounding_box_three = Rect::new_from_x_y(
            destination_box.x() + destination_box.width - X_OFFSET,
            destination_box.y(),
            X_OFFSET,
            END_HEIGHT,
        );

        vec![bounding_box_one, bounding_box_two, bounding_box_three]
    }
}
impl Obstacle for Platform {
    fn draw(&self, renderer: &Renderer) {
        let platform = self
            .sheet
            .frames
            .get("13.png")
            .expect("13.png does not exist");

        renderer.draw_image(
            &self.image,
            &Rect::new_from_x_y(
                platform.frame.x,
                platform.frame.y,
                platform.frame.w * 3,
                platform.frame.h,
            ),
            &self.destination_box(),
        )
    }
    fn move_horizontally(&mut self, x: i16) {
        self.position.x += x;
    }
    fn check_intersection(&self, boy: &mut RedHatBoy) {
        if let Some(box_to_land_on) = self
            .bounding_boxes()
            .iter()
            .find(|&bounding_box| boy.bounding_box().intersects(bounding_box))
        {
            if boy.velocity_y() > 0 && boy.pos_y() < self.position.y {
                boy.land_on(box_to_land_on.y());
            } else {
                if boy.velocity_y() < 0 && boy.pos_y() > self.position.y {
                    boy.hit_ceiling();
                }
                boy.knock_out();
            }
        }
    }
    fn draw_bounding_box(&self, renderer: &Renderer) {
        for bounding_box in self.bounding_boxes() {
            renderer.draw_rect(&bounding_box);
        }
    }
    fn right(&self) -> i16 {
        self.bounding_boxes()
            .last()
            .unwrap_or(&Rect::default())
            .right()
    }
}

pub struct Barrier {
    image: Image,
}
impl Barrier {
    pub fn new(image: Image) -> Self {
        Barrier { image }
    }
}
impl Obstacle for Barrier {
    fn check_intersection(&self, boy: &mut RedHatBoy) {
        if boy.bounding_box().intersects(self.image.bounding_box()) {
            boy.knock_out();
        }
    }

    fn draw(&self, renderer: &Renderer) {
        self.image.draw(renderer);
    }

    fn move_horizontally(&mut self, x: i16) {
        self.image.move_horizontally(x);
    }

    fn draw_bounding_box(&self, renderer: &Renderer) {
        self.image.draw_bounding_box(renderer);
    }

    fn right(&self) -> i16 {
        self.image.right()
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

                let platform_sheet = browser::fetch_json("tiles.json").await?;

                let platform = Platform::new(
                    platform_sheet.into_serde::<Sheet>()?,
                    engine::load_image("tiles.png").await?,
                    Point {
                        x: 200,
                        y: LOW_PLATFORM,
                    },
                );

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
                    obstacles: vec![
                        Box::new(Barrier::new(Image::new(stone, Point { x: 150, y: 546 }))),
                        Box::new(platform),
                    ],
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
