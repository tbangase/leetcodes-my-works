fn main() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}

struct Solution;

#[derive(Debug, Clone)]
enum Roman {
        I,
        V,
        X,
        L,
        C,
        D,
        M
    }
    
    impl Roman {
        pub fn value(&self) -> i32 {
            match self {
                Roman::I => 1,
                Roman::V => 5,
                Roman::X => 10,
                Roman::L => 50,
                Roman::C => 100,
                Roman::D => 500,
                Roman::M => 1000,
            }
        }
        
        pub fn from(c: char) -> Self {
            match c {
                'I' => Self::I,
                'V' => Self::V,
                'X' => Self::X,
                'L' => Self::L,
                'C' => Self::C,
                'D' => Self::D,
                'M' => Self::M,
                _ => panic!("Non registered char appeared!!: {}", c)
            }
        }
    }

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut prev_roman: Option<Roman> = None;
        let mut res = 0;
        
        for c in s.chars().rev() {
            let roman = Roman::from(c);

            let prev_value = if let Some(val) = prev_roman {
                val.value()
            } else {
                0
            };

            if prev_value > roman.value() {
                res -= roman.value();
            } else {
                res += roman.value();
            }

            prev_roman = Some(roman);
        }
        
        res
    }
}
