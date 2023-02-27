use crate::response::{StudentData, StudentListResponse, StudentResponse,SingleStudentResponse};
use crate::{
    error::Error::*,model::Student, schema::CreateStudentSchema, schema::UpdateStudentSchema, Result,
};
use futures::StreamExt;
use mongodb::bson::{doc, oid::ObjectId,Document};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions,ReturnDocument};
use mongodb::{bson,options::ClientOptions,Client,Collection, IndexModel};
use warp::query;
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
            roll: student.roll.to_owned(),
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
    pub async fn create_student(&self,body: &CreateStudentSchema)-> Result<Option<SingleStudentResponse>>{
        let name = body.name.to_owned().unwrap_or("".to_string());
        let email = body.email.to_owned().unwrap_or("".to_string());
        let course = body.course.to_owned().unwrap_or("".to_string());
        let university = body.university.to_owned().unwrap_or("".to_string());
        let serialized_data = bson::to_bson(&body).map_err(MongoSerializBsonError)?;
        let document = serialized_data.as_document().unwrap();
        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder().keys(doc! {"roll":1}).options(options).build();
        self.Student_collection.create_index(index, None).await.expect("error creating index!");
        let mut inserted_doc = doc!{"name": name, "email": email, "course": course, "university":university};
        inserted_doc.extend(document.clone());

        let insert_result = self.collection.insert_one(&inserted_doc,None).await.map_err(|e|{
            if e.to_string().contains("E11000 duplicate key error collection"){
                return MongoDuplicateError(e);
            }
            return MongoQueryError(e);
        })?;
        let new_id = insert_result.inserted_id.as_object_id().expect("issue with new -id");

        let student_doc = self.Student_collection.find_one(doc! {"_id":new_id}, None).await.map_err(MongoQueryError)?;
        if student_doc.is_none(){
            return Ok(None);
        }
        let student_response = SingleStudentResponse{
            status: "success".to_string(),
            data: StudentData { student: self.doc_to_student(&student_doc.unwrap()).unwrap(), },
        };
        Ok(Some(student_response))
    }
    pub async fn get_student(&self,id:&str)-> Result<Option<SingleStudentResponse>>{
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        let student_doc = self.Student_collection.find_one(doc!{"_id":oid}, None).await.map_err(MongoQueryError)?;

        if student_doc.is_none(){
            return Ok(None);
        }

        let student_response = SingleStudentResponse{
            status: "success".to_string(),
            data: StudentData { student: self.doc_to_student(&student_doc.unwrap()).unwrap(), },
        };
        Ok(Some(student_response))
    }
    pub async fn edit_student(&self,id: &str,body:&UpdateStudentSchema,)->Result<Option<SingleStudentResponse>>{
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        let find_one_and_update_options = FindOneAndUpdateOptions::builder().return_document(ReturnDocument::After).build();

        let serialized_data = bson::to_bson(body).map_err(MongoSerializBsonError)?;
        let document = serialized_data.as_document().unwrap();
        let update = doc! {"$set": document};

        let student_doc = self.Student_collection.find_one_and_update(query, update, find_one_and_update_options).await.map_err(MongoQueryError)?;

        if student_doc.is_none(){
            return Ok(None);
        }

        let student_response = SingleStudentResponse{
            status:"success".to_string(),
            data: StudentData { student: self.doc_to_student(&student_doc.unwrap()).unwrap(),},
        };
        Ok(Some(student_response))
    }
    pub async fn delete_student(&self, id: &str) -> Result<Option<()>> {
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
     
        let result = self
            .collection
            .delete_one(doc! {"_id":oid }, None)
            .await
            .map_err(MongoQueryError)?;
     
        if result.deleted_count == 0 {
            return Ok(None);
        }
     
        Ok(Some(()))
    }
}