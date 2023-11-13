use sqlx::PgPool;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    common::{
        channels::{ClientMessage, ClientSender, Method, UploadFileData},
        jwt_manager::AuthUser,
    },
    errors::{PentaractError, PentaractResult},
    models::files::{FSElement, InFile},
    repositories::files::FilesRepository,
    schemas::files::InFileSchema,
};

pub struct FilesService<'d> {
    repo: FilesRepository<'d>,
    tx: ClientSender,
}

impl<'d> FilesService<'d> {
    pub fn new(db: &'d PgPool, tx: ClientSender) -> Self {
        let repo = FilesRepository::new(db);
        Self { repo, tx }
    }

    pub async fn upload(&self, in_schema: InFileSchema, user: &AuthUser) -> PentaractResult<()> {
        // 0. path validation
        if Self::validate_filepath(&in_schema.path) {
            return Err(PentaractError::InvalidPath);
        }

        // 1. saving file in db
        let in_file = InFile::new(in_schema.path, in_schema.storage_id);
        let file = self.repo.create_file(in_file).await?;

        // 2. sending file to storage manager
        let (resp_tx, resp_rx) = oneshot::channel();

        let message = {
            let upload_file_data = UploadFileData {
                file_id: file.id,
                user_id: user.id,
                file_data: in_schema.file.as_ref().into(),
            };
            ClientMessage {
                method: Method::UploadFile(upload_file_data),
                tx: resp_tx,
            }
        };

        tracing::debug!("sending task to manager");
        let _ = self.tx.send(message).await;

        // 3. waiting for a storage manager result
        if let Err(e) = resp_rx.await.unwrap().and({
            tracing::debug!("file loaded successfully");

            // 4. setting file as uploaded
            self.repo.set_as_uploaded(file.id).await
        }) {
            tracing::error!("{e}");

            // fallback logic: deleting file
            let _ = self.repo.delete(file.id).await;

            return Err(e);
        };

        Ok(())
    }

    pub async fn list_dir(self, storage_id: Uuid, path: &str) -> PentaractResult<Vec<FSElement>> {
        self.repo.list_dir(storage_id, path).await
    }

    /////////////////////////////////////////////////////////////////////
    ////    Helpers
    /////////////////////////////////////////////////////////////////////

    fn validate_filepath(path: &str) -> bool {
        Self::validate_path(path) && !path.ends_with(r"/")
    }

    fn validate_path(path: &str) -> bool {
        !path.starts_with(r"/") && !path.contains(r"//")
    }
}