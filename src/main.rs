use piston_window::*;
use rand::Rng;
const STEP:f64=20.0;            // Size of each character
const WIDTH:f64=640.0;          // Size of the Window : WIDTH
const HEIGHT:f64=480.0;         // Size of the Window : HEIGHT

// Rain Object
struct Rain{
    chars:Vec<char>,            // Vector of characters
    opacity:f32,                // Opacity of the rain characters
    x:f64,                      // x-coordinate of the rain
    y:f64,                      // y-coordinate of the rain 
    timer:f64,                  // Timer for updating the rain
    max:f64                     // Maximum timer value
}

/*
Implementation of the Rain
*/
impl Rain{
    // Advances the rain by one step
    fn next<R: Rng + ?Sized>(&mut self,rng:&mut R){
        self.chars.remove(0);
        self.y+=STEP;
        self.add(rng)
    }

    // Adds a new random character to the rain
    fn add<R: Rng + ?Sized>(&mut self,rng:&mut R){
        let seed:u32=(rng.gen::<u32>()%94)+33;
        let c=std::char::from_u32(seed).unwrap();
        self.chars.push(c)
    }

    // Constructs a new Rain object with random parameters
    fn new<R: Rng + ?Sized>(rng:&mut R,ry:bool) -> Rain{
        let y;
        let l=(rng.gen::<usize>()%12)+5;
        if ry{ y=rng.gen::<f64>()*HEIGHT }
        else{ y=-(l as f64)*STEP }
        let mut r=Rain{
            chars:Vec::new(),
            x:rng.gen::<f64>()*WIDTH,
            max:(rng.gen::<f64>()*0.1)+0.05,
            opacity:(rng.gen::<f32>()*0.9)+0.1,
            timer:0.0,
            y:y
        };
        for _ in 0..l{ r.add(rng) };
        r
    }
}

// Generates a vector of 80 Rain objects
fn get_rains<R: Rng + ?Sized>(rng:&mut R) -> Vec<Rain>{
    let mut v:Vec<Rain>=Vec::new();
    for _ in 0..80{ v.push(Rain::new(rng,true)) }
    v
}

fn main(){
    // Initialize a new ThreadRng
    let mut rng=rand::thread_rng();

    // Initialize a new window of size (WIDTH = 640, HEIGHT = 480)
    let mut window:PistonWindow=WindowSettings::new("Matrix Rain Project",(WIDTH,HEIGHT))
        .exit_on_esc(true).build().unwrap_or_else(|e|{
            panic!("Failed to build PistonWindow: {}",e)
        });

    // Set new font = "Atop-R99O3"
    let mut glyphs=window.load_font("Atop-R99O3.ttf").unwrap();
    let mut rains=get_rains(&mut rng);
    let mut buffer=String::from("A");

    // Start the main loop
    println!("Launching matrix rain...");
    while let Some(e)=window.next(){
        e.update(|u|{
            let r=&mut rains;
            let rng1=&mut rng;
            for i in 0..r.len(){
                r[i].timer+=u.dt;
                if r[i].timer>=r[i].max{
                    r[i].timer-=r[i].max;
                    r[i].next(rng1);
                    if r[i].y>HEIGHT{
                        r.remove(i);
                        r.push(Rain::new(rng1,false));
                    }
                }
            }
        });

        // 
        window.draw_2d(&e,|c,g,d|{
            let r=&rains;
            let s=&mut buffer;
            clear([0.0,0.0,0.0,1.0],g);
            for i in 0..r.len(){
                for j in 0..r[i].chars.len(){
                    s.clear();
                    s.push(r[i].chars[j]);
                    let transform=c.transform.trans(r[i].x,r[i].y+(STEP*j as f64));
                    text([0.0,1.0,0.0,r[i].opacity],STEP as u32,s.as_str(),&mut glyphs,transform,g).unwrap();
                }
            }
            glyphs.factory.encoder.flush(d)
        });
    }
}