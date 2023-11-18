use crate::{
    generator::Generator,
    vector2::{get_random_dirs, Vector2, RIGHT, UP},
};

pub struct LayingStick {
    field_len: Vector2,
    pos: Vector2,
    field: Vec<Vec<bool>>,
}

impl LayingStick {
    pub fn new(height: usize, width: usize) -> Self {
        let mut field = vec![vec![false; width]; height];
        // 外周に壁を配置
        for line in field.iter_mut() {
            (line[0], line[width - 1]) = (true, true);
        }
        for j in 1..(width - 1) {
            (field[0][j], field[height - 1][j]) = (true, true);
        }
        for i in (2..height).step_by(2) {
            for j in (2..width).step_by(2) {
                field[i][j] = true;
            }
        }
        Self {
            field_len: Vector2::new(height as i32, width as i32),
            pos: Vector2::new(2, 2),
            field,
        }
    }
}

impl Generator for LayingStick {
    fn in_process(&self) -> bool {
        self.pos.y <= self.field_len.y - 3
    }

    fn proceed(&mut self) {
        for dir in get_random_dirs() {
            // y座標が2より大きい場合、上方向に壁を生成できない
            if self.pos.y > 2 && dir == UP {
                continue;
            }
            // 壁を配置する
            let pos = self.pos + dir;
            if !self.field[pos.y as usize][pos.x as usize] {
                self.field[pos.y as usize][pos.x as usize] = true;
                break;
            }
        }
        self.pos = if self.pos.x < self.field_len.x - 3 {
            self.pos + RIGHT * 2
        } else {
            Vector2::new(self.pos.y + 2, 2)
        };
    }

    fn get_field(&self) -> &Vec<Vec<bool>> {
        &self.field
    }
}
