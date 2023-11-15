use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::vec::Vec;

pub fn read_csv_with_header(file: &Path) -> Result<HashMap<String, Vec<String>>, Error> {
    let mut record = HashMap::new();
    let content = fs::read_to_string(file)?;

    let mut lines = content.lines().collect::<Vec<&str>>();
    if lines.is_empty() {
        return Err(Error::new(std::io::ErrorKind::Other, "文件内容为空"));
    }

    let headers = lines.remove(0).split(",").collect::<Vec<&str>>();
    if headers.is_empty() {
        return Err(Error::new(std::io::ErrorKind::Other, "标题行为空"));
    }

    for i in 0..headers.len() {
        record.insert(headers[i].to_string(), vec![]);
    }

    for line in lines {
        let values = line.split(",").collect::<Vec<&str>>();

        for i in 0..values.len() {
            if let Some(list) = record.get_mut(headers[i]) {
                list.push(values[i].to_string());
            }
        }
    }

    Ok(record)
}

pub fn read_as_heatmap(path: &Path, which: usize) -> Result<Vec<Vec<String>>, Error> {
    let mut heat_map = vec![vec![]];
    let content = fs::read_to_string(path)?;
    if content.is_empty() {
        return Err(Error::new(ErrorKind::Other, "文件无内容"));
    }
    let lines = content.lines().collect::<Vec<&str>>();
    lines
        .iter()
        .skip(4 * which + 1)
        .take(4)
        .for_each(|&s| heat_map.push(s.split(",").map(|s| s.to_string()).collect::<Vec<String>>()));

    Ok(heat_map)
}

#[cfg(test)]
mod test{
    use std::{path::{self, Path}, f32::consts::E};
    use super::{read_csv_with_header, read_as_heatmap};

    #[test]
    pub fn test_read_with_header(){
        let path = Path::new("D:\\rust\\test.csv");
        let result = read_csv_with_header(path).unwrap();
        for ele in result {
            let (key,val) = ele;
            print!("{key}:");
            val.iter().for_each(|v|{print!("{}",v)});
            println!("");
        }
    }

    #[test]
    pub fn test_read_as_heatmap(){
        let path = Path::new("D:\\rust\\test.csv");
        let result = read_as_heatmap(path, 0).unwrap();
        result.iter().for_each(|v|{
            v.iter().for_each(|e|{print!("{} ",e)});
            println!("");
        })
    }

}