pub fn run() {
    let marks: i32 = 95;
    {
        let mut _grades: char = 'N';
        if marks >= 90 {
            _grades = 'A'
        } else if marks >= 80 {
            _grades = 'B'
        } else if marks >= 70 {
            _grades = 'C'
        } else {
            _grades = 'D'
        }
        println!("{}", _grades);
    }

    {
        let g = if marks >= 90 {
            'A'
        } else if marks >= 80 {
            'B'
        } else if marks >= 70 {
            'C'
        } else {
            'D'
        };

        println!("{}", g);
    }

    {
        let m = 12;
        let mut grade2: char = 'N';
        match m {
            90..=100 => grade2 = 'A',
            80..=89 => grade2 = 'B',
            70..=79 => grade2 = 'C',
            _ => grade2 = 'D',
        }
        println!("{}", grade2);
    }
}
