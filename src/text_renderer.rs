use piston_window::types::Color;
use piston_window::{Context, G2d, GfxDevice, Glyphs, PistonWindow, Transformed};

extern crate find_folder;

pub struct Renderer {
  cache: Box<Glyphs>,
}

type RenderStrings = Vec<TextWithProps>;

impl Renderer {
  pub fn new(window: &mut PistonWindow) -> Renderer {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
      .for_folder("assets")
      .unwrap();
    println!("{:?}", assets);
    let cache = window
      .load_font(assets.join("AtariClassic-gry3.ttf"))
      .unwrap();
    let cache = Box::new(cache);
    Renderer { cache }
  }

  pub fn draw_text(&mut self, s: &str, props: &TextProps, c: &Context, g: &mut G2d) {
    // Position
    let (text_x, text_y) = props.position;
    let transform = c.transform.trans(text_x, text_y);

    // Draw text
    piston_window::text::Text::new_color(props.color, props.size)
      .draw(s, self.cache.as_mut(), &c.draw_state, transform, g)
      .unwrap();
  }

  fn _draw_text(&self, cache: &mut Glyphs, s: &str, props: &TextProps, c: &Context, g: &mut G2d) {
    // Position
    let (text_x, text_y) = props.position;
    let transform = c.transform.trans(text_x, text_y);

    // Draw text
    piston_window::text::Text::new_color(props.color, props.size)
      .draw(s, cache, &c.draw_state, transform, g)
      .unwrap();
  }

  pub fn flush(&mut self, device: &mut GfxDevice) {
    self.cache.factory.encoder.flush(device);
  }

  pub fn add_text(&mut self, strings: &mut RenderStrings, s: &str, props: &TextProps) {
    strings.push(TextWithProps {
      text: String::from(s),
      props: props.clone(),
    })
  }

  pub fn draw(
    &mut self,
    strings: &RenderStrings,
    c: &Context,
    g: &mut G2d,
    device: &mut GfxDevice,
  ) {
    for string in strings.iter() {
      self.draw_text(&string.text, &string.props, c, g);
    }
    self.flush(device);
  }
}

#[derive(Clone)]
pub struct TextProps {
  pub position: (f64, f64),
  pub color: Color,
  pub size: u32,
}

#[derive(Clone)]
pub struct TextWithProps {
  props: TextProps,
  text: String,
}
