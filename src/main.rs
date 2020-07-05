// Necessary imports here 
use actix_web::{App, get, post, put, delete, HttpResponse, HttpServer};

// -------------------- Global Variables ----------------------
static mut AUTO_INCR_ID: i32 = 0;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {    
    // Initializing server
    HttpServer::new(|| {
        App::new()
        // Associating service(s)/route_handler(s)
         .service(find_all)
         .service(find)
         .service(create)
         .service(update)
         .service(delete)
    })
    // Binding socket address server will receive requests on
    .bind("127.0.0.1:5000")?
    .run()
    .await
}

// -------------------------- Models -----------------------------

// This struct will be used for taking input from user's request
struct Student {
    first_name: String,
    last_name: String,
    department: String,
    is_graduated: bool,
    age: i32,
}

// This struct will be used for retrieval from data sources
struct Students {
    id: i32,
    first_name: String,
    last_name: String,
    department: String,
    is_graduated: bool,
    age: i32,
}

impl Students {
    // Constructor function to get a new Student object
    fn new(id : i32, first_name : String, last_name : String, department : String,
         is_graduated : bool, age : i32) -> Students {
             Students {
                id,
                first_name,
                last_name,
                department,
                is_graduated,
                age 
            }
    }

    // Function which transform Student object to Students object
    fn from(student : Student) -> Students {
        Students::new(get_id(), student.first_name, student.last_name, student.department,
             student.is_graduated, student.age)
    }
}
// ------------------------ End Models ---------------------------

// ---------------------- Helper Functions -----------------------

// Function to increment id by 1 after each insertion 
pub fn add_to_id() {
    unsafe {
        AUTO_INCR_ID += 1;
    }
}

// Getter function to get Global ID
pub fn get_id() -> i32 {
    let mut id :i32;
    unsafe {
        add_to_id();
        id = AUTO_INCR_ID;
    }
    id
}

// -------------------- End Helper Functions ---------------------


// ----------------------- Route Handlers ------------------------

// This route handler will list all the data available
#[get("/students")]
async fn find_all() -> HttpResponse {
    HttpResponse::Ok().body("List of students")
}

// This route handler will list data with specific id
#[get("/students/{id}")]
async fn find() -> HttpResponse {
    HttpResponse::Ok().body("Listing student with specific id")
}

// This route handler will create a new record
#[post("/students")]
async fn create() -> HttpResponse {
    HttpResponse::Ok().body("Creating a new record......")
}

// This route handler will update an existing record
#[put("/students/{id}")]
async fn update() -> HttpResponse {
    HttpResponse::Ok().body("Updating record......")
}

// This route handler will delete an specified record
#[delete("/students/{id}")]
async fn delete() -> HttpResponse {
    HttpResponse::Ok().body("Deleting record......")
}

// ---------------------- End Route-Handlers ----------------------