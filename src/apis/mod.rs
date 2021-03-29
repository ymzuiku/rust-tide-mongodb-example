use crate::AppState;

mod restful_example;
// mod pg_user;
mod mongo_example;
mod postgres_example;

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
    app.at("/mongo_example/update").post(mongo_example::update);
    app.at("/mongo_example/find").post(mongo_example::find);
    app.at("/mongo_example/find_by_document")
        .post(mongo_example::find_by_document);

    app.at("/pg_example/create_table")
        .post(postgres_example::create_table);
    app.at("/pg_example/table_info")
        .post(postgres_example::table_info);
    app.at("/pg_example/add").post(postgres_example::add);
    app.at("/pg_example/find").post(postgres_example::find);
    app.at("/pg_example/update").post(postgres_example::update);
    app.at("/pg_example/del").post(postgres_example::del);
}
