use nannou::prelude::*;
use nannou::noise::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    things: Vec<Thing>,
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

const N_THINGS: usize = 2000;

fn model(app: &App) -> Model {
    let _window = app.new_window().size(1024, 1024).view(view).build().unwrap();
    let mut things= Vec::new();

    for _i in 0..N_THINGS {
        let thing = Thing::new(vec2(
            (random::<f32>() - 0.5) * 1024.0,
            (random::<f32>() - 0.5) * 1024.0
        ));
        things.push(thing);
    }

    let noise = Perlin::new();

    Model { 
        things,
        noise,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32 / 120.0;
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
}

fn view (app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.rect().w_h(1024.0, 1024.0).color(srgba(0.0, 0.0, 0.00, 0.1));

    // draw.background().color(BLACK);
    // draw.ellipse().x_y(100.0, 200.).color(WHITE);

    // for i in 0..10 {
    //     let angle = i as f32 +0.1 / TAU + time;
    //     draw.ellipse().x_y(100.0 * angle.cos(), 100.0 * angle.sin()).color(GREEN);
    // }

    for thing in model.things.iter() {
        draw.polyline().points(thing.positions.iter().cloned()).color(PURPLE);
        // draw.ellipse().xy(thing.position).radius(5.0).color(PURPLE);
    }

    draw.to_frame(app, &frame).unwrap();
}