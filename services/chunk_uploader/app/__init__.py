import logging

from flask import Flask
from apscheduler.schedulers.background import BackgroundScheduler
from apscheduler.jobstores.memory import MemoryJobStore
from apscheduler.executors.pool import ProcessPoolExecutor
from apscheduler.executors.pool import ThreadPoolExecutor

from app.config import AppConfig
from app.pinata import Pinata
from app.solana import Solana
from app.task import Task


config = AppConfig()
logging.basicConfig(
    level=config.LOG_LEVEL,
    format='%(asctime)s %(name)-12s %(levelname)-8s %(message)s',
    datefmt='%m-%d %H:%M:%S'
)
scheduler = BackgroundScheduler(
    jobstores={
        'default': MemoryJobStore(),
    },
    executors={
        # 'default': ProcessPoolExecutor()
        'default': ThreadPoolExecutor()
    },
    job_defaults={
        'coalesce': False,
        'max_instances': config.SCHEDULER_MAX_INSTANCES
    }
)
app =  Flask(__name__)
pinata = Pinata(config)
solana = Solana(config)
task = Task(config, solana, pinata, scheduler)

__all__ = config, scheduler, app, task
