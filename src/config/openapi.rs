use utoipa::{
    Modify,
    openapi::{
        Components,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if openapi.components.is_none() {
            openapi.components = Some(Components::new());
        }
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "token_jwt",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

#[derive(utoipa::OpenApi)]
#[openapi(
        info(
            title = "mrcn.tr Backend Swagger API Docs",
            version="1.0"
        ),
        tags(

        ),
        modifiers(&SecurityAddon)
    )]
pub struct ApiDoc;
