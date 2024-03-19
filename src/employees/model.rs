use crate::db;
use crate::error_handlers::CustomError;
use crate::schema::employees;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = crate::schema::employees)]
pub struct Employee {
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary: i32,
    pub age: i32,
}

#[derive(Serialize, Deserialize, Selectable, Queryable, Insertable)]
#[diesel(table_name = crate::schema::employees)]
pub struct Employees {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary: i32,
    pub age: i32,
}

impl Employees {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = &mut db::connection()?;
        let res = employees::table.select(Employees::as_select()).load(conn)?;
        Ok(res)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let employee = employees::table.filter(employees::id.eq(id)).first(conn)?;
        Ok(employee)
    }

    pub fn create(data: Employee) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let res = diesel::insert_into(employees::table)
            .values(data)
            .get_result(conn)?;
        Ok(res)
    }

    pub fn update(id: i32, value: Employee) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let res = diesel::update(employees::table)
            .filter(employees::id.eq(id))
            .set(value)
            .get_result(conn)?;
        Ok(res)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = &mut db::connection()?;
        let res = diesel::delete(employees::table)
            .filter(employees::id.eq(id))
            .execute(conn)?;
        Ok(res)
    }
}
