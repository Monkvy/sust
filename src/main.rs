
fn events(_: winit::event::Event<()>) {
    
}


fn main() -> Result<(), sust::err::Error> {
    let a = sust::Vector(4, 2);
    let b = sust::Vector(2.5, 3.25);

    println!("{:?}", a.cast::<f32>() + b);

    let window = sust::window::new("Sust window", sust::Vector(400, 400), Some(sust::Vector(400, 400)))?;
    window.run(events)?;

    Ok(())
}
