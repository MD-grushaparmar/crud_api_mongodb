//use crate::error;
use crate::{error::Error::*, handler::StudentRequest, Student, Result};
//use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{doc, document::Document, oid::ObjectId};
use mongodb::{options::ClientOptions, Client, Collection};

const ID: &str = "_id";
const NAME: &str = "name";
const EMAIL: &str = "email";
const COURSE: &str = "course";
const UNIVERSITY: &str = "university";
pub struct  DB{
    pub client: Client,
}
impl DB{
    pub async fn init()-> Result<Self> {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        //client_options.app_name = Some("booky".to_string());

        Ok(Self { client: Client::with_options(client_options)?, })
    }

    pub async fn fetch_student(&self) -> Result<Vec<Student>> {
        let mut cursor = self
            .get_collection()
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut result: Vec<Student> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_student(&doc?)?);
        }
        Ok(result)
    }

    pub async fn create_student(&self, entry: &StudentRequest) -> Result<()> {
        let doc = doc! {
            NAME: entry.name.clone(),
            EMAIL: entry.email.clone(),
            COURSE: entry.course.clone(),
            UNIVERSITY: entry.university.clone(),
        };

        self.get_collection()
            .insert_one(doc, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn edit_student(&self,id: &str, entry: &StudentRequest) -> Result<()> {
        let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        let doc = doc! {
            NAME : entry.name.clone(),
            EMAIL: entry.email.clone(),
            COURSE: entry.course.clone(),
            UNIVERSITY: entry.university.clone(),
        };

        self.get_collection()
            .update_one(query, doc, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn delete_student( &self,id: &str) -> Result<()> {
        let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let filter = doc! {
            "_id": oid,
        };

        self.get_collection()
            .delete_one(filter, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }


    fn doc_to_student( &self,doc: &Document) -> Result<Student> {
        let id = doc.get_object_id(ID)?;
        let name = doc.get_str(NAME)?;
        let email = doc.get_str(EMAIL)?;
        let course = doc.get_str(COURSE)?;
        let university = doc.get_str(UNIVERSITY)?;

        let student = Student {
            id: id.to_hex(),
            name: name.to_owned(),
            email: email.to_owned(),
            course: course.to_owned(),
            university: university.to_owned(),
        };
        Ok(student)
    }
    pub async fn mongo_connect()->Client{
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
        //client_options.app_name = Some("Rust Demo".to_string());
        let client = Client::with_options(client_options).unwrap();
        client
    }
    
    pub fn get_collection(&self)->Collection{
        let collection_list = self.client.database("mydatabase").collection::<Student>("Student");
        collection_list
    }
}