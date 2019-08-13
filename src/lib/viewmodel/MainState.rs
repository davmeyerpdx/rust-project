use crate::*;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;

pub struct MainState{
    image1: graphics::Image,
    rotation: f32
}
impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let a = Card::new_by_rankid(1, Heart);
        let image1 = graphics::Image::new(ctx, format!("/{}.png",a.stringfy()))?;
        let s = MainState{
            image1,
            rotation: 1.0,
        };
        Ok(s)
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        /*
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.rotation += 0.01;
        }
        */
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // Draw an image.
        let dst = ggez::mint::Point2{ x: 20.0, y: 20.0 };
        let dst2 = ggez::mint::Point2{ x: 20.0, y: 340.0 };
        graphics::draw(ctx, &self.image1, (dst,))?;
        let a = Card::new_by_rankid(3, Heart);
        let image2 = graphics::Image::new(ctx, format!("/{}.png",a.stringfy()))?;
        graphics::draw(ctx, &image2, (dst2,))?;
        graphics::present(ctx)?;
        Ok(())
    }
}

// struct HumanView{
//     pub human: Human,
//     pub images: Vec<(graphics::Image, ggez::mint::Point2<f32>)>,
// }
// struct BankerView{
//     pub banker: Banker,
//     pub images: Vec<(graphics::Image, ggez::mint::Point2<f32>)>,
//     pub back_side_image: (graphics::Image, ggez::mint::Point2<f32>),
// }

// impl HumanView{
//     fn new() -> Self {
//         HumanView{human: Human::new(), images: Vec::new()}
//     }
// }
