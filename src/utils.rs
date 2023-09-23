// #![allow(dead_code)]
use std::{collections::HashMap, error::Error, fmt::Display, fs, marker::PhantomData};

use rouille::Response;
use sqlite::Connection;

use crate::model::user::UserDao;

pub struct DataAccess<'a> {
    pub user_dao: UserDao<'a>,
    _pd: PhantomData<u8>,
}
impl<'a> DataAccess<'a> {
    /// #Safety
    /// If the [`Connection`] it points is moved produces unexpecte behaviour
    unsafe fn new(conection: *const Connection) -> Self {
        let ref_unch = conection.as_ref().unwrap();
        Self {
            user_dao: UserDao::new(ref_unch).unwrap(),
            _pd: PhantomData,
        }
    }
}
struct OwnedMathodsData<'a> {
    mime_hash: HashMap<&'a str, String>,
}
impl<'a> OwnedMathodsData<'a> {
    fn new() -> Self {
        let mime_hash = include!("long_lines/mime_hashmap.txt");
        Self { mime_hash }
    }
}
pub struct Modules<'a> {
    method_data: OwnedMathodsData<'a>,
    conection: Connection,
    data_access: Option<DataAccess<'a>>,
    ptr: *const Connection,
}
impl<'a> Modules<'a> {
    pub fn new(conection: Connection) -> Self {
        let r = Self {
            method_data: OwnedMathodsData::new(),
            data_access: None,
            conection,
            ptr: std::ptr::null(),
        };

        r
    }
    pub fn fill_none(&mut self) {
        let cur_ptr = &self.conection as *const _;
        if self.ptr != cur_ptr {
            ////////////////////////////  Safety   ////////////////////////////////////
            // self.ptr keeps track of the raw pointer used and reset's it id needed //
            ///////////////////////////////////////////////////////////////////////////
            let data_access = unsafe { DataAccess::new(&self.conection) };
            self.data_access = Some(data_access);
            self.ptr = cur_ptr;
        }
    }
    pub fn data_access(&mut self) -> &mut DataAccess<'a> {
        self.fill_none();
        self.data_access.as_mut().unwrap()
    }
    const DEFAULT_MIME: &str = "application/octet-stream";
    pub fn mime_form_extension(&self, extension: &str) -> &str {
        if let Some(a) = self.method_data.mime_hash.get(extension) {
            a
        } else {
            Self::DEFAULT_MIME
        }
    }
}
/// #Safety
/// This is fine
unsafe impl<'a> Sync for Modules<'a> {}
/// #Safety
/// This is fine
unsafe impl<'a> Send for Modules<'a> {}

pub fn create_db(connection: &mut Connection) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("db/ura_loader.sql")?;
    let split = file.split(";");
    for table in split {
        connection.execute(table)?;
    }
    Ok(())
}
pub fn base_ur(raw_url: &str) -> &str {
    if let Some(end) = raw_url.find('?') {
        &raw_url[..end]
    } else {
        let len = raw_url.len();
        if raw_url.ends_with('/') {
            &raw_url[..len - 1]
        } else {
            raw_url
        }
    }
}
pub fn get_extension(file_names: &str) -> Option<&str> {
    let start = file_names.rfind('.')?;
    Some(&file_names[start..])
}

#[derive(Debug)]
pub enum AppError {
    UnexpectedNone,
}
impl Error for AppError {}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppError")
    }
}

pub fn empty_response_from_esstatus_code(status_code: u16) -> Response {
    Response {
        status_code,
        headers: vec![],
        data: rouille::ResponseBody::empty(),
        upgrade: None,
    }
}
