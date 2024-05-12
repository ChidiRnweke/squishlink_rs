use std::sync::Arc;

use crate::{config::AppState, generator::database::PostgresRepository};
use tokio::time::{sleep, Duration};

pub async fn spawn_cleanup_task(db_config: Arc<AppState>) {
    let one_day = Duration::from_secs(86400);
    loop {
        let Ok(mut repo) = PostgresRepository::from_config(&db_config.app_config.db_config) else {
            log::error!("Failed to obtain a database connection during for the cleanup task.");
            continue;
        };

        let result = repo.cleanup_old_links();
        match result {
            Ok(res) => log::info!("Cleaned up {res} old links."),
            Err(e) => log::error!("The following error occurred while doing db cleanup: {e}"),
        }
        sleep(one_day).await;
    }
}
