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
            let cols = line.split(',').map(|x| x.trim().to_string()).collect();
            if i == 0 {
                headers = cols;
            } else {
                rows.push(cols);
            }
        }

        let csv = Csv { headers, rows };
        Ok(csv)
    }

    fn get(&self, row: usize, cols: &str) -> Option<&str> {
        let idx = self.headers.iter().position(|x| x == cols)?;
        self.rows.get(row)?.get(idx).map(|s| s.as_str())
    }

    fn get_records(&self) -> Vec<Record> {
        let mut records = Vec::new();
        for row in &self.rows {
            if row.len() < 3 {
                continue;
            }
            if let Ok(age) = row[1].parse::<u32>() {
                records.push(Record::new(row[0].clone(), age, row[2].clone()));
            }
        }
        records
    }
}

fn main() -> std::io::Result<()> {
    let path = "text.csv";
    let csv = Csv::from_file(path)?;

    let records = csv.get_records();

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

        assert!(csv.get(0, "local").is_none(), "wrong city on row 1");

        assert!(csv.get(6, "local").is_none(), "wrong city on row 1");
    }

    #[test]
    fn test_get_records() {
        let csv = Csv::from_file("text.csv").unwrap();
        let records = csv.get_records();

        // Should only parse valid records (Alice and Bob)
        assert_eq!(records.len(), 2, "Should have exactly 2 valid records");

        // Test first record (Alice)
        assert_eq!(
            records[0].name, "Alice",
            "First record name should be Alice"
        );
        assert_eq!(records[0].age, 30, "First record age should be 30");
        assert_eq!(
            records[0].city, "London",
            "First record city should be London"
        );

        // Test second record (Bob)
        assert_eq!(records[1].name, "Bob", "Second record name should be Bob");
        assert_eq!(records[1].age, 25, "Second record age should be 25");
        assert_eq!(
            records[1].city, "Paris",
            "Second record city should be Paris"
        );

        // Verify that invalid records were skipped:
        // - "mike, 30" (missing city column)
        // - "sarah, r, lagos" (age "r" can't parse to u32)
        let names: Vec<&str> = records.iter().map(|r| r.name.as_str()).collect();
        assert!(
            !names.contains(&"mike"),
            "mike should be skipped (missing column)"
        );
        assert!(
            !names.contains(&"sarah"),
            "sarah should be skipped (invalid age)"
        );
    }
}
