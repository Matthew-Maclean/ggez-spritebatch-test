use ggez::
{
    self,
    Context, ContextBuilder, GameResult,
    event::{self, EventHandler},
    graphics,
};

fn main() -> GameResult<()>
{
    let (mut ctx, mut eloop) = ContextBuilder::new("sb test", "sb tester")
        .window_mode(ggez::conf::WindowMode
        {
            width: 400f32,
            height: 400f32,
            ..Default::default()
        })
        .build()?;

    let mut game = SBTest::new(&mut ctx, 250)?;

    event::run(&mut ctx, &mut eloop, &mut game)
}

use ggez::graphics::spritebatch::{SpriteBatch, SpriteIdx};
use ggez::input::mouse::MouseButton;

struct SBTest
{
    size: usize,
    batch: SpriteBatch,
    idxs: Vec<Vec<SpriteIdx>>,
    button: Option<MouseButton>,
}

impl SBTest
{
    pub fn new(ctx: &mut Context, size: usize) -> GameResult<SBTest>
    {
        let (batch, idxs) = SBTest::init_batch(ctx, size)?;

        Ok(SBTest
        {
            size: size,
            batch: batch,
            idxs: idxs,
            button: None,
        })
    }

    fn init_batch(ctx: &mut Context, size: usize) -> GameResult<(SpriteBatch, Vec<Vec<SpriteIdx>>)>
    {
        use graphics::{Image, DrawParam};

        let image = Image::from_rgba8(ctx, 1, 1, &[255, 255, 255, 255])?;

        let mut sb = SpriteBatch::new(image);
        
        let mut idxs = vec![Vec::with_capacity(size); size];
        for x in 0..size
        {
            for y in 0..size
            {
                let param = DrawParam::new()
                    .dest(ggez::mint::Point2{ x: x as f32, y: y as f32});

                let idx = sb.add(param);
                idxs[x].push(idx);
            }
        }

        Ok((sb, idxs))
    }

    fn locate_mouse(&self, ctx: &Context, x: f32, y: f32) -> Option<(usize, usize)>
    {
        let (w, h) = graphics::drawable_size(ctx);
        let (sx, sy) = (w / self.size as f32, h / self.size as f32);

        let (cx, cy) = ((x / sx).trunc() as usize, (y / sy).trunc() as usize);

        if cx < self.size && cy < self.size
        {
            Some((cx, cy))
        }
        else
        {
            None
        }
    }

    fn change_cell(&mut self, x: usize, y: usize, colour: graphics::Color) -> GameResult<()>
    {
        let idx = self.idxs[x][y];
        self.batch.set(idx, graphics::DrawParam::new()
            .dest(ggez::mint::Point2{ x: x as f32, y: y as f32 })
            .color(colour))
    }
}


impl EventHandler for SBTest
{
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>
    {
        println!("{:2.2} fps", ggez::timer::fps(ctx));

        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _: f32, _: f32)
    {
        let b = if let Some(b) = self.button { b } else { return };

        let (cx, cy) = if let Some((cx, cy)) = self.locate_mouse(ctx, x, y)
        {
            (cx, cy)
        }
        else
        {
            return
        };

        let colour = match b
        {
            MouseButton::Left   => graphics::Color::from_rgb(255, 0, 0),
            MouseButton::Right  => graphics::Color::from_rgb(0, 255, 0),
            MouseButton::Middle => graphics::Color::from_rgb(0, 0, 255),
            _ => return,
        };

        self.change_cell(cx, cy, colour).unwrap();
    }

    fn mouse_button_down_event(&mut self, _: &mut Context, b: MouseButton, _: f32, _: f32)
    {
        self.button = Some(b);
    }

    fn mouse_button_up_event(&mut self, _: &mut Context, b: MouseButton, _: f32, _: f32)
    {
        let ob = if let Some(b) = self.button { b } else { return };

        if ob == b
        {
            self.button = None;
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>
    {
        graphics::clear(ctx, graphics::BLACK);

        let (w, h) = graphics::drawable_size(ctx);

        graphics::draw(ctx, &self.batch, graphics::DrawParam::new()
            .scale(ggez::mint::Vector2{ x: w / self.size as f32, y: h / self.size as f32}))?;

        graphics::present(ctx)
    }
}
