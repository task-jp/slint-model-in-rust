use std::error::Error;
use std::rc::Rc;
use slint::VecModel;
use slint::Color;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let size = ui.window().size();

    let model = Rc::new(VecModel::default());
    for _ in 0..1000 {
        let width = rand::random::<f32>() * size.width as f32 * 0.5;
        let height = rand::random::<f32>() * size.height as f32 * 0.5;
        model.push(Rect{
            x: rand::random::<f32>() * (size.width as f32 - width),
            y: rand::random::<f32>() * (size.height as f32 - height),
            width: width,
            height: height,
            color: Color::from_rgb_u8(rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()),
        });
    }
    ui.set_model(model.clone().into()); 

    ui.run()?;

    Ok(())
}
