use std::sync::Arc;

use tokio::sync::RwLock;
use tokio_cron_scheduler::JobScheduler;

use crate::{cli::Args, config::Config, logger::Logger, service::Service};

pub type Context = Arc<ContextData>;

pub struct ContextData {
    pub cli_opts: Args,
    pub cfg: RwLock<Config>,

    pub logger: RwLock<Logger>,
    pub services: RwLock<Vec<Service>>,

    pub sched: RwLock<JobScheduler>,
}

impl ContextData {
    pub fn new(
        cli_opts: Args,
        cfg: Config,
        logger: Logger,
        services: Vec<Service>,
        sched: JobScheduler,
    ) -> Arc<Self> {
        Arc::new(ContextData {
            cli_opts: cli_opts.clone(),
            cfg: RwLock::new(cfg),

            logger: RwLock::new(logger),

            services: RwLock::new(services),

            sched: RwLock::new(sched),
        })
    }
}
