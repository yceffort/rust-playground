use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// 세포 struct
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

// 우주 struct
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
       // public method. 이를 자바스크립트에서 쓸 수 있게 할 것이다.
       pub fn tick(&mut self) {
        // 현재 세포들을 모두 꺼내와서 복사해둔다.
          let mut next = self.cells.clone();
  
        // 현재 모든 셀을 순환한다.
          for row in 0..self.height {
              for col in 0..self.width {
                  let idx = self.get_index(row, col);
                  // 현재 세포
                  let cell = self.cells[idx];
                  // 주변 세포가 몇개나 살아 있는지 계산한다
                  let live_neighbors = self.live_neighbor_count(row, col);
  
                  // 다음 셀은 다음과 같이 결정된다.
                  let next_cell = match (cell, live_neighbors) {
    
                      // 규칙1. 살아있는 세포 근처에 두명 미만의 세포가 살아있다면, 죽는다.
                      (Cell::Alive, x) if x < 2 => Cell::Dead,
                      // 규칙 2: 살아있는 세포 규칙에 2~3의 살아있는 세포가 있다면, 산다.
                      (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                      // 규칙3: 살아있는 이웃세포가 3 보다 많다면 죽는다
                      (Cell::Alive, x) if x > 3 => Cell::Dead,
                      // 규칙4: 살아있는 이웃이 정확히 3개있는 죽은세포는 살아난다.
                      (Cell::Dead, 3) => Cell::Alive,
                      // 그외의 다른 셀은 그대로...
                      (otherwise, _) => otherwise,
                  };
                  next[idx] = next_cell;
              }
          }
          self.cells = next;
      }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // 이웃에 살아있는 세포가 몇개인지 센다.
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        // count
        let mut count = 0;
        // raw, height 순회
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    // 생성자역할
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}