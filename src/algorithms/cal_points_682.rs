// https://leetcode.com/problems/baseball-game/

pub fn cal_points(ops: Vec<String>) -> i32 {
    let mut points = vec![];
    for e in ops {
        match e.as_str() {
            "C" => {
                points.pop();
            },
            "D" => {
                if let Some(l) = points.last() {
                    points.push(l * 2);
                }
            },
            "+" => {
                let len = points.len();
                points.push(points[len - 1] + points[len -2]);
            }
            _ => {
                if let Ok(i) = &e.parse::<i32>() {
                    points.push(*i);
                }
            }
        }
    }
    points.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(30, cal_points(vec!["5".to_owned(),"2".to_owned(),"C".to_owned(),"D".to_owned(),"+".to_owned()]))
    }

}