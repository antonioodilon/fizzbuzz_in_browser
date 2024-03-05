use actix_web::{web, App, HttpServer, Responder, HttpRequest};

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    println!("Server is firing up!");
    println!("Welcome to Fizz Buzz! Please change the address inside your browser by providing a start and an end value. Use integer numbers.");
    println!("\tHere is an example: http://127.0.0.1:8080/fizz_buzz/from=3%to=25");

    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(hello))
        .route("/{name}", web::get().to(hello))
        .route("/fizz_buzz/from={start}%to={end}", web::get().to(fizzbuzz))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    //println!("Hello, world!");
}

async fn hello(request: HttpRequest) -> impl Responder
{
    let world = request.match_info().get("name").unwrap_or("World");

    format!("Hello {}", world)
}

async fn fizzbuzz(request: HttpRequest) -> impl Responder
{
    let fizz:String = String::from("fizz!");
    let buzz:String = String::from("buzz!");
    let fizzbuzz:String = String::from("fizzbuzz!");

    let mut result:String = String::from("");
    let new_line:String = String::from("\n");
    

    let start = request.match_info().get("start").unwrap_or("World");
    let end = request.match_info().get("end").unwrap_or("World");

    let start_result:Result<i32, _> = start.parse::<i32>();
    let mut start_int:i32;
    let end_result:Result<i32, _> = end.parse::<i32>();
    let end_int:i32;

    match start_result
    {
        Ok(int_success) => start_int = int_success,
        Err(_) => 
        {
            return format!("The start value must be an integer number, not a float, a letter or some other character. Example: http://127.0.0.1:8080/fizz_buzz/from=2%to=20");
        },
    }

    match end_result
    {
        Ok(int_success) => end_int = int_success,
        Err(_) =>
        {
            return format!("The end value must be an integer number, not a float, a letter or some other character. Example: http://127.0.0.1:8080/fizz_buzz/from=-10%to=22");
        },
    }

    if start_int > end_int
    {
        return format!("Start number cannot be greater than end number.");
    }

    while start_int <= end_int
    {
        if start_int == 0
        {
            result.push_str(&start_int.to_string());
            result.push_str(&new_line);
        }
        else if start_int % 15 == 0
        {
            result.push_str(&fizzbuzz);
            result.push_str(&new_line);
        }
        else if start_int % 3 == 0
        {
            result.push_str(&fizz);
            result.push_str(&new_line);
            //return fizz;
        } else if start_int % 5 == 0
        {
            result.push_str(&buzz);
            result.push_str(&new_line);
        } else
        {
            result.push_str(&start_int.to_string());
            result.push_str(&new_line);
        }

        start_int+=1;
    }

    result // result is a string that, when displayed on the browser, will show all the numbers in the range the user
    // provided, with each number being separated by a new line
}