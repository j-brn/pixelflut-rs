use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use image::imageops::FilterType;
use image::GenericImageView;
use std::env;
use std::sync::Arc;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_path = env::var("IMAGE").expect("IMAGE must be set");
    let host = env::var("HOST").expect("HOST must be set");
    let offset_x = env::var("OFFSET_X")
        .expect("X must be set")
        .parse::<i32>()?;
    let offset_y = env::var("OFFSET_Y")
        .expect("Y must be set")
        .parse::<i32>()?;
    let connections = env::var("CONNECTIONS")
        .expect("CONNECTIONS must be set")
        .parse::<u32>()?;
    let height = env::var("HEIGHT")
        .expect("HEIGHT must be set")
        .parse::<u32>()?;
    let width = env::var("WIDTH")
        .expect("WIDTH must be set")
        .parse::<u32>()?;

    let image = image::open(image_path)?;
    let instructions = Arc::new(
        image
            .resize(width, height, FilterType::Gaussian)
            .pixels()
            .map(|(x, y, pixel)| {
                let (r, g, b, a) = (pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]);

                format!(
                    "PX {} {} {:x}{:x}{:x}{:x}\n",
                    offset_x as i32 + x as i32,
                    offset_y as i32 + y as i32,
                    r,
                    g,
                    b,
                    a
                )
            })
            .collect::<Vec<String>>(),
    );

    let mut tasks = Vec::new();
    let draw_offset = (height * height / connections) as usize;

    for i in 0..connections {
        let host = host.clone();
        let mut conn = TcpStream::connect(host).await.unwrap();
        let instructions = instructions.clone();

        tasks.push(task::spawn(async move {
            for instruction in instructions.iter().cycle().skip(i as usize * draw_offset) {
                if let Err(e) = conn.write_all(instruction.as_bytes()).await {
                    eprintln!("{}", e)
                }
            }
        }))
    }

    for task in tasks {
        task.await;
    }

    Ok(())
}
