use core::f32;
use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::vec::Vec;

use crate::entity::task_body::{TaskBody, TaskResponse};

pub fn read_csv_with_header(
    file: &Path,
    page: usize,
    limit: usize,
) -> Result<HashMap<String, Vec<String>>, Error> {
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

    lines
        .iter()
        .skip((page - 1) * limit)
        .take(limit)
        .for_each(|&line| {
            let values = line.split(",").collect::<Vec<&str>>();

            for i in 0..values.len() {
                if let Some(list) = record.get_mut(headers[i]) {
                    list.push(values[i].to_string());
                }
            }
        });

    Ok(record)
}

pub fn read_heatmap(path: &Path, which: usize) -> Result<Vec<Vec<f32>>, Error> {
    let file = fs::read_to_string(path)?;
    let lines = file.lines().collect::<Vec<&str>>();
    if lines.is_empty() {
        return Err(Error::new(ErrorKind::NotFound, "无内容"));
    }
    let result = lines
        .iter()
        .skip(4 * which + 1)
        .take(4)
        .map(|&line| {
            line.split(",")
                .map(|s| s.parse::<f32>().unwrap_or(0f32))
                .collect()
        })
        .collect::<Vec<Vec<f32>>>();
    Ok(result)
}

pub fn read_all_heatmap(task_body: &TaskBody, which: usize) -> Result<TaskResponse, Error> {
    let mut all_maps = HashMap::new();
    for ele in fs::read_dir("D:\\rust")? {
        let entry = ele?;
        let file_name = entry.file_name().into_string().unwrap();
        if file_name.starts_with(&task_body.task_id) && file_name.ends_with("saliency.csv") {
            if let Ok(heatmap) = read_heatmap(entry.path().as_path(), which) {
                all_maps.insert(file_name, heatmap);
            }
        }
    }

    Ok(TaskResponse {
        task_body: task_body.to_owned(),
        csv_content: None,
        heat_map: Some(all_maps),
    })
}

pub fn read_by_page(
    task_body: &TaskBody,
    page: usize,
    limit: usize,
) -> Result<TaskResponse, Error> {
    let mut output = HashMap::new();
    for ele in fs::read_dir("D:/rust/")? {
        let entry = ele?;
        let path = entry.path();
        if path.ends_with("predict.csv") && path.starts_with(&task_body.task_id) {
            output.insert(
                entry.file_name().into_string().unwrap(),
                read_csv_with_header(path.as_path(), page, limit)?,
            );
        }
    }

    Ok(TaskResponse {
        task_body: task_body.to_owned(),
        csv_content: Some(output),
        heat_map: None,
    })
}

#[cfg(test)]
mod test {
    use super::{read_csv_with_header};
    use std::path::Path;

    #[test]
    pub fn test_read_with_header() {
        let path = Path::new("D:\\rust\\test.csv");
        let result = read_csv_with_header(path, 1, 4).unwrap();
        for ele in result {
            let (key, val) = ele;
            print!("{key}:");
            val.iter().for_each(|v| print!("{}", v));
            println!("");
        }
    }

    #[test]
    pub fn test_read_as_heatmap() {
        let path = Path::new("D:\\rust\\test.csv");
    }
}
