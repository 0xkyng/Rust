fn main() {
    let marks = 95;
    let mut grade = 'N';

    if marks >= 90 {
        grade = 'A';
    } else if marks >= 80 {
        grade = 'B';
    } else if marks >= 70 {
        grade = 'C';
    } else if marks >= 60 {
        grade = 'E';
    } else {
        grade = 'F';
    }

    // RETURNING VALUES
    let marks = 95;

    let mut grade = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else if marks >= 60 {
        'C'
    } else {
        'F'
    };

    // MATCHING EXPRESSIOND
    let marks = 95;
    let mut grade = 'N';

    match marks {
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        70..=79 => grade = 'C',
        _ => grade = 'F',
    }

    // RETURNING VALUES
    let marks = 95;

    let grade = match marks {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        _ => 'F'
    };
}

pub fn bigger(a: i32, b:i32) -> i32 {
    if a > b {
        a
    } else  {
        b
    }
}
