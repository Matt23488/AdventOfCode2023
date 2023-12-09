#[derive(Debug)]
pub struct OasisAnaylsis(Histories);

#[derive(Debug)]
struct History(Vec<i64>);

type Histories = Vec<History>;

impl OasisAnaylsis {
    pub fn create(input: &str) -> Self {
        Self(input.lines().map(History::create).collect())
    }

    pub fn extrapolated_sum(&self, reverse: bool) -> i64 {
        self.0
            .iter()
            .map(|history| history.extrapolate(reverse))
            .sum()
    }
}

impl History {
    fn create(input: &str) -> Self {
        Self(input.split(" ").filter_map(|n| n.parse().ok()).collect())
    }

    fn extrapolate(&self, reverse: bool) -> i64 {
        let mut rows = vec![self.0.clone()];
        let mut difference_row = &rows[0];

        while difference_row.iter().any(|&value| value != 0) {
            let mut new_row = vec![];
            new_row.reserve(difference_row.len());
            
            for i in 1..difference_row.len() {
                new_row.push(difference_row[i] - difference_row[i - 1]);
            }

            rows.push(new_row);
            difference_row = &rows[rows.len() - 1];
        }

        let mut a = rows.pop().expect("No difference rows found");
        a.push(0);

        while let Some(mut b) = rows.pop() {
            let x = b[if reverse { 0 } else { b.len() - 1 }];
            let y = a[a.len() - 1];

            b.push(if reverse { x - y } else { x + y });
            a = b;
        }

        a[a.len() - 1]
    }
}
