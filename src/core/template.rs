use std::collections::HashMap;
use tera::{Tera, Error, Context};


pub fn get_template_as_string(variables:&HashMap<&str,String>)->Result<String,Error>{
    if let Ok(tera) = Tera::new("templates/*"){
        let mut context  = Context::new();
        // context.insert("val", "lfasdofjaosdfjoias");
        variables.iter().for_each(|(&key,val)|{context.insert(key, val)});
        return Ok(tera.render("a.html", &context).unwrap());
    }else{
        return Err(Error::msg("获取模版引擎失败"));
    }
}

#[cfg(test)]
mod test{

    #[test]
    pub fn test_template_render(){
        // let task_body = TaskBody::new();
        // let s = get_template_as_string(task_body).unwrap();
        // println!("{}",s);
    }
}