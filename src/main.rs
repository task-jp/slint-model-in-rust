use std::error::Error;
use std::rc::Rc;
use slint::VecModel;
use slint::Color;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let size = ui.window().size();
    let half_width = size.width / 2;
    let half_height = size.height / 2;

    let model = Rc::new(VecModel::default());
    // insert 100 rectangles with random position and color
    for _ in 0..100 {
        model.push(Rect{
            x: rand::random::<f32>() * half_width as f32,
            y: rand::random::<f32>() * half_height as f32,
            width: 10.0 + rand::random::<f32>() * half_width as f32,
            height: 10.0 + rand::random::<f32>() * half_height as f32,
            color: Color::from_rgb_u8(rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()),
        });
    }
    ui.set_model(model.clone().into()); 

    ui.run()?;

    Ok(())
}
