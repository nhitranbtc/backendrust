pub mod elastic_store;
pub mod model;

#[cfg(test)]
mod tests {
    use crate::elastic_actor::elastic_store::{
        commands::{Document, EnsureIndexExists, MappingDoc},
        queries::{SearchDocument},
        Client,
    };
    use std::error::Error;
    #[test]
    //#[allow(dead_code)]
    fn test_ensure_index_exists() -> Result<(), Box<dyn Error>> {
        let client = Client::new("http://localhost:9200")?;
        client.ensure_index_exists()?;
        Ok(())
    }

    //#[test]
    #[allow(dead_code)]
    fn test_save_sample_doc() -> Result<(), Box<dyn Error>> {
        let client = Client::new("http://localhost:9200")?;
        client.save_sample_doc()?;
        Ok(())
    }
    //#[test]
    #[allow(dead_code)]
    fn test_index_res() -> Result<(), Box<dyn Error>> {
        let client = Client::new("http://localhost:9200")?;
        let data = json!({
            "title":"Title Index Res"
        });
        client.index(data)?;
        Ok(())
    }
    //#[test]
    #[allow(dead_code)]
    fn test_get() -> Result<(), Box<dyn Error>> {
        let id = "1";
        let client = Client::new("http://localhost:9200")?;
        let doc = client.get(&id);
        println!("doc {:?}", doc.unwrap());
        Ok(())
    }
    //#[test]
    #[allow(dead_code)]
    fn test_get_index() -> Result<(), Box<dyn Error>> {
        let client = Client::new("http://localhost:9200")?;
        let index = client.get_mapping();
        println!("index {:?}", index.unwrap());
        Ok(())
    }

    #[test]
    //#[allow(dead_code)]
    fn test_search() -> Result<(), Box<dyn Error>> {
        let client = Client::new("http://localhost:9200")?;
        let dbs = client.search("Title");
        for db in dbs.unwrap().documents() {
            println!("search {:?}", db);
        }
        Ok(())
    }
}
