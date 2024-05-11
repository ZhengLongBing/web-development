use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum DbErr {
    NoFindDbURL,
    DbConnectErr(sea_orm::DbErr),
}

impl Display for DbErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DbErr::NoFindDbURL => write!(f, "没有找到环境变量 DATABASE_URL ！"),
            DbErr::DbConnectErr(err) => write!(f, "数据库连接失败！\n错误信息：{err}"),
        }
    }
}
