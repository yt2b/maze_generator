use crate::vector2::Vector2;

pub struct Walker {
    field_len: Vector2,
    stack: Vec<(Vector2, Vec<Vector2>)>,
    get_dirs: Box<dyn Fn() -> Vec<Vector2>>,
}

impl Walker {
    pub fn new(field_len: Vector2, pos: Vector2, get_dirs: Box<dyn Fn() -> Vec<Vector2>>) -> Self {
        let stack = vec![(pos, get_dirs())];
        Walker {
            field_len,
            stack,
            get_dirs,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn walk(&mut self, is_valid: impl Fn(Vector2) -> bool) -> Option<(Vector2, Vector2)> {
        while let Some((pos, mut dirs)) = self.stack.pop() {
            // 次の進行方向を決める
            let next_dir = dirs.iter().position(|d| {
                let new_pos = pos + *d * 2;
                // 範囲内か
                let is_inside = (0 <= new_pos.y && new_pos.y < self.field_len.y)
                    && (0 <= new_pos.x && new_pos.x < self.field_len.x);
                // まだ通っていない座標か
                let is_new_pos = self.stack.iter().all(|(p, _)| new_pos != *p);
                is_inside && is_new_pos && is_valid(new_pos)
            });
            if let Some(idx) = next_dir {
                // 現在の座標を保存
                let dir = dirs.remove(idx);
                self.stack.push((pos, dirs));
                // 次の座標を保存する
                let new_pos = pos + dir * 2;
                self.stack.push((new_pos, (self.get_dirs)()));
                return Some((pos, new_pos));
            }
        }
        None
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::Walker;
    use crate::vector2::{Vector2, DOWN, LEFT, RIGHT, UP};

    #[test]
    fn test_walker() {
        let field = [
            [false, true, true],
            [false, true, true],
            [false, false, false],
        ];
        let get_dirs = Box::new(|| vec![UP, DOWN, LEFT, RIGHT]);
        let mut walker = Walker::new(Vector2::new(3, 3), Vector2::new(0, 0), get_dirs);
        for expected in [
            Some((Vector2::new(0, 0), Vector2::new(2, 0))),
            Some((Vector2::new(2, 0), Vector2::new(2, 2))),
            None,
        ] {
            let actual = walker.walk(|new_pos| !field[new_pos.y as usize][new_pos.x as usize]);
            assert_eq!(actual, expected);
        }
    }
}
