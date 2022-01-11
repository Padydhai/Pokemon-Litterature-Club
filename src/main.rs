//Bienvenue dans notre projet
//Voici les implémentations, la plupart sont mit par défaut, et certaine 
//ne trouve pas d'utilité dans le code (pour l'instant).

//use sauv;
use ggez;
use ggez::event;
//use ggez::event::KeyCode;
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
    text: graphics::Text,
    text2: graphics::Text,
    bye: graphics::Text,
    img: graphics::Image,
    img2: graphics::Image,
    bkg: graphics::Image,
    maintext: graphics::Text
    
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        
        let bkg = graphics::Image::new(ctx, "/fdpkm.png")?;
        let font = graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?;
        let text = graphics::Text::new(("Appuie sur 'espace' pour commencer.", font, 24.0));
        let text2 = graphics::Text::new(("Appuie sur 'espace' pour commencer.", font , 24.0));
        let img = graphics::Image::new(ctx, "/pkm3.png")?;
        let img2 = graphics::Image::new(ctx, "/sbpkm2.png")?;
        let bye = graphics::Text::new(("Appuie sur 'echap' pour quitter.", font, 12.0));
        
        let mut sound = audio::Source::new(ctx, "/soundtrack.mp3")?;
        let _ = sound.play_detached(ctx);
        
        //let dialog = jison.get("acte1");
        let pointjson = File::open("data.json")?;
        let jison: serde_json::Value = serde_json::from_reader(pointjson).unwrap();
        
        let maintext = graphics::Text::new(("Réceptacle du texte principal", font, 30.0));     
        
        let s = MainState { 
            text, 
            text2, 
            bye,
            img, 
            img2,
            bkg,
            maintext
        };
        Ok(s)
    }
}


impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into()); 
        
        let color = Color::from((0, 0, 0, 255));
        graphics::draw(ctx, &self.bkg, (Vec2::new(0.0, -1000.0),))?;
        
        graphics::draw(ctx, &self.text2, (Vec2::new(205.0, 405.0), color))?;
        graphics::draw(ctx, &self.text, (Vec2::new(200.0, 400.0),))?;
        graphics::draw(ctx, &self.bye, (Vec2::new(0.0, 0.0),))?;
        
        graphics::draw(ctx, &self.img, (Vec2::new(180.0, 100.0),))?;
        graphics::draw(ctx, &self.img2, (Vec2::new(180.0, 260.0),))?;

        graphics::draw(ctx, &self.maintext, (Vec2::new(50.0, 500.0),))?;

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
            //input::keyboard::KeyCode::Space => self.(ctx),
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
