use super::*;

pub fn replace_column(data_1: String, data_2: String, column: &str, mark: &str) -> Result<String, Error> {
    let mut lines_1 = data_1.lines();
    let mut lines_2 = data_2.lines();
    let headers_1 = lines_1.next().unwrap();
    let headers_2 = lines_2.next().unwrap();
    println!("{}", headers_1);
    println!("{}", headers_2);
    let columns_1: Vec<&str> = headers_1.split(',').collect();
    let columns_2: Vec<&str> = headers_2.split(',').collect();
    println!("{:?}", columns_1);
    println!("{:?}", columns_2);
    let column_number_1 = columns_1.iter().position(|&e| e == column);
    let column_number_1 = match column_number_1 {
        Some(column) => column,
        None => Err("column name doesn't exist in the input file")?
    };
    let column_number_2 = columns_2.iter().position(|&e| e == column);
    let column_number_2 = match column_number_2 {
        Some(column) => column,
        None => Err("column name doesn't exist in the input file")?
    };
    let mark_number_1 = columns_1.iter().position(|&e| e == mark);
    let mark_number_1 = match mark_number_1 {
        Some(mark) => mark,
        None => Err("mark name doesn't exist in the input file")?
    };
    let mark_number_2 = columns_2.iter().position(|&e| e == mark);
    let mark_number_2 = match mark_number_2 {
        Some(mark) => mark,
        None => Err("mark name doesn't exist in the input file")?
    };
    let mut result = String::with_capacity(data_1.capacity());
    result.push_str(&columns_1.join(","));
    result.push('\n');
    'outer: for line_1 in lines_1 {
        let mut records_1: Vec<&str> = line_1.split(',').collect();
        let mut lines_2 = data_2.lines();
        lines_2.next();
        'inner: for line_2 in lines_2 {
            let records_2: Vec<&str> = line_2.split(',').collect();
            if records_2[mark_number_2] == records_1[mark_number_1] {
                records_1[column_number_1] = records_2[column_number_2];
                println!("{}", records_1[column_number_1]);
                break 'inner;
            }
        }
        result.push_str(&records_1.join(","));
        result.push('\n');
    }
    Ok(result)
}