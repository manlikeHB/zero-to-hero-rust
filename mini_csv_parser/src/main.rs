use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
struct Record {
    name: String,
    age: u32,
    city: String,
}

impl Record {
    fn new(name: String, age: u32, city: String) -> Self {
        Record { name, age, city }
    }
}

struct Csv {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Csv {
    fn from_file(path: &str) -> std::io::Result<Csv> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);

        let mut headers = Vec::<String>::new();
        let mut rows = Vec::<Vec<String>>::new();

        for (i, res) in reader.lines().enumerate() {
            let line = res?;
            let cols = line.split(",").map(|x| x.to_string()).collect();
            if i == 0 {
                headers = cols;
            } else {
                rows.push(cols);
            }
        }

        let csv = Csv {
            headers,
            rows,
        };
        Ok(csv)
    }

    fn get(&self, row: usize, cols: &str) -> Option<&str> {
        let idx = self.headers.iter().position(|x| x == cols)?;
        self.rows.get(row)?.get(idx).map(|s| s.as_str())
    }

    fn iter_records(&self) -> Vec<Record> {
        let mut records = Vec::<Record>::new();
        for i in 1..self.rows.len() {
            let data: Vec<String> = self.rows[i].iter().map(|a| a.trim().to_string()).collect();
            if data.len() < 3 {
                continue;
            }
            if let Ok(n) = data[1].parse::<u32>() {
                records.push(Record::new(data[0].clone(), n, data[2].clone()));
            }
        }
        records
    }
}

fn main() -> std::io::Result<()> {
    let path = "text.csv";
    let csv = Csv::from_file(path)?;

    let records = csv.iter_records();

    println!("Records: {:?}", records);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_file() {
        let csv = match Csv::from_file("text.csv") {
            Ok(csv) => csv,
            Err(e) => panic!("Fail to load Csv: {}", e),
        };

        assert!(!csv.rows.is_empty(), "Csv should not be empty");
    }

    #[test]
    fn test_get() {
        let csv = match Csv::from_file("text.csv") {
            Ok(csv) => csv,
            Err(e) => panic!("Fail to load Csv: {}", e),
        };

        assert!(
            csv.get(0, "name").unwrap() == "Alice",
            "wrong name on row 1"
        );
        assert!(
            csv.get(0, "city").unwrap() == "London",
            "wrong city on row 1"
        );

        assert!(
            csv.get(0, "local").is_none(),
            "wrong city on row 1"
        );

        assert!(
            csv.get(6, "local").is_none(),
            "wrong city on row 1"
        );
    }
}
