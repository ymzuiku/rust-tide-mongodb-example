use crate::AppState;

mod restful_example;
// mod pg_user;
mod mongo_example;

pub fn register_apis(app: &mut tide::Server<AppState>) {
    // examples
    app.at("/restful_example/post_error")
        .post(restful_example::post_error);
    app.at("/restful_example/post_get_error")
        .post(restful_example::post_get_error);
    app.at("/restful_example/post_unwrap")
        .post(restful_example::post_unwrap);
    app.at("/restful_example/post_panic")
        .post(restful_example::post_panic);

    app.at("/restful_example/get_query")
        .get(restful_example::get_query);
    app.at("/restful_example/get_params/:name/:age")
        .get(restful_example::get_params);
    app.at("/restful_example/post_body")
        .post(restful_example::post_body);
    app.at("/restful_example/post_body_doc")
        .post(restful_example::post_body_doc);

    app.at("/mongo_example/info").post(mongo_example::info);
    app.at("/mongo_example/add").post(mongo_example::add);
    app.at("/mongo_example/del").post(mongo_example::del);
    app.at("/mongo_example/find").post(mongo_example::find);
}