#![allow(unused)]
mod lib;
use lib::model::*;

use ggez;
use ggez::event::{self, Axis, Button, GamepadId, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Align, Color, DrawParam, Font, Scale, Text, TextFragment};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;
use std::{thread, time};

const SCREEN_SIZE: (f32, f32) = (600.0, 400.0);
//------Change the Card every time when a new card shoule be added to screen----------
const SPACE_OUT: f32 = 40.0;
const BANKER_LINE: f32 = 20.0;
const HUMAN_LINE: f32 = 340.0;

//------Change the Card every time when a new card shoule be added to screen----------

pub struct MainState {
    pub deck: Deck,
    pub human: Human,
    pub human_images: Vec<(graphics::Image, ggez::mint::Point2<f32>)>,
    pub banker: Banker,
    pub banker_images: Vec<(graphics::Image, ggez::mint::Point2<f32>)>,
    pub game_state: i32,
    pub HM_DST: f32,
    pub BK_DST: f32,
}
impl MainState {
    fn hm_dst(&mut self) -> f32{
        let tmp = self.HM_DST;
        self.HM_DST += SPACE_OUT;
        tmp
    }
    fn bk_dst(&mut self) -> f32{
        let tmp = self.BK_DST;
        self.BK_DST += SPACE_OUT;
        tmp
    }
    fn result_draw_reset(&mut self, ctx: &mut Context, text: &str) {
        for i in &self.human_images {
            let img = &i.0;
            let dst = &i.1;
            graphics::draw(ctx, img, (*dst,)).unwrap();
        }
        for i in &self.banker_images {
            let img = &i.0;
            let dst = &i.1;
            graphics::draw(ctx, img, (*dst,)).unwrap();
        }
        let text_str = text.to_string();
        let mut text_view = graphics::Text::default();
        let color = Color::from((255, 0, 0, 255));
        for ch in text_str.chars() {
            text_view.add(
                TextFragment::new(ch).scale(Scale::uniform(80.0 + 80.0 * rand::random::<f32>())),
            );
        }
        graphics::draw(
            ctx,
            &text_view,
            (ggez::mint::Point2 { x: 100.0, y: 100.0 }, 0.0, color),
        )
        .unwrap();
        graphics::present(ctx).unwrap();
        let ten_millis = time::Duration::from_millis(4000);
        let now = time::Instant::now();
        thread::sleep(ten_millis);
        let rest_chips = self.human.chip;
        *self = MainState {
            deck: Deck::new(),
            human: Human::new(),
            human_images: Vec::new(),
            banker: Banker::new(),
            banker_images: Vec::new(),
            game_state: 0,
            HM_DST: 20.0,
            BK_DST:20.0,
        };
        self.deck.shuffle();
        self.human.chip = rest_chips;
    }
    //-----human get a card
    fn human_draw_card(&mut self, ctx: &mut Context) {
        self.human.draw_card(&mut self.deck);
        let hmdst = self.hm_dst();
        let get_top = self.human.lightcard.last().unwrap();
        self.human_images.push((
            graphics::Image::new(ctx, format!("/{}.png", get_top.stringfy())).unwrap(),
            ggez::mint::Point2 {
                x: hmdst,
                y: HUMAN_LINE,
            },
        ));
    }
    //------banker get a card
    fn banker_draw_card(&mut self, ctx: &mut Context) {
        if self.banker.lightcard.is_empty() {
            self.banker.draw_card(&mut self.deck);
            let bkdst = self.bk_dst();
            let get_top = self.banker.lightcard.last().unwrap();
            self.banker_images.push((
                graphics::Image::new(ctx, format!("/{}.png", get_top.stringfy())).unwrap(),
                ggez::mint::Point2 {
                    x: bkdst,
                    y: BANKER_LINE,
                },
            ));
        } else if self.banker.darkcard.is_none() {
            //The second card of banker should be back side
            self.banker.draw_card(&mut self.deck);
            let bkdst = self.bk_dst();
            let get_top = &self.banker.darkcard.as_ref().unwrap();
            self.banker_images.push((
                graphics::Image::new(ctx, "/back_side.png".to_string()).unwrap(),
                ggez::mint::Point2 {
                    x: bkdst,
                    y: BANKER_LINE,
                },
            ));
        } else {
            self.banker.draw_card(&mut self.deck);
            let bkdst = self.bk_dst();
            let get_top = self.banker.lightcard.last().unwrap();
            self.banker_images.push((
                graphics::Image::new(ctx, format!("/{}.png", get_top.stringfy())).unwrap(),
                ggez::mint::Point2 {
                    x: bkdst,
                    y: BANKER_LINE,
                },
            ));
        }
    }
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut s = MainState {
            deck: Deck::new(),
            human: Human::new(),
            human_images: Vec::new(),
            banker: Banker::new(),
            banker_images: Vec::new(),
            game_state: 0,
            HM_DST: 20.0,
            BK_DST:20.0,
        };
        s.deck.shuffle();
        Ok(s)
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        //---------show the human's chip----------
        let bets_view = graphics::Text::new(format!("chips:{}", self.human.chip.to_string()));
        let color = Color::from((0, 255, 0, 255));
        graphics::draw(
            ctx,
            &bets_view,
            (ggez::mint::Point2 { x: 400.0, y: 20.0 }, 0.0, color),
        )?;
        //---------show the human's bet----------
        let bets_view = graphics::Text::new(format!("bet:{}", self.human.bet.to_string()));
        let color = Color::from((0, 255, 0, 255));
        graphics::draw(
            ctx,
            &bets_view,
            (ggez::mint::Point2 { x: 400.0, y: 40.0 }, 0.0, color),
        )?;
        //---------show the human's bet----------
        //---------show the human's chip----------
        match self.game_state {
            0 => {
                //--------------Add bet button----------------
                let text = graphics::Text::new("Add bet:    +1    +5    +10");
                let color = Color::from((0, 255, 0, 255));
                let dest_point = ggez::mint::Point2 { x: 60.0, y: 150.0 };
                graphics::draw(ctx, &text, (dest_point, 0.0, color))?;
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    [130.0, 145.0, 30.0, 25.0].into(),
                    [0.0, 0.0, 0.0, 0.8].into(),
                )?;
                graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
                let rectangle2 = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    [170.0, 145.0, 30.0, 25.0].into(),
                    [0.0, 0.0, 0.0, 0.8].into(),
                )?;
                graphics::draw(ctx, &rectangle2, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
                let rectangle3 = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    [210.0, 145.0, 32.0, 25.0].into(),
                    [0.0, 0.0, 0.0, 0.8].into(),
                )?;
                graphics::draw(ctx, &rectangle3, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
                //--------------Add bet button----------------
                //--------------start button----------------
                let start = graphics::Text::new("start");
                let color = Color::from((0, 255, 0, 255));
                let dest_point = ggez::mint::Point2 { x: 350.0, y: 150.0 };
                let rectangle4 = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    [340.0, 145.0, 50.0, 25.0].into(),
                    [0.0, 0.0, 0.0, 0.8].into(),
                )?;
                graphics::draw(ctx, &rectangle4, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
                graphics::draw(ctx, &start, (dest_point, 0.0, color))?;
                //--------------start button----------------
            }
            1 => {
                //-----------------HIT BUTTON----------------------
                let text = graphics::Text::new("Hit");
                let color = Color::from((255, 0, 0, 255));
                let dest_point = ggez::mint::Point2 { x: 60.0, y: 200.0 };
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    [40.0, 190.0, 60.0, 40.0].into(),
                    [0.0, 0.0, 0.0, 0.8].into(),
                )?;
                graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
                graphics::draw(ctx, &text, (dest_point, 0.0, color))?;
                //-------------------------------------------------
                //-----------------STAND BUTTON----------------------
                let text = graphics::Text::new("Stand");
                let color = Color::from((255, 0, 0, 255));
                let dest_point = ggez::mint::Point2 { x: 130.0, y: 200.0 };
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    [120.0, 190.0, 60.0, 40.0].into(),
                    [0.0, 0.0, 0.0, 0.8].into(),
                )?;
                graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
                graphics::draw(ctx, &text, (dest_point, 0.0, color))?;
                //-------------------------------------------------
                //-----------------DOUBLE BUTTON----------------------
                let text = graphics::Text::new("Double");
                let color = Color::from((255, 0, 0, 255));
                let dest_point = ggez::mint::Point2 { x: 205.0, y: 200.0 };
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    [200.0, 190.0, 60.0, 40.0].into(),
                    [0.0, 0.0, 0.0, 0.8].into(),
                )?;
                graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
                graphics::draw(ctx, &text, (dest_point, 0.0, color))?;
                //-------------------------------------------------
                //-----------------Surrender BUTTON----------------------
                let text = graphics::Text::new("Surrender");
                let color = Color::from((255, 0, 0, 255));
                let dest_point = ggez::mint::Point2 { x: 285.0, y: 200.0 };
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    [280.0, 190.0, 80.0, 40.0].into(),
                    [0.0, 0.0, 0.0, 0.8].into(),
                )?;
                graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
                graphics::draw(ctx, &text, (dest_point, 0.0, color))?;
                //-------------------------------------------------
                //--------------draw cards -------------------
                for i in &self.human_images {
                    let img = &i.0;
                    let dst = &i.1;
                    graphics::draw(ctx, img, (*dst,))?;
                }
                for i in &self.banker_images {
                    let img = &i.0;
                    let dst = &i.1;
                    graphics::draw(ctx, img, (*dst,))?;
                }
                //--------------draw cards -------------------
            }
            -1 => {
                graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    [0.0, 0.0, 600.0, 400.0].into(),
                    [0.0, 0.0, 0.0, 0.8].into(),
                )?;
                graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
                let text_str = "Game End".to_string();
                let mut text_view = graphics::Text::default();
                let color = Color::from((255, 0, 0, 255));
                for ch in text_str.chars() {
                    text_view.add(
                        TextFragment::new(ch)
                            .scale(Scale::uniform(80.0 + 80.0 * rand::random::<f32>())),
                    );
                }
                graphics::draw(
                    ctx,
                    &text_view,
                    (ggez::mint::Point2 { x: 30.0, y: 100.0 }, 0.0, color),
                )
                .unwrap();
            }
            _ => {}
        }

        graphics::present(ctx)?;
        Ok(())
    }
    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        //------------------Hit buttons event--------------------
        if (x > 60.0 && y > 190.0) && (x < 120.0 && y < 250.0) && self.game_state == 1 {
            // Hit Button
            self.human.draw_card(&mut self.deck);
            let hmdst = self.hm_dst();
            let get_top = self.human.lightcard.last().unwrap();
            self.human_images.push((
                graphics::Image::new(ctx, format!("/{}.png", get_top.stringfy())).unwrap(),
                ggez::mint::Point2 {
                    x: hmdst,
                    y: HUMAN_LINE,
                },
            ));
            let text = graphics::Text::new("Hit");
            let color = Color::from((255, 0, 0, 255));
            let dest_point = ggez::mint::Point2 { x: 60.0, y: 200.0 };
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                [40.0, 190.0, 60.0, 40.0].into(),
                [1.0, 1.0, 1.0, 0.9].into(),
            )
            .unwrap();
            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
            graphics::draw(ctx, &text, (dest_point, 0.0, color)).unwrap();
            graphics::present(ctx).unwrap();
            if self.human.compute_value() > 21 {
                self.human.lose();
                let i = self.human_images.last().unwrap();
                let img = &i.0;
                let dst = &i.1;
                graphics::draw(ctx, img, (*dst,)).unwrap();
                graphics::present(ctx).unwrap();
                self.result_draw_reset(ctx, "BUST!!!");
                if self.human.chip == 0 {
                    self.game_state = -1;
                }
            }
        }
        //------------------Hit buttons event--------------------

        //------------------Stand buttons event--------------------
        if (x > 120.0 && y > 190.0)
            && (x < 120.0 + 60.0 && y < 190.0 + 40.0)
            && self.game_state == 1
        {
            while self.banker.compute_value() < 17 {
                self.banker_draw_card(ctx);
            }
            let get_top = &self.banker.darkcard.as_ref().unwrap();
            self.banker_images.push((
                graphics::Image::new(ctx, format!("/{}.png", get_top.stringfy())).unwrap(),
                ggez::mint::Point2 {
                    x: 60.0,
                    y: BANKER_LINE,
                },
            ));
            if self.banker.compute_value() > 21 {
                self.human.win();
                self.result_draw_reset(ctx, "You win");
            } else if self.banker.compute_value() == self.human.compute_value() {
                if self.banker.blackjack && self.human.blackjack {
                    self.human.tie();
                    self.result_draw_reset(ctx, "Tie");
                } else if !self.banker.blackjack && self.human.blackjack {
                    self.human.win();
                    self.result_draw_reset(ctx, "You win");
                } else if !self.banker.blackjack && !self.human.blackjack {
                    self.result_draw_reset(ctx, "You lose");
                    self.human.lose();
                    if self.human.chip == 0 {
                        self.game_state = -1;
                    }
                }
            } else if self.banker.compute_value() < self.human.compute_value() {
                self.result_draw_reset(ctx, "You win");
                self.human.win();
            } else if self.banker.compute_value() > self.human.compute_value() {
                self.result_draw_reset(ctx, "You lose");
                self.human.lose();
                if self.human.chip == 0 {
                    self.game_state = -1;
                }
            }
        }
        //------------------Stand buttons event--------------------
        //------------------Double buttons event--------------------
        if (x > 200.0 && y > 190.0)
            && (x < 200.0 + 60.0 && y < 190.0 + 40.0)
            && self.game_state == 1
        {
            self.human.add_bet(self.human.bet);
        }
        //------------------Double buttons event--------------------
        //------------------Surrender buttons event--------------------
        if (x > 280.0 && y > 190.0)
            && (x < 280.0 + 80.0 && y < 190.0 + 40.0)
            && self.game_state == 1
        {
            self.result_draw_reset(ctx, "You lose");
            if self.human.chip == 0 {
                self.game_state = -1;
            }
        }
        //------------------Surrender buttons event--------------------
        //------------------Add bet buttons event--------------------
        if (x > 130.0 && y > 145.0)
            && (x < 130.0 + 30.0 && y < 145.0 + 25.0)
            && self.game_state == 0
        {
            self.human.add_bet(1);
        }
        if (x > 170.0 && y > 145.0)
            && (x < 170.0 + 30.0 && y < 145.0 + 25.0)
            && self.game_state == 0
        {
            self.human.add_bet(5);
        }
        if (x > 210.0 && y > 145.0)
            && (x < 210.0 + 30.0 && y < 145.0 + 25.0)
            && self.game_state == 0
        {
            self.human.add_bet(10);
        }
        if (x > 340.0 && y > 145.0)
            && (x < 340.0 + 50.0 && y < 145.0 + 25.0)
            && self.game_state == 0
        {
            self.human_draw_card(ctx);
            self.human_draw_card(ctx);
            self.banker_draw_card(ctx);
            self.banker_draw_card(ctx);
            self.game_state = 1;
        }
        //------------------Add bet buttons event--------------------
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    //const SCREEN_SIZE: (f32, f32) = (600.0, 400.0);
    let cb = ggez::ContextBuilder::new("TEST", "ggez")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("TEST"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1));

    let (ctx, events_loop) = &mut cb.build()?;

    println!("{}", graphics::renderer_info(ctx)?);
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, events_loop, state)
}
