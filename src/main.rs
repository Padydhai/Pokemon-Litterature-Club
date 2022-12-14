//Bienvenue dans notre projet
//Voici les implémentations, la plupart sont mit par défaut, et certaine 
//ne trouve pas d'utilité dans le code (pour l'instant).

//use sauv;
use ggez;
use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::input::keyboard;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::graphics::{self, Align, Color, DrawParam, Font, PxScale, Text, TextFragment};
//use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use glam::Vec2;
//use std::collections::BTreeMap;
use std::env;
//use std::f32;
use std::path;
use ggez::input;
use ggez::audio;
use ggez::audio::SoundSource;

use serde::Deserialize;
use std::fs::File;
use std::io::Read;

//Pour avoir des bouttons 
pub enum Menu {
    Main,
    Sauvegarde,
    Next,
    Quit
}

//Pour la partie interface
struct MainState {
    i: i32,
    j: i32,
    text: graphics::Text,
    text2: graphics::Text,
    text3: graphics::Text,
    text4: graphics::Text,
    bye: graphics::Text,
    img: graphics::Image,
    img2: graphics::Image,
    bkg: graphics::Image,
    maintext: graphics::Text,
    
    bkg2: graphics::Image,
    dia: graphics::Image,
    expT7: graphics::Image,
    sound: audio::Source,
    sound2: audio::Source,
    selectsound: audio::Source

}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let pointjson = File::open("./data.json")?;
        let jison: serde_json::Value = serde_json::from_reader(pointjson).unwrap(); 
        
        let bkg = graphics::Image::new(ctx, "/fdpkm.png")?;
        let bkg2 = graphics::Image::new(ctx, "/fdpkmA3.png")?;
        let font = graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?;
        
        let text = graphics::Text::new((jison["menu"][1].to_string(), font, 24.0));
        let text2 = graphics::Text::new((jison["menu"][1].to_string(), font, 24.0));
        let text3 = graphics::Text::new((jison["menu"][0].to_string(), font, 24.0));
        let text4 = graphics::Text::new((jison["menu"][0].to_string(), font, 24.0));
        let img = graphics::Image::new(ctx, "/pkm3.png")?;
        let img2 = graphics::Image::new(ctx, "/sbpkm2.png")?;
        let dia = graphics::Image::new(ctx, "/diag4.png")?;
        let expT7 = graphics::Image::new(ctx, "/expT7.png")?;
        let bye = graphics::Text::new((jison["menu"][2].to_string(), font, 12.0));
        let maintext = graphics::Text::new((jison["acte1"][0].to_string(), font, 26.0));     
        
        let mut sound = audio::Source::new(ctx, "/soundtrack.mp3")?;
        let _ = sound.play(ctx);

        
        
        let mut sound = audio::Source::new(ctx, "/soundtrack.mp3")?;
        let _ = sound.play(ctx); 
        let mut sound2 = audio::Source::new(ctx, "/soundtrack2.mp3")?;
        let mut selectsound = audio::Source::new(ctx, "/selectsound.mp3")?;
        let mut i = 0;
        let mut j = 0;

        let s = MainState { 
            i,
            j,
            text, 
            text2, 
            text3,
            text4,
            bye,
            img, 
            img2,
            bkg,
            maintext,
            bkg2,
            dia,
            expT7,
            sound,
            sound2,
            selectsound };
        Ok(s)
    }
    
    fn suite(&mut self, ctx: &mut Context) {
        if self.i < 1 {    
            let _ = self.sound.stop(ctx);
            self.selectsound.play_detached(ctx);
            self.selectsound.stop(ctx);
            self.sound2.play(ctx);
            println!("Suite lancée.")
        } else {
            println!("RAS {}", self.i)
        }
    }

    fn page_suite(&mut self, ctx: &mut Context) {
        println!("Page Suivante. {}" , self.j)
    }

}


impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
     

        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into()); 
        
        let color = Color::from((0, 0, 0, 255));

        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            self.i += 1; 
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Space) {
            if self.i == 0 {
                self.j = 0;
            } else {
                self.j += 1;
            } 
        }

        if self.i < 1 {
            graphics::draw(ctx, &self.bkg, (Vec2::new(0.0, -1000.0),))?;
        
            graphics::draw(ctx, &self.text2, (Vec2::new(205.0, 405.0), color))?;
            graphics::draw(ctx, &self.text, (Vec2::new(200.0, 400.0),))?;
            graphics::draw(ctx, &self.bye, (Vec2::new(0.0, 0.0),))?;
        
            graphics::draw(ctx, &self.img, (Vec2::new(180.0, 100.0),))?;
            graphics::draw(ctx, &self.img2, (Vec2::new(180.0, 260.0),))?;
        }

        
        
        
        if self.i >= 1 {
            if self.j < 1 {
                graphics::draw(ctx, &self.bkg2, (Vec2::new(-235.0, 0.0),))?;
                graphics::draw(ctx, &self.text4, (Vec2::new(100.0, 405.0), color))?;
                graphics::draw(ctx, &self.text3, (Vec2::new(105.0, 400.0),))?;
            }
        }
        
        if self.j > 1{
            graphics::draw(ctx, &self.bkg2, (Vec2::new(-235.0, 0.0),))?;
            graphics::draw(ctx, &self.dia, (Vec2::new(200.0, 440.0),))?;
            graphics::draw(ctx, &self.expT7, (Vec2::new(20.0, 440.0),))?;
            graphics::draw(ctx, &self.maintext, (Vec2::new(235.0, 500.0),))?;
        }

        graphics::present(ctx)?;
        
        
        Ok(())
    }
    

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: input::keyboard::KeyCode,
        _keymod: input::keyboard::KeyMods,
        _repeat: bool,
    ) { 
        match keycode {
            input::keyboard::KeyCode::Return => self.suite(ctx),
            input::keyboard::KeyCode::Space => self.page_suite(ctx),
            input::keyboard::KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
    }
    
}

pub fn main() -> GameResult {
    
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("src");
        path
    } else {
        path::PathBuf::from("./src")
    };
    
    
    
    let cb = ggez::ContextBuilder::new("pokerust", "ggez")
    .window_setup(WindowSetup::default().title("Projet Rust 21-22"))
    .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    
    
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
    
}
