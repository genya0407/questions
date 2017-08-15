use super::r2d2;
use super::r2d2_postgres::{PostgresConnectionManager,TlsMode};

pub type Pool = r2d2::Pool<PostgresConnectionManager>;

pub fn get_pool() -> Pool {
    let config = r2d2::Config::default();
    let manager = PostgresConnectionManager::new(
    	"postgresql://question:question@localhost/question",
    	TlsMode::None
    ).unwrap();
    
    r2d2::Pool::new(config, manager).unwrap()
}