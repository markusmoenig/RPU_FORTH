use crate::prelude::*;
use rand::{thread_rng, Rng, rngs::ThreadRng};

pub struct World {
    pub camera              : Camera,

    pub map                 : Map,
}

impl World {
    pub fn new() -> Self {

        let camera = Camera::new(vec3f(0.0, 1.0, 2.0), Vec3f::new(0.0, 0.0, 0.0), 120.0);

        let mut map = Map::new();
        map.build_aabb();

        Self {
            camera,

            map,
        }
    }

    pub fn render(&self, buffer: &mut ColorBuffer, context: &Context, iteration: i32) {

        let width = buffer.width;
        let height = buffer.height as f32;

        let screen = vec2f(buffer.width as f32, buffer.height as f32);

        //let time = (iteration as f32 * 1000.0 / 60.0) / 1000.0;
        let _start = self.get_time();

        buffer.pixels
            .par_rchunks_exact_mut(width * 4)
            .enumerate()
            .for_each(|(j, line)| {
                for (i, pixel) in line.chunks_exact_mut(4).enumerate() {

                    #[inline(always)]
                    pub fn mix_color(a: &[f32], b: &[f32], v: f32) -> [f32; 4] {
                        [   (1.0 - v) * a[0] + b[0] * v,
                            (1.0 - v) * a[1] + b[1] * v,
                            (1.0 - v) * a[2] + b[2] * v,
                            (1.0 - v) * a[3] + b[3] * v ]
                    }

                    let i = j * width + i;

                    let x = (i % width) as f32;
                    let y = height - (i / width) as f32;

                    let uv = vec2f(x / width as f32, 1.0 - (y / height));

                    let mut rng: ThreadRng = thread_rng();

                    // let cam_off = hash3_2(vec3f(time, uv.x, uv.y));
                    let cam_off = vec2f(rng.gen(), rng.gen());
                    //let cam_off = vec2f(0.5, 0.5);
                    // let ray = self.camera.create_ray(uv, screen, cam_off);

                    let mut ray;

                    if context.iso_state {
                        ray = self.camera.create_iso_ray(uv, screen, cam_off);
                    } else {
                        ray = self.camera.create_ray(uv, screen, cam_off);
                    }

                    let mut color = [0.0, 0.0, 0.0, 1.0];
                    let mut hit_something = false;

                    if let Some(aabb) = &self.map.aabb {
                        if self.map.ray_aabb(&ray, aabb) == true {
                            if context.render_state == false {
                                color = [0.0, 0.0, 0.0, 1.0];

                                if let Some(hit) = self.dda_recursive(&ray) {
                                    //color = [hit.normal.x.abs(), hit.normal.y.abs(), hit.normal.z.abs(), 1.0];
                                    color = context.palette.at_f_to_linear(hit.value.0);
                                    hit_something = true;
                                }
                            } else {

                                /*
                                let max_depth = 2;

                                let mut acc = Vec3f::zero();
                                let mut mask = Vec3f::one();

                                let pi = std::f32::consts::PI;
                                let mut hit_something = false;
                                /*
                                for _depth in 0..max_depth {

                                    if let Some(hit) = self.dda_recursive(&ray) {

                                        hit_something = true;

                                        let n = hit.normal;
                                        let nl = n * signum(-dot(n, ray.d));

                                        let material = &context.materials[hit.value.1 as usize];

                                        let roughness = material.roughness;
                                        let alpha = roughness * roughness;
                                        let metallic = material.metallic;
                                        let reflectance = material.reflectance;
                                        let diffuse = context.palette.at_vec_to_linear(hit.value.0);
                                        let color = diffuse;
                                        let emission = material.emission * diffuse;

                                        let mut brdf = vec3f(0.0, 0.0, 0.0);

                                        let light_pos = vec3f(0.0, 6.0, 0.0);
                                        let light_radius = 1.0;
                                        let light_emission = vec3f(200.0, 200.0, 200.0);

                                        let x = hit.hitpoint - 0.005 * n;

                                        if reflectance == 1.0 || rng.gen::<f32>() < reflectance {

                                            #[inline(always)]
                                            pub fn mix(a: &f32, b: &f32, v: f32) -> f32 {
                                                (1.0 - v) * a + b * v
                                            }

                                            let l0 = light_pos - x;
                                            let cos_a_max = sqrt(1. - clamp(light_radius * light_radius / dot(l0, l0), 0.0, 1.0));
                                            let cosa = mix(&cos_a_max, &1.0, rng.gen());
                                            let l = jitter(l0, 2.0 * pi * rng.gen::<f32>(), sqrt(1.0 - cosa*cosa), cosa);

                                            if let Some(_hit_refl) = self.dda_recursive(&Ray::new(x, l)) {

                                            } else {
                                                // No hit, we assume we hit it for now

                                                let omega = 2.0 * pi * (1.0 - cos_a_max);
                                                brdf += (light_emission * clamp(ggx(nl, ray.d, l, roughness, metallic),0.0,1.0) * omega) / pi;
                                            }

                                            let xsi_1 = rng.gen::<f32>();
                                            let xsi_2 = rng.gen::<f32>();
                                            let phi = atan((alpha * sqrt(xsi_1)) / sqrt(1.0 - xsi_1));
                                            let theta = 2.0 * pi * xsi_2;
                                            let direction = angle_to_dir(nl, theta, phi);
                                            ray = Ray::new(x, direction);
                                            acc += mask * emission + mask * color * brdf;
                                            mask *= color;
                                        } else {

                                            #[inline(always)]
                                            pub fn mix(a: &f32, b: &f32, v: f32) -> f32 {
                                                (1.0 - v) * a + b * v
                                            }

                                            let r2 = rng.gen();
                                            let d = jitter(nl, 2.0 * pi * rng.gen::<f32>(), sqrt(r2), sqrt(1.0 - r2));
                                            let mut e = Vec3f::zero();

                                            let l0 = light_pos - x;

                                            let cos_a_max = sqrt(1.0 - clamp(light_radius * light_radius / dot(l0, l0), 0., 1.));
                                            let cosa = mix(&cos_a_max, &1.0, rng.gen());
                                            let l = jitter(l0, 2.0 * pi * rng.gen::<f32>(), sqrt(1.0 - cosa * cosa), cosa);

                                            if let Some(_hit_refl) = self.dda_recursive(&Ray::new(x, l)) {

                                            } else {
                                                // No hit, we assume we hit it for now

                                                let omega = 2.0 * pi * (1.0 - cos_a_max);
                                                e += (light_emission * clamp(dot(l, n),0.0,1.0) * omega) / pi;
                                            }

                                            acc += mask * emission + mask * color * e;
                                            mask *= color;
                                            ray = Ray::new(x, d);
                                        }
                                    } else {
                                        acc += mask * vec3f(0.5, 0.5, 0.5);
                                        break;
                                    }
                                }*/

                                color = [acc.x, acc.y, acc.z, 1.0];

                                if hit_something {
                                    // Clip color to the palette
                                    let index = context.palette.closest(color[0].powf(0.4545), color[1].powf(0.4545), color[2].powf(0.4545));
                                    color = context.palette.at_f(index);
                                } else {
                                    color = [0.15, 0.15, 0.15, 1.0];
                                }*/
                            }
                        }
                    }

                    if hit_something == false {
                        let normal = vec3f(0.0, 1.0, 0.0);
                        let denom = dot(normal, ray.d);

                        if denom.abs() > 0.0001 {
                            let t = dot(Vec3f::zero() - ray.o, normal) / denom;
                            if t >= 0.0 {
                                let plane_hit = ray.at(t);

                                if (plane_hit.z.floor().abs() as i32 % 2) == 0 {
                                    if (plane_hit.x.floor().abs() as i32 % 2) == 0 {
                                        color = [0.1, 0.1, 0.1, 0.5];
                                    } else {
                                        color = [0.15, 0.15, 0.15, 0.5];
                                    }
                                } else {
                                    if (plane_hit.x.floor().abs() as i32 % 2) == 1 {
                                        color = [0.1, 0.1, 0.1, 0.5];
                                    } else {
                                        color = [0.15, 0.15, 0.15, 0.5];
                                    }
                                }
                            }
                        }
                    }

                    // Accumulate
                    let mix = mix_color(pixel, &color, 1.0 / (iteration + 1) as f32);
                    pixel.copy_from_slice(&mix);
                }
        });

        let _stop = self.get_time();
        //println!("renter time {:?}, iter: {}", _stop - _start, iteration);
    }

    pub fn hit_at(&self, pos: Vec2f, buffer: &ColorBuffer, iso_state: bool) -> Option<HitRecord> {

        let x: f32 = pos.x / buffer.width as f32;
        let y: f32 = pos.y / buffer.height as f32;

        let screen = vec2f(buffer.width as f32, buffer.height as f32);

        let uv = vec2f(x, 1.0 - y);

        let ray;
        if iso_state {
            ray = self.camera.create_iso_ray(uv, screen, vec2f(0.5, 0.5));
        } else {
            ray = self.camera.create_orbit_ray(uv, screen, vec2f(0.5, 0.5));
        }

        if let Some(hit) = self.dda_recursive(&ray) {
            Some(hit)
        } else {
            None
        }
    }

    fn _dda(&self, ray: &Ray) -> Option<HitRecord> {

        // Based on https://www.shadertoy.com/view/ct33Rn

        fn equal(l: f32, r: Vec3f) -> Vec3f {
            vec3f(
                if l == r.x { 1.0 } else { 0.0 },
                if l == r.y { 1.0 } else { 0.0 },
                if l == r.z { 1.0 } else { 0.0 },
            )
        }

        let ro = ray.o;
        let rd = ray.d;

        let mut i = floor(ro);
        let mut dist = 0.0;

        let mut normal = Vec3f::zero();
        let srd = signum(rd);

        let rdi = 1.0 / (2.0 * rd);
        let mut hit = false;

        let mut key: Vec3<i32> = Vec3i::zero();

        for _ii in 0..20 {
            key = Vec3i::from(i);

            if self.map.tiles.contains_key(&(key.x, key.y, key.z)) {
                hit = true;
                break;
            }

            let plain = (1.0 + srd - 2.0 * (ro - i)) * rdi;
            dist = min(plain.x, min(plain.y, plain.z));
            normal = equal(dist, plain) * srd;
            i += normal;
        }

        if hit {
            let mut hit_record = HitRecord::new();

            hit_record.hitpoint = ray.at(dist);
            hit_record.key = key;
            hit_record.distance = dist;
            hit_record.normal = normal;

            Some(hit_record)
        } else {
            None
        }

        /*
        vec3 i = floor(ro);
        float dist;
        int ii;
        vec3 normal;
        vec3 srd = sign(rd);
        vec3 rdi = 1./(2.*rd);
        for (ii = 0; ii < 200; ii += 1) {
            if (getBlock(i)) break;
            vec3 plain = ((1.+srd-2.*(ro-i))*rdi);
            dist = min(plain.x, min(plain.y, plain.z));
            normal = vec3(equal(vec3(dist), plain))*srd;
            i += normal;
        }
        if (ii == 200) return hit(vec3(-1), -1., vec3(-1.0));
        vec3 position = ro+rd*dist;
        return hit(normal, dist, position);*/

    }

    fn dda_recursive(&self, ray: &Ray) -> Option<HitRecord> {

        // Based on https://www.shadertoy.com/view/ct33Rn

        fn equal(l: f32, r: Vec3f) -> Vec3f {
            vec3f(
                if l == r.x { 1.0 } else { 0.0 },
                if l == r.y { 1.0 } else { 0.0 },
                if l == r.z { 1.0 } else { 0.0 },
            )
        }

        let ro = ray.o;
        let rd = ray.d;

        let mut i = floor(ro);
        let mut dist = 0.0;

        let mut normal;//= Vec3f::zero();
        let srd = signum(rd);

        let rdi = 1.0 / (2.0 * rd);

        let mut key: Vec3<i32>;// = Vec3i::zero();

        for _ii in 0..20 {
            key = Vec3i::from(i);

            if let Some(tile) = self.map.tiles.get(&(key.x, key.y, key.z)) {

                let mut lro = ray.at(dist);
                lro -= Vec3f::from(key);
                lro *= tile.size as f32;
                lro = lro - rd * 0.01;

                if let Some(mut hit) = tile.dda(&Ray::new(lro, rd)) {
                    hit.key = key;
                    hit.hitpoint = ray.at(dist + hit.distance / (tile.size as f32));
                    hit.distance = dist;
                    return Some(hit);
                }
            }

            let plain = (1.0 + srd - 2.0 * (ro - i)) * rdi;
            dist = min(plain.x, min(plain.y, plain.z));
            normal = equal(dist, plain) * srd;
            i += normal;
        }

        None
        /*
        if hit {
            let mut hit_record = HitRecord::new();

            hit_record.hitpoint = ray.at(dist);
            hit_record.key = key;
            hit_record.distance = dist;
            hit_record.normal = normal;

            Some(hit_record)
        } else {
            None
        }*/
    }


    /// Get a tile
    pub fn get_tile(&self, at: Vec3i) -> Option<Tile> {
        if let Some(tile) = self.map.tiles.get(&(at.x, at.y, at.z)) {
            Some(tile.clone())
        } else {
            None
        }
    }

    /// Set a tile
    pub fn set_tile(&mut self, at: Vec3i, mut tile: Tile) {
        tile.build_aabb();
        self.map.tiles.insert((at.x, at.y, at.z), tile);
    }

    /// Set the tile we are looking at
    pub fn set_focus(&mut self, at: Vec3i) {
        self.camera.center.x = at.x as f32 + 0.5;
        self.camera.center.y = at.y as f32 + 0.5;
        self.camera.compute_orbit(vec2f(0.0, 0.0));
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn compile(&mut self, sdf: SDF3D, position: Vec3f) {
        let bbox = sdf.create_bbox(position);

        let tiles: Vec<Vec3<i32>> = self.map.create_tiles_aabb(&bbox);
        // let tiles: Vec<Vec3<i32>> = vec![vec3i(0, 0, 0)];

        //println!("{:?}", tiles);

        for tile_key in &tiles {
            if let Some(mut tile) = self.get_tile(*tile_key) {
                let size = tile.size;

                println!("{}", tile_key);

                for y in 0..size {
                    for x in 0..size {
                        for z in 0..size {
                            let pos = self.to_world_coord(*tile_key, vec3i(x as i32, y as i32, z as i32));

                            /*

                            let p = pos - vec3f(0.0, 0.2, 0.0) - hp; let b = vec3f(0.2, 0.2, 0.2);
                            let q = abs(p) - b;
                            let d1 = length(max(q,vec3f(0.0, 0.0, 0.0))) + min(max(q.x,max(q.y,q.z)),0.0);
                            if d1 < 0.0 {
                                tile.set_voxel(x, y, z, Some((10, 10)));
                            }*/

                            /*
                            let p = abs(pos.xy() - vec2f(0.0, 0.3) - hp.xy()) - vec2f(0.2, 0.2);
                            let mut d = length(max(p,Vec2f::new(0.0, 0.0))) + min(max(p.x,p.y),0.0);

                            d = abs(d) - 0.05;

                            let h = 0.2;
                            let w = vec2f( d, abs(pos.z - hp.z) - h );
                            d = min(max(w.x,w.y),0.0) + length(max(w,vec2f(0.0, 0.0)));
                            */

                            let d = sdf.distance(pos, position);


                            if d < 0.0 {
                                tile.set_voxel(x, y, z, Some((sdf.get_color(), 10)));
                            }
                        }
                    }
                }

                self.set_tile(*tile_key, tile);
            }
        }

        self.map.build_aabb();
    }

    /// Converts the hit keys to a world coordinate
    pub fn to_world_coord(&self, key: Vec3i, tile_key: Vec3i) -> Vec3f {
        let mut wc = Vec3f::from(key);

        let size = Map::tile_size() as f32;
        wc.x += tile_key.x as f32 / size;
        wc.y += tile_key.y as f32 / size;
        wc.z += tile_key.z as f32 / size;

        wc
    }

    /// Converts the world coordinate to hit keys
    pub fn to_tile_coord(&self, wc: Vec3f) -> (Vec3i, Vec3i) {
        let size = Map::tile_size() as f32;
        let key = Vec3i::new(wc.x as i32, wc.y as i32, wc.z as i32);
        let tile_key = Vec3i::new((frac(wc.x) * size) as i32, (frac(wc.y) * size) as i32, (frac(wc.z) * size) as i32);
        (key, tile_key)
    }

    /// Gets the current time in milliseconds
    fn get_time(&self) -> u128 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let stop = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
            stop.as_millis()
    }

}