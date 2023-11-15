use std::{fs, io, path::Path};

pub fn read_dir_item<F>(path: &Path, filter: F) -> io::Result<Vec<String>>
where
    F: Fn(&str) -> bool,
{
    let mut results = vec![];
    if let files = fs::read_dir(path).unwrap() {
        for file in files {
            let item = file.unwrap().file_name();
            if filter(item.to_str().unwrap()) {
                let file_path = format!("{}/{}", path.to_str().unwrap(), item.to_str().unwrap());
                results.push(file_path);
            }
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "文件无法打开"));
    }

    Ok(results)
}


#[cfg(test)]
mod test{
    use std::path::Path;
    use super::read_dir_item;

    #[test]
    pub fn test_read_dir_item(){
        let path = Path::new("D:\\rust");
        let result = read_dir_item(path, |str|{true}).unwrap();
        result.iter().for_each(|s|{println!("{}",s)});
    }

}