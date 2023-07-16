use std::ops::{Index, IndexMut};

// x, y
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Int2 {
    pub x: usize,
    pub y: usize,
}

impl Int2 {
    pub fn with_x(mut self, x: usize) -> Int2 {
        self.x = x;
        self
    }

    pub fn with_y(mut self, y: usize) -> Int2 {
        self.y = y;
        self
    }
}

impl From<(usize, usize)> for Int2 {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array2D<T> {
    pub width: usize,
    pub height: usize,
    cols: Vec<Vec<T>>,
}

impl<T: Clone> Array2D<T> {
    pub fn transpose(&self) -> Array2D<T> {
        Array2D {
            width: self.height,
            height: self.width,
            cols: transpose_vecs(&self.cols),
        }
    }

    pub fn rows(&self) -> Vec<(Vec<T>, Int2)> {
        (0..self.height)
            .map(|row_index| {
                let row: Vec<T> = self.cols.iter().map(|col| col[row_index].clone()).collect();
                (row, Int2 { x: 0, y: row_index })
            })
            .collect()
    }

    pub fn cols(&self) -> Vec<(Vec<T>, Int2)> {
        (0..self.width)
            .map(|col_index| {
                let col: Vec<T> = self.cols[col_index].clone();
                (col, Int2 { x: col_index, y: 0 })
            })
            .collect()
    }

    /// only diags that are 3 or longer
    pub fn diags(&self, min_len: usize) -> Vec<(Vec<T>, Int2)> {
        let mut diag_start_points: Vec<Int2> = vec![];
        for i in 0..(self.height + 1 - min_len) {
            diag_start_points.push(Int2 { x: 0, y: i });
        }
        for i in 1..(self.width + 1 - min_len) {
            diag_start_points.push(Int2 { x: i, y: 0 });
        }
        diag_start_points
            .into_iter()
            .map(|start_point| {
                let Int2 { mut x, mut y } = start_point;
                let travel = (self.width - x).min(self.height - y);
                let mut diag: Vec<T> = vec![];
                for _ in 0..travel {
                    let c = self[Int2 { x, y }].clone();
                    diag.push(c);
                    x += 1;
                    y += 1;
                }
                (diag, start_point)
            })
            .collect()
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Array2D<T> {
    type Error = ();

    fn try_from(cols: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let width = cols.len();
        let height = cols[0].len();
        if cols.iter().any(|c| c.len() != height) {
            return Err(());
        }
        Ok(Array2D {
            width,
            height,
            cols,
        })
    }
}

impl<T> Index<Int2> for Array2D<T> {
    type Output = T;

    fn index(&self, index: Int2) -> &Self::Output {
        &self.cols[index.x][index.y]
    }
}

impl<T> IndexMut<Int2> for Array2D<T> {
    fn index_mut(&mut self, index: Int2) -> &mut Self::Output {
        &mut self.cols[index.x][index.y]
    }
}

fn transpose_vecs<T: Clone>(vecs: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let inner_len = vecs.len();
    let outer_len = vecs[0].len();

    let mut outer: Vec<Vec<T>> = vec![];
    for o in 0..outer_len {
        let mut inner: Vec<T> = vec![];
        for i in 0..inner_len {
            inner.push(vecs[i][o].clone());
        }
        outer.push(inner);
    }
    outer
}

#[cfg(test)]
mod test {
    use crate::models::array2d::Array2D;

    fn number_array(width: usize, height: usize) -> Array2D<i32> {
        let mut c = 0;
        let cols: Vec<Vec<i32>> = (0..width)
            .map(|_| {
                (0..height)
                    .map(|_| {
                        c += 1;
                        c
                    })
                    .collect()
            })
            .collect();
        
        Array2D::try_from(cols).unwrap()
    }

    #[test]
    fn array2d_construction() {
        let broken_cols = vec![vec![0, 1], vec![0, 1, 2], vec![0, 1, 2]];
        assert!(Array2D::try_from(broken_cols).is_err());

        let cols = vec![vec![0, 1, 2], vec![0, 1, 2], vec![0, 1, 2]];
        let arr = Array2D::try_from(cols).unwrap();
        let transposed_cols = arr.transpose().cols;
        assert_eq!(
            transposed_cols,
            vec![vec![0, 0, 0], vec![1, 1, 1], vec![2, 2, 2]]
        )
    }

    #[test]
    fn array2d_rows_cols_diags() {
        let arr = number_array(7, 4);
        // A B C D E _ _
        // X A B C D E _
        // _ X A B C D E
        // _ _ X A B C D
        assert_eq!(arr.diags(3).len(), 6);
        assert_eq!(arr.diags(4).len(), 4);

        assert_eq!(arr.rows().len(), 4);
        assert_eq!(arr.cols().len(), 7);
    }
}
