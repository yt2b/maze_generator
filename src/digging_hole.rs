use crate::{
    generator::Generator,
    vector2::{get_random_dirs, Vector2},
    walker::Walker,
};
use rand::Rng;

pub struct DiggingHole {
    walker: Walker,
    field: Vec<Vec<bool>>,
}

impl DiggingHole {
    pub fn new(height: usize, width: usize) -> Self {
        let mut rng = rand::thread_rng();
        let field_len = Vector2::new(height as i32, width as i32);
        let y = 1 + rng.gen_range(0, (height - 1) / 2) as i32 * 2;
        let x = 1 + rng.gen_range(0, (width - 1) / 2) as i32 * 2;
        let pos = Vector2::new(y, x);
        let walker = Walker::new(field_len, pos, Box::new(get_random_dirs));
        let mut field = vec![vec![true; width]; height];
        field[pos.y as usize][pos.x as usize] = false;
        Self { walker, field }
    }
}

impl Generator for DiggingHole {
    fn in_process(&self) -> bool {
        !self.walker.is_empty()
    }

    fn proceed(&mut self) {
        // 範囲内の壁を探す
        let result = self
            .walker
            .walk(|next_pos| self.field[next_pos.y as usize][next_pos.x as usize]);
        if let Some((pos, new_pos)) = result {
            // 進行方向へ道を進める
            let inter_pos = (pos + new_pos) / 2;
            self.field[inter_pos.y as usize][inter_pos.x as usize] = false;
            self.field[new_pos.y as usize][new_pos.x as usize] = false;
        }
    }

    fn get_field(&self) -> &Vec<Vec<bool>> {
        &self.field
    }
}
