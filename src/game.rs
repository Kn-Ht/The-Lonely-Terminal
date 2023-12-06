use raylib::prelude::*;

pub const WINDOW_SIZE: (i32, i32) = (960, 840);
pub const WINDOW_TITLE: &str = "Hello World!";

#[derive(Debug)]
pub struct Fonts {
    pub martian: Font,
    pub termplus: Font,
}

impl Fonts {
    pub fn get() -> Self {
        Self {
            martian: load_font!("../font/MartianMonoNF.ttf"),
            termplus: load_font!("../font/TermPlusNF.ttf"),
        }
    }
}

#[derive(Debug)]
pub struct Game {
    // core
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    // audio
    pub rl_audio: RaylibAudio,
    pub fonts: Fonts,
}

impl Game {
    // construct from existing
    pub fn new() -> Self {
        // set config flags
        unsafe {
            raylib::ffi::SetConfigFlags(
                ConfigFlags::FLAG_WINDOW_HIGHDPI as u32
            );
        }

        let (mut rl, thread) = raylib::init()
            .size(WINDOW_SIZE.0, WINDOW_SIZE.1)
            .title(WINDOW_TITLE)
            .build();
        
        rl.set_target_fps(
            unsafe {
                raylib::ffi::GetMonitorRefreshRate(
                    raylib::ffi::GetCurrentMonitor()
                )
            } as u32
        );

        rl.set_exit_key(None);

        let rl_audio = RaylibAudio::init_audio_device();
        let fonts = Fonts::get();

        Self {
            rl,
            thread,
            rl_audio,
            fonts,
        }
    }
    pub fn fps(&self) -> u32 {
        self.rl.get_fps()
    }
    pub fn run(
        &mut self,
        event_handler: fn(&mut Self, Option<KeyboardKey>),
    ) {
        while !self.rl.window_should_close() {
            let key = self.rl.get_key_pressed();
            event_handler(self, key);
        }
    }
}
