use crate::prelude::*;

pub struct RPU {
    pub world               : World,
    pub preview             : World,

    pub context             : Context,
    pub stack               : Vec<Value>,

    pub dictionary          : FxHashMap<String, Vec<Value>>
}

impl RPU {
    pub fn new() -> Self {

        let world = World::new();
        let preview = World::new();

        let context: Context = Context::new();

        let dictionary = FxHashMap::default();

        Self {
            world,
            preview,

            context,
            stack           : vec![],

            dictionary,
        }
    }

    pub fn scan(&mut self, scanner: &mut Scanner, buffer: &mut ColorBuffer) -> (bool, Vec<String>) {

        let mut output_image = false;
        let mut output_text = vec![];
        let mut error = false;

        loop {
            let token = scanner.scan_token(false);

            let kind = token.kind;

            if kind == TokenType::Colon {
                println!("cc {}", token.lexeme);
            } else
            if kind == TokenType::Eof {
                break;
            } else
            if kind == TokenType::Dot {
                if let Some(v) = self.stack.pop() {
                    output_text.push(v.to_string());
                } else {
                    output_text.push(format!("Stack is empty.").to_string());
                    error = true;
                }
            } else
            if kind == TokenType::Number {
                if let Some(n) = token.lexeme.parse::<f32>().ok() {
                    self.stack.push(Value::Number(n));
                }
            } else
            if kind == TokenType::Identifier {
                if token.lexeme == "SPHERE" {
                    let mut sdf = SDF3D::new(SDF3DType::Sphere);
                    if let Some(err) = sdf.read_properties(&mut self.stack).err() {
                        output_text.push(err);
                        error = true;
                    } else {
                        output_image = true;
                        // self.preview.clear();
                        self.preview.compile(sdf, Vec3f::new(0.5, 0.0
                            , 0.5));
                        self.preview.render(buffer, &mut self.context, 0);
                    }
                }
            }

            //output_text.push(format!("{:?}", token).to_string());
        }

        (output_image, output_text)
    }

    pub fn render_world(&mut self, buffer: &mut ColorBuffer) {

        self.world.render(buffer, &mut self.context, 0);
        return;

        // let width = buffer.width;
        // let height = buffer.height as f32;

        // let screen = vec2f(buffer.width as f32, buffer.height as f32);

        //let time = (iteration as f32 * 1000.0 / 60.0) / 1000.0;
        //let _start = self.get_time();
        /*
        buffer.pixels
            .par_rchunks_exact_mut(width * 4)
            .enumerate()
            .for_each(|(j, line)| {
                for (i, pixel) in line.chunks_exact_mut(4).enumerate() {

                    #[inline(always)]
                    pub fn _mix_color(a: &[f32], b: &[f32], v: f32) -> [f32; 4] {
                        [   (1.0 - v) * a[0] + b[0] * v,
                            (1.0 - v) * a[1] + b[1] * v,
                            (1.0 - v) * a[2] + b[2] * v,
                            (1.0 - v) * a[3] + b[3] * v ]
                    }

                    let i = j * width + i;

                    let x = (i % width) as f32;
                    let y = height - (i / width) as f32;

                    let uv = vec2f(x / width as f32, 1.0 - (y / height));

                    let color = [uv.x, uv.y, 0.0, 1.0];

                    pixel.copy_from_slice(&color);
                }
        });*/

    }

    pub fn render_palette(&mut self, buffer: &mut ColorBuffer) {

        let width = buffer.width;
        let height = buffer.height as f32;

        // let screen = vec2f(buffer.width as f32, buffer.height as f32);

        // let time = (iteration as f32 * 1000.0 / 60.0) / 1000.0;
        // let _start = self.get_time();

        buffer.pixels
            .par_rchunks_exact_mut(width * 4)
            .enumerate()
            .for_each(|(j, line)| {
                for (i, pixel) in line.chunks_exact_mut(4).enumerate() {

                    #[inline(always)]
                    pub fn _mix_color(a: &[f32], b: &[f32], v: f32) -> [f32; 4] {
                        [   (1.0 - v) * a[0] + b[0] * v,
                            (1.0 - v) * a[1] + b[1] * v,
                            (1.0 - v) * a[2] + b[2] * v,
                            (1.0 - v) * a[3] + b[3] * v ]
                    }

                    let i = j * width + i;

                    let x = (i % width) as f32;
                    let y = height - (i / width) as f32;

                    let uv = vec2f(x / width as f32, 1.0 - (y / height));

                    let color = [uv.x, uv.y, 0.0, 1.0];

                    pixel.copy_from_slice(&color);
                }
        });

    }
}