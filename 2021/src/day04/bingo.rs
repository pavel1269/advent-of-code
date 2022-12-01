const SHEET_SIZE: usize = 5;

#[derive(Clone)]
pub struct Bingo {
    highlighted: Vec<Vec<bool>>,
    numbers: Vec<Vec<i32>>,
    won: bool,
}

impl Bingo {
    pub fn mark_number(&mut self, marked_number: i32) -> bool {
        for (row_index, row) in self.numbers.iter().enumerate() {
            for (column_index, number) in row.into_iter().enumerate() {
                if *number == marked_number {
                    self.highlighted[row_index][column_index] = true;
                }
            }
        }

        let won = self.check_victory();
        self.won = won;
        return won;
    }

    pub fn is_won(&self) -> bool {
        return self.won;
    }

    fn check_victory(&self) -> bool {
        let mut column_counter = vec![0; SHEET_SIZE];

        for row in self.highlighted.iter() {
            let mut row_counter = 0;
            for (column_index, highlighted) in row.into_iter().enumerate() {
                if *highlighted {
                    row_counter += 1;
                    column_counter[column_index] += 1;
                }
            }

            if row_counter == SHEET_SIZE {
                return true;
            }
        }

        return column_counter.iter().copied().filter(|counter| *counter == SHEET_SIZE).count() > 0;
    }

    pub fn get_sum_unmarked_numbers(&self) -> i64 {
        let mut sum = 0;
        for (row_index, row) in self.highlighted.iter().enumerate() {
            for (column_index, highlighted) in row.into_iter().enumerate() {
                if !highlighted {
                    sum += self.numbers[row_index][column_index] as i64;
                }
            }
        }

        return sum;
    }

    pub fn from(numbers: &Vec<Vec<i32>>) -> Bingo {
        assert_eq!(numbers.len(), SHEET_SIZE);

        let mut bingo = Bingo {
            highlighted: vec![vec![false; SHEET_SIZE]; SHEET_SIZE],
            numbers: vec![vec![0; SHEET_SIZE]; SHEET_SIZE],
            won: false,
        };

        for (row_index, row) in numbers.into_iter().enumerate() {
            assert_eq!(row.len(), SHEET_SIZE);
            for (column_index, number) in row.into_iter().enumerate() {
                bingo.numbers[row_index][column_index] = *number;
            }
        }

        return bingo;
    }
}
