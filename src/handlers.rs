use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::{Student, CreateStudent}; // Ensure models are correctly imported

#[actix_web::post("/students")]
pub async fn create_student_handler(
    student_data: web::Json<CreateStudent>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let new_student = student_data.into_inner();

    // Validate the incoming student data
    if let Err(validation_errors) = new_student.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({ "errors": validation_errors }));
    }

    // Log the received student data (optional, for debugging)
    // println!("Received student data: {:?}", new_student);

    match sqlx::query_as!(
        Student,
        r#"
        INSERT INTO students (name, surname, phonenumber, birthday, email)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, surname, phonenumber, birthday, email
        "#,
        new_student.name,
        new_student.surname,
        new_student.phonenumber,
        new_student.birthday,
        new_student.email
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(created_student) => {
            // Log the created student (optional)
            // println!("Created student: {:?}", created_student);
            HttpResponse::Created().json(created_student)
        }
        Err(e) => {
            // Log the error (important for debugging)
            eprintln!("Failed to create student: {:?}", e);
            match e {
                sqlx::Error::Database(db_err) => {
                    // Check for specific database errors if needed
                    // For example, unique constraint violation
                    if db_err.constraint().is_some() {
                        HttpResponse::Conflict().json(serde_json::json!({
                            "error": "Student already exists or constraint violation.",
                            "details": db_err.message().to_string()
                        }))
                    } else {
                        HttpResponse::InternalServerError().json(serde_json::json!({
                            "error": "Database error while creating student.",
                            "details": db_err.message().to_string()
                        }))
                    }
                }
                _ => HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to create student due to an unexpected error.",
                    "details": e.to_string()
                })),
            }
        }
    }
}
