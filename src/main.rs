use nannou::prelude::*;
use nannou::noise::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    things: Vec<Thing>,
    things_two: Vec<ThingTwo>,
    noise: Perlin,
}

struct Thing {
    positions: Vec<Vec2>,
}

impl Thing {
    pub fn new(p: Vec2) -> Self {
        let mut positions = Vec::new();
        positions.push(p);
        Thing {
            positions
        }
    }
}

struct ThingTwo {
    positions: Vec<Vec2>,
}

impl ThingTwo {
    pub fn new(p: Vec2) -> Self {
        let mut positions = Vec::new();
        positions.push(p);
        ThingTwo {
            positions
        }
    }
}

const N_THINGS: usize = 1000;
const M_THINGS: usize = 2000;

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .event(event)
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    let mut things: Vec<Thing> = Vec::new();
    let mut things_two: Vec<ThingTwo> = Vec::new();


    for _i in 0..N_THINGS {
        let thing = Thing::new(vec2(
            (random::<f32>() - 0.5) * 1024.0,
            (random::<f32>() - 0.5) * 1024.0
        ));
        things.push(thing);
    }

    for _i in 0..M_THINGS {
        let thing = ThingTwo::new(vec2(
            (random::<f32>() - 0.5) * 1024.0,
            (random::<f32>() - 0.5) * 1024.0
        ));
        things_two.push(thing);
    }

    let noise = Perlin::new();

    Model { 
        things,
        things_two,
        noise,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32 / 120.0;
    let time_two = app.elapsed_frames() as f32 / 500.0;
    // let sn = 0.01 * time.cos() as f64 * 0.05;
    let sn = 0.01 * time.tan() as f64 * 0.9;
    for thing in model.things.iter_mut() {

        thing.positions.clear();
        thing.positions.push(vec2(
            (random::<f32>() - 0.5) * 1024.0,
            (random::<f32>() - 0.5) * 1024.0,
        ));

        for _k in 0..50 {
            let last = thing.positions[0];
            let new = last + vec2(            
                model.noise.get([sn*last.x as f64, sn*last.y as f64, 0.0]) as f32,
                model.noise.get([sn*last.x as f64, sn*last.y as f64, 1.0]) as f32,
            );
            thing.positions.insert(0, new);
        }
    }
    
    let tn = 0.50 * time_two.sin() as f64 * 0.10;
    for thing in model.things_two.iter_mut() {

        thing.positions.clear();
        thing.positions.push(vec2(
            (random::<f32>() - 0.5) * 1024.0,
            (random::<f32>() - 0.5) * 1024.0,
        ));

        for _k in 0..50 {
            let last = thing.positions[0];
            let new = last + vec2(            
                model.noise.get([tn*last.x as f64, tn*last.y as f64, 0.5]) as f32,
                model.noise.get([tn*last.x as f64, tn*last.y as f64, 0.5]) as f32,
            );
            thing.positions.insert(0, new);
        }
    }
}

fn view (app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.rect().w_h(1024.0, 1024.0).color(srgba(0.0, 0.0, 0.00, 0.1));


    for thing in model.things.iter() {
        draw.polyline().points(thing.positions.iter().cloned()).color(MEDIUMPURPLE);       
    }

    for thing_m  in model.things_two.iter() {
        draw.polyline().points(thing_m.positions.iter().cloned()).color(MEDIUMSLATEBLUE);
    }

    
    let n_points = 8;
    let win = app.window_rect();
    let radius = win.w().min(win.h()) * 0.15;
    let points = (0..n_points).map(|i| {
        let fract = i as f32 / n_points as f32;
        let phase = fract;
        let x = radius * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
        pt2(x, y)
    });

    draw.polygon()
        .x(-win.w() * 0.25)
        .color(ORANGERED)
        // .rgba(255.0, 15., 25., 0.5)
        .rotate(-app.time * 0.80)
        .stroke(RED)
        .stroke_weight(10.0)
        .join_bevel()
        .points(points);

    draw.to_frame(app, &frame).unwrap();
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    println!("event captured {:?}", event);

    match event {
        // Keyboard events
        KeyPressed(_key) => {}
        KeyReleased(_key) => {}
        ReceivedCharacter(_char) => {}

        // Mouse events
        MouseMoved(_pos) => {}
        MousePressed(_button) => {}
        MouseReleased(_button) => {}
        MouseWheel(_amount, _phase) => {}
        MouseEntered => {}
        MouseExited => {}

        // Touch events
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}

        // Window events
        Moved(_pos) => {}
        Resized(_size) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}
