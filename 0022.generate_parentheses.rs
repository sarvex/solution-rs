impl Solution {
  fn generate(left: i32, right: i32, s: &mut String, result: &mut Vec<String>) {
    if left == 0 && right == 0 {
      result.push(s.clone());
      return;
    }
    if left > 0 {
      s.push('(');
      Self::generate(left - 1, right, s, result);
      s.pop();
    }
    if right > left {
      s.push(')');
      Self::generate(left, right - 1, s, result);
      s.pop();
    }
  }

  pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result = Vec::new();
    Self::generate(n, n, &mut String::new(), &mut result);
    result
  }
}
