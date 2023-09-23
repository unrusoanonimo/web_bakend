use sqlite::{Connection, Statement};

use crate::data::user::{LoginData, User, UserData};
struct UserStatements<'a> {
    login: Statement<'a>,
    insert: Statement<'a>,
    get_by_token: Statement<'a>,
    count: Statement<'a>,
}

impl<'a> UserStatements<'a> {
    fn new(conection: &'a Connection) -> Result<UserStatements<'_>, sqlite::Error> {
        let login = conection
            .prepare("SELECT count(*) as valid FROM USER_MACS WHERE id == :id AND mac == :mac")?;
        let insert = conection.prepare(
            "INSERT INTO `USER` (`id`, `token`, `permissions`) VALUES (:id, :token, :perms)",
        )?;
        let get_by_token = conection
            .prepare("SELECT `id`,`token`,`permissions` FROM `USER` WHERE token= :token")?;
        let count = conection.prepare("SELECT COUNT(*) AS COUNT FROM USER")?;
        Ok(Self {
            login,
            insert,
            get_by_token,
            count,
        })
    }
}
pub struct UserDao<'a> {
    conection: &'a Connection,
    sttms: UserStatements<'a>,
}
impl<'a> UserDao<'a> {
    pub fn new(conection: &'a Connection) -> Result<Self, sqlite::Error> {
        Ok(Self {
            sttms: UserStatements::new(conection)?,
            conection,
        })
    }
    pub fn get_by_token(&mut self, token: &str) -> Result<UserData, sqlite::Error> {
        let sttm = &mut self.sttms.get_by_token;
        sttm.reset()?;

        sttm.bind((":token", token))?;
        sttm.next()?;

        let id: i64 = sttm.read(0)?;
        let token: String = sttm.read(1)?;
        let permissions: i64 = sttm.read(2)?;

        Ok(UserData::new(id as u64, token, permissions as u32))
    }
    pub fn validate_credentials(&mut self, cred: &LoginData) -> Result<bool, sqlite::Error> {
        let id = *cred.id();
        let mac = cred.mac();
        let sttm = &mut self.sttms.login;
        sttm.reset()?;
        sttm.bind((":id", id as i64))?;
        sttm.bind((":mac", mac))?;

        sttm.next()?;

        let valid = sttm.read::<i64, _>("valid")?;
        Ok(valid > 0)
    }
    pub fn insert<T: User>(&mut self, user: T) -> Result<(), sqlite::Error> {
        let id = *user.id();
        let perms = *user.permissions();
        let token = user.token();

        let sttm = &mut self.sttms.insert;
        sttm.reset()?;

        sttm.bind((":token", token))?;
        sttm.bind((":id", id as i64))?;
        sttm.bind((":perms", perms as i64))?;
        sttm.next()?;

        Ok(())
    }
    pub fn count(&mut self) -> Result<u64, sqlite::Error> {
        let sttm = &mut self.sttms.count;
        sttm.reset()?;

        sttm.next()?;
        let count: i64 = sttm.read("count")?;
        Ok(count as u64)
    }
}
