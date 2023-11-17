use std::fmt::Display;
use chrono::{ NaiveDate};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug,PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "exec_env_code")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub model_name: String,
    pub pycode: String,
    pub pypath: String,
    create_time: NaiveDate,
    update_time: NaiveDate,
}


impl Display for Model{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"id:{},name:{},model_name:{},pycode:{},pypath:{},crate_time:{},last_modified_time:{}"
                ,self.id,self.name,self.model_name,self.pycode,self.pypath,self.create_time,self.update_time)
    }
}

#[derive(Clone, Copy, Debug, EnumIter,DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {
}


#[cfg(test)]
mod test{
    use super::Model;

    #[test]
    pub fn test_display(){
    }
}