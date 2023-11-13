use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::vec::Vec;

pub fn readCSVWithHeader(file: &Path) -> Result<HashMap<String, Vec<String>>, Error> {
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

pub fn readHeatMap(path: &Path, which: usize) -> Result<Vec<Vec<String>>, Error> {
    let mut heatMap = vec![vec![]];
    let content = fs::read_to_string(path)?;
    if content.is_empty() {
        return Err(Error::new(ErrorKind::Other, "文件无内容"));
    }
    let lines = content.lines().collect::<Vec<&str>>();
    lines
        .iter()
        .skip(4 * which + 1)
        .take(4)
        .for_each(|&s| heatMap.push(s.split(",").map(|s| s.to_string()).collect::<Vec<String>>()));

    Ok(heatMap)
}
