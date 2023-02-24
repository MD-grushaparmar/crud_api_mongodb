use crate::response::{StudentData, StudentListResponse, StudentResponse,SingleStudentResponse};
use crate::{
    error::Error::*,model::Student, schema::CreateStudentSchema, schema::UpdateStudentSchema, Result,
};
use futures::StreamExt;
use mongodb::bson::{doc, oid::ObjectId,Document};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions,ReturnDocument};
use mongodb::{bson,options::ClientOptions,Client,Collection, IndexModel};
use std::str::FromStr;

#[derive(Clone,Debug)]
pub struct DB{
    pub Student_collection: Collection<Student>,
    pub collection: Collection<Document>,
}

impl DB{
    pub async fn init() -> Result<Self>{
        let mut client_options= ClientOptions::parse("mongodb://localhost:27017").await?;
        let client = Client::with_options(client_options).unwrap();

        let database = client.database("mydatabase");
        
        let Student_collection:Collection<Student> = database.collection("Student");
        let collection = database.collection::<Document>("Student");

        println!("✔ Database connected successfully");

        Ok(Self{
            Student_collection,
            collection,
        })
    }

    fn doc_to_student(&self,student: &Student)-> Result<StudentResponse>{
        let student_response = StudentResponse{
            id: student.id.to_hex(),
            name: student.name.to_owned(),
            email: student.email.to_owned(),
            course: student.course.to_owned(),
            university: student.university.to_owned(),
        };
        Ok(student_response)
    }
    pub async fn fetch_students(&self)-> Result<StudentListResponse>{
        let mut cursor = self.Student_collection.find(None,None).await.map_err(MongoQueryError)?;

        let mut json_result: Vec<StudentResponse> = Vec::new();
        while let Some(doc) = cursor.next().await{
            json_result.push(self.doc_to_student(&doc.unwrap())?);
        }
        let json_student_list = StudentListResponse{
            status: "success".to_string(),
            results: json_result.len(),
            students:json_result,
        };
        Ok(json_student_list)
    }
}