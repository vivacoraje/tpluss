
use std::fmt;


#[derive(Debug)]
enum QuantityError {
    ParseError,
    UnknownUnit(String),
}

impl fmt::Display for QuantityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuantityError::ParseError => {
                write!(f, "解析数量字段出错")
            }
            QuantityError::UnknownUnit(e) => {
                write!(f, "发现未知计量单位'{}'，单位只能是'件'或者'袋'", &e)
            }
        }
    }
}
enum Unit {
    Bag(u32),
    Box(u32),
}

struct Quantity(Option<Unit>, Option<Unit>);

impl Quantity {
    fn new(box_: Option<u32>, bag_: Option<u32>) -> Self {
        Self(
            box_.and_then(|v| Some(Unit::Box(v))),
            bag_.and_then(|v| Some(Unit::Bag(v))),
        )
    }

    fn from_quantity(field: &str) -> Result<Self, QuantityError> {
        let q = Self::split_to_tuple(field.to_string());

        let mut quantity = Quantity(None, None);

        // for (v, u) in q {
        //     match u.as_str() {
        //         "件" => quantity.1=Some(Unit::Box(v)),
        //         _ => Err(QuantityError::UnknownUnit(u)),
        //     }
        // }

        for (v, u) in q {
            if u == "件" {
                quantity.0 = Some(Unit::Box(v));
            } else if u == "袋" {
                quantity.1 = Some(Unit::Bag(v));
            } else {
                return Err(QuantityError::UnknownUnit(u));
            }
        }

        Ok(quantity)
    }

    fn split_to_tuple(field: String) -> Vec<(u32, String)> {
        let mut v: Vec<(u32, String)> = Vec::new();

        field
            .chars()
            .fold(("".to_string(), "".to_string()), |acc, c| {
                if let Ok(v) = c.to_string().parse::<u32>() {
                    (format!("{}{}", acc.0, v), "".to_string())
                } else {
                    v.push((acc.0.parse::<u32>().unwrap(), c.to_string()));
                    ("".to_string(), "".to_string())
                }
            });

        v
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Quantity(boxx, bagg) = self;

        let b1 = if let Some(Unit::Box(b)) = boxx {
            format!("{}件", &b)
        } else {
            "".to_string()
        };

        let b2 = if let Some(Unit::Bag(b)) = bagg {
            format!("{}袋", &b)
        } else {
            "".to_string()
        };

        write!(f, "{}", b1 + b2.as_str())
    }
}

enum UnitInDb {
    // 15 件
    // 16 袋
}