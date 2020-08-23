use crate::connection::*;

pub struct Broker {
    connection: Option<Connection>,
    node_id: i32,
}

impl Broker {
    pub fn new(node_id: i32) -> Broker {
        Broker {
            connection: None,
            // TODO: secondary connection for CG etc.
            node_id
        }
    }

    pub async fn open(&mut self, address: &str) -> std::io::Result<()> {
        let mut connection = Connection::new();
        connection.open(address).await?;
        self.connection = Some(connection);
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        match self.connection {
            Some(_) => true,
            None => false
        }
    }

    pub async fn metadata(&mut self) {
        
    }
}
