extern crate sdl2;
extern crate gl;

fn main() {
    let sdl = setup_sdl();
    let vid_sub = setup_view_subsystem(&sdl);
    let window = setup_window(&vid_sub);
    let event_pump = setup_event_pump(&sdl);
    let _gl_context = setup_gl_context(&window);
    let _gl = setup_gl(&vid_sub); 
    
    setup_clear_color();
    main_game_loop(event_pump, &window);    
}

fn setup_sdl()->sdl2::Sdl {
    return sdl2::init().unwrap();
}

fn setup_window(video_subsystem: &sdl2::VideoSubsystem) -> sdl2::video::Window {
    return video_subsystem.window("Game", 800, 600).opengl().resizable().build().unwrap();
}

fn setup_view_subsystem(sdl: &sdl2::Sdl) -> sdl2::VideoSubsystem {
    return sdl.video().unwrap();
}

fn setup_event_pump(sdl: &sdl2::Sdl) -> sdl2::EventPump {
    return sdl.event_pump().unwrap();
}

fn setup_gl_context(window: &sdl2::video::Window) -> sdl2::video::GLContext {
    return window.gl_create_context().unwrap(); 
}

fn setup_gl(video_subsystem: &sdl2::VideoSubsystem) {
    return gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
}

fn setup_clear_color() {
    unsafe{ gl::ClearColor(0.3, 0.3, 0.5, 1.0); }
}

fn clear_color() {
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
}

fn custom_render(window: &sdl2::video::Window){
   window.gl_swap_window(); 
}

fn main_game_loop(mut event_pump: sdl2::EventPump , window: &sdl2::video::Window) {
    //main game loop 
    'main: loop{
        //catch events each 
        for event in event_pump.poll_iter(){
            match event { sdl2::event::Event::Quit {..} => break 'main, _ => {}, }
        }

        //render code here
        clear_color();
        custom_render(&window);
    }
}
