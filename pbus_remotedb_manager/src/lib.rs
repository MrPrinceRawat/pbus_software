use std::error::Error;
use tokio_postgres::NoTls;
use tokio_postgres::{tls::NoTlsStream, Socket};
use utility::Target;

#[derive(Debug)]
pub struct TableField {
    pub name: String,
    pub data_type: String,
}

pub struct DbHandler {
    pub client: tokio_postgres::Client,
}

impl DbHandler {
    pub async fn new(
        host: &String,
        user: &String,
        dbname: &String,
        password: &String,
    ) -> Result<DbHandler, Box<dyn Error>> {
        // Configure the connection parameters
        let (client, connection) = tokio_postgres::connect(
            format!(
                "host={} user={} dbname={} password={}",
                host, user, dbname, password
            )
            .as_str(),
            NoTls,
        )
        .await?;

        // Spawn a new tokio runtime for the connection
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(DbHandler { client })
    }

    pub async fn get_tables(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut tables = Vec::new();
        let rows = self.client.query("SELECT table_name FROM information_schema.tables WHERE table_schema='public' AND table_type='BASE TABLE';", &[]).await?;
        for row in rows {
            tables.push(row.get(0));
        }
        Ok(tables)
    }
    pub async fn get_targets(&self) -> Result<Vec<Target>, Box<dyn Error>> {
        let mut tables: Vec<Target> = Vec::new();
        let rows = self.client.query("SELECT table_name FROM information_schema.tables WHERE table_schema='public' AND table_type='BASE TABLE';", &[]).await?;
        for row in rows {
            tables.push(Target::new(row.get(0)));
        }
        Ok(tables)
    }

    pub async fn get_table_fields(&self, table: String) -> Result<Vec<TableField>, Box<dyn Error>> {
        let mut fields = Vec::new();
        let rows = self.client.query("SELECT * FROM information_schema.columns WHERE table_schema='public' AND table_name=$1;", &[&table]).await?;
        for row in rows {
            let name: String = row.get(3);
            let data_type: String = row.get(7);

            let field = TableField { name, data_type };
            fields.push(field);
        }
        Ok(fields)
    }

    pub async fn get_table_data(&self, table: Target) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
        let mut data = Vec::new();
        let rows = self
            .client
            .query(format!("SELECT * FROM {};", table.get_name()).as_str(), &[])
            .await?;
        for row in rows {
            let mut row_data = Vec::new();
            for i in 0..row.len() {
                row_data.push(row.get(i));
            }
            data.push(row_data);
        }
        Ok(data)
    }

    pub async fn get_rows(
        &self,
        table: &Target,
        last_id: i32,
    ) -> Result<(Vec<serde_json::Value>, i32), Box<dyn Error>> {
        let rows = self
            .client
            .query(
                format!(
                    "SELECT row_to_json({}) FROM {} WHERE id > {};",
                    table.get_name(),
                    table.get_name(),
                    last_id
                )
                .as_str(),
                &[],
            )
            .await
            .unwrap();

        let row: Vec<serde_json::Value> = rows
            .iter()
            .map(|row| row.get(0))
            .collect::<Vec<serde_json::Value>>();

        if row.len() == 0 {
            return Ok((row, last_id));
        }

        let js_row: serde_json::Value = rows[rows.len() - 1].get(0);

        let last_id: i32 = js_row["id"].as_i64().unwrap() as i32;

        Ok((row, last_id))
    }
}
