use crate::common::{ReleaseAsset, TempHomeDir};
use httptest::http::response;
use httptest::{matchers::request, responders::status_code, Expectation, Server};

pub struct ReleaseServer {
    server: Server,
}

impl ReleaseServer {
    pub fn new(home_dir: &TempHomeDir) -> Self {
        let server = Server::run();
        let download_url_template = server.url_str(
            "/any/arbitrary/path/{{version}}/dfx-{{version}}-{{arch}}-{{platform}}.tar.gz",
        );
        home_dir
            .settings()
            .write_download_url_template(&download_url_template);
        let manifest_url = server.url_str("/manifest.json");
        home_dir.settings().write_manifest_url(&manifest_url);
        Self { server }
    }

    pub fn expect_get(&self, asset: &ReleaseAsset) {
        self.server.expect(
            Expectation::matching(request::method_path("GET", url_path(asset)))
                .respond_with(asset.ok_response()),
        );
    }

    pub fn expect_get_respond_not_found(&self, asset: &ReleaseAsset) {
        self.server.expect(
            Expectation::matching(request::method_path("GET", url_path(asset)))
                .respond_with(status_code(404)),
        );
    }

    pub fn expect_get_manifest(&self, contents: &str) {
        self.server.expect(
            Expectation::matching(request::method_path("GET", "/manifest.json")).respond_with(
                response::Builder::new()
                    .status(200)
                    .body(contents.as_bytes().to_vec())
                    .unwrap(),
            ),
        );
    }
}

fn url_path(asset: &ReleaseAsset) -> String {
    let version = &asset.version;
    let filename = &asset.filename;
    format!("/any/arbitrary/path/{version}/{filename}")
}
