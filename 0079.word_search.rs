impl Solution {
  fn search(
    i: usize,
    j: usize,
    c: usize,
    word: &[u8],
    board: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
  ) -> bool {
    if (board[i][j] as u8) != word[c] {
      return false;
    }
    if c == word.len() - 1 {
      return true;
    }
    seen[i][j] = true;
    let dirs = [
      [-1, 0],
      [0, -1],
      [1, 0],
      [0, 1],
    ];
    for [x, y] in dirs.into_iter() {
      // 索引合法性审核
      let i = x + (i as i32);
      let j = y + (j as i32);
      if i < 0 || i == (board.len() as i32) || j < 0 || j == (board[0].len() as i32) {
        continue;
      }
      let (i, j) = (i as usize, j as usize);
      if !seen[i][j] && Self::search(i, j, c + 1, word, board, seen) {
        return true;
      }
    }
    seen[i][j] = false;
    false
  }

  pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let m = board.len();
    let n = board[0].len();
    let word = word.as_bytes();
    let mut seen = vec![vec![false; n]; m];
    for i in 0..m {
      for j in 0..n {
        if Self::search(i, j, 0, word, &board, &mut seen) {
          return true;
        }
      }
    }
    false
  }
}
