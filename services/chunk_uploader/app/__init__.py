from apscheduler.schedulers.background import BackgroundScheduler
from apscheduler.jobstores.memory import MemoryJobStore
from apscheduler.executors.pool import ProcessPoolExecutor

from app.config import AppConfig

config: AppConfig = AppConfig()
scheduler = BackgroundScheduler(
    jobstores={
        'default': MemoryJobStore(),
    },
    executors={
        'default': ProcessPoolExecutor()
    },
    job_defaults={
        'coalesce': False,
        'max_instances': config.SCHEDULER_MAX_INSTANCES
    }
)

__all__ = config, scheduler,
