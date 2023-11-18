use crate::{
    generator::Generator,
    vector2::{get_random_dirs, Vector2},
    walker::Walker,
};
use rand::seq::SliceRandom;

pub struct ExtendingWall {
    walker: Walker,
    walls: Vec<Vector2>,
    field: Vec<Vec<bool>>,
}

impl ExtendingWall {
    pub fn new(height: usize, width: usize) -> Self {
        let mut field = vec![vec![false; width]; height];
        // 外周に壁を配置
        for line in field.iter_mut() {
            (line[0], line[width - 1]) = (true, true);
        }
        for j in 1..(width - 1) {
            (field[0][j], field[height - 1][j]) = (true, true);
        }
        let mut walls: Vec<Vector2> = Vec::new();
        for i in (2..(height - 1)).step_by(2) {
            for j in (2..(width - 1)).step_by(2) {
                walls.push(Vector2::new(i as i32, j as i32));
            }
        }
        walls.shuffle(&mut rand::thread_rng());
        let pos = walls.pop().unwrap();
        let field_len = Vector2::new(height as i32, width as i32);
        field[pos.y as usize][pos.x as usize] = true;
        let walker = Walker::new(field_len, pos, Box::new(get_random_dirs));
        Self {
            walker,
            walls,
            field,
        }
    }
}

impl Generator for ExtendingWall {
    fn in_process(&self) -> bool {
        !self.walker.is_empty() || !self.walls.is_empty()
    }

    fn proceed(&mut self) {
        // 新しい座標から壁伸ばしを開始
        if self.walker.is_empty() {
            while let Some(pos) = self.walls.pop() {
                // すでに壁ならスキップ
                if self.field[pos.y as usize][pos.x as usize] {
                    continue;
                }
                self.field[pos.y as usize][pos.x as usize] = true;
                // 新しい座標を設定
                let field_len = Vector2::new(self.field.len() as i32, self.field[0].len() as i32);
                self.walker = Walker::new(field_len, pos, Box::new(get_random_dirs));
                break;
            }
            return;
        }
        // 壁を伸ばしを続ける
        while !self.walker.is_empty() {
            let result = self.walker.walk(|_| true);
            if let Some((pos, new_pos)) = result {
                // 次の座標が壁なら壁伸ばし終了
                if self.field[new_pos.y as usize][new_pos.x as usize] {
                    self.walker.clear();
                }
                // 進行方向へ道を進める
                let inter_pos = (pos + new_pos) / 2;
                self.field[inter_pos.y as usize][inter_pos.x as usize] = true;
                self.field[new_pos.y as usize][new_pos.x as usize] = true;
                break;
            }
        }
    }

    fn get_field(&self) -> &Vec<Vec<bool>> {
        &self.field
    }
}
