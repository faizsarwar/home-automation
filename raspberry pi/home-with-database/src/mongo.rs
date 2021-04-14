use mongodb::{
    sync::{
        Client, 
        Collection
    },
    results::{
        DeleteResult
    },
    bson::{
        self,
        doc,
        Bson, 
        Bson::Document,
        oid::ObjectId
    },
    error::Error
};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Todo{
    #[serde(rename = "_id")] 
    pub id:Option<ObjectId>,            //user_name database ki wo key hai jiski value fetch ya update krni hai jitnii keys hungi sari ad hungi
    pub switch:Option<String>,
    pub state:Option<String>,
    pub switch_number:Option<String>
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct InsertableTodo{
    pub switch:Option<String>,           //user_name database ki wo key hai jiski value fetch ya update krni hai jitnii keys hungi sari ad hungi
    pub state:Option<String>,
    pub switch_number:Option<String>
}

impl InsertableTodo{
    pub fn from_todo(todo:Todo)->InsertableTodo{
        InsertableTodo{
            switch:todo.switch,   //user_name json data ki key hai aur bhi keys hungii tu wo bhi add krwaingay
            state:todo.state,
            switch_number:todo.switch_number
        }
    }
}

// collection Name
//const TODO:&str = "todo_item";

// Mongo DB Conection
fn mongo_connection()->Result<Collection, Error>{
    // let client = Client::with_uri_str("mongodb://localhost:27017")?;
    //old piaic database
    // let client = Client::with_uri_str("mongodb+srv://faiz:perfectcup@cluster0.7jdjv.azure.mongodb.net/test?authSource=admin&replicaSet=atlas-gctccg-shard-0&readPreference=primary&appname=MongoDB%20Compass&ssl=true")?;
    let client = Client::with_uri_str("mongodb://faiz:perfectcup@cluster0-shard-00-00.slckv.mongodb.net:27017,cluster0-shard-00-01.slckv.mongodb.net:27017,cluster0-shard-00-02.slckv.mongodb.net:27017/home-1?authSource=admin&replicaSet=atlas-tm3b8a-shard-0&w=majority%22&readPreference=primary&appname=MongoDB%20Compass&retryWrites=true&ssl=true")?;
    let db = client.database("home-1");
    let collection = db.collection("room-1");
    Ok(collection)
}


// GET Collection 
fn doc_coll()->Collection{
    let collection = match mongo_connection(){
        Ok(coll) => coll,
        Err(_) => panic!("Error in collection")
    };
    collection
}



// Get All Data
pub fn all()->Result<Vec<Todo>,Error>{
    let collection = doc_coll();
    let cursor = collection.find(None, None).unwrap();


    cursor.map(|result|match result{
        
    Ok(doc) => match bson::from_bson(Document(doc)) {
        Ok(result_model) => Ok(result_model),
        Err(_) => panic!("document not found"),
    },
    Err(err) => Err(err),
        })
.collect::<Result<Vec<Todo>, Error>>()
}



pub fn get_data(key:String)->Vec<String>{
    let collection = doc_coll();
    let cursor = collection.find(None, None).unwrap();
    let mut v1:Vec<String>=Vec::new();
    cursor.map(|result|match result{   
    Ok(doc) => match bson::from_bson(Document(doc.clone())) {
        Ok(result_model) =>{  
        if let Some(title) =doc.get(&key).and_then(Bson::as_str){
            v1.push(title.clone().to_string());
            };
        
        Ok(result_model)},
        Err(_) => panic!("document not found"),
    },
    Err(err) => Err(err),
        })
.collect::<Result<Vec<Todo>, Error>>();
v1
}


// Insert One Record
pub fn insert(todo:Todo)->Result<ObjectId,Error>{
    let collection = doc_coll();
    let insertable = InsertableTodo::from_todo(todo.clone());

    match bson::to_bson(&insertable){
        Ok(model_bson)=>match model_bson {
            Document(model_doc)=>{
                match collection.insert_one(model_doc, None) {
                    Ok(res)=>match res.inserted_id {
                        res=> match bson::from_bson(res) {
                            Ok(res) => Ok(res),
                            Err(_) => panic!("Error"),
                        },
                        _ => panic!("Error")
                    },
                    Err(err) => Err(err)
                }
            }
            _ => panic!("Not inserted")
        },
        Err(_) => panic!("Not Found !!! or inserted")
    }
}

pub fn delete_collection(id: ObjectId)-> Result<DeleteResult, Error>{
    let collection = doc_coll();
    collection.delete_one(doc! {"_id": id}, None)
}

pub fn update_collection(id: ObjectId, todo: Todo) -> Result<Todo, Error> {
    let collection = doc_coll();
    let mut new_todo = todo.clone();
    new_todo.id = Some(id.clone());
    match bson::to_bson(&new_todo) {
        Ok(model_bson) => match model_bson {
            Document(model_doc) => {
                match collection.replace_one(doc! {"_id": id}, model_doc, None)
                {
                    Ok(_) => Ok(new_todo),
                    Err(err) => Err(err),
                }
            }
            _ => panic!("Error insert document"),
        },
        Err(_) => panic!("Error insert document"),
    }
}
