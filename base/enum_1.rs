enum HttpResultCode {
    Ok = 200,
    NotFound = 404,
    Teapot = 418,
}

fn main() {
    let code: HttpResultCode = HttpResultCode::NotFound;

    //assert_eq!(code as i32, 404);

    let msg = match code {
        HttpResultCode::Ok => "Ok",
	HttpResultCode::NotFound => "Not Found",
	HttpResultCode::Teapot => "Teapot"
    };
    println!("{}", msg);
}
