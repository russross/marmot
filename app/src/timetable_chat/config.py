from functools import lru_cache
from pathlib import Path

from pydantic import SecretStr
from pydantic_settings import BaseSettings, SettingsConfigDict

APP_ROOT = Path(__file__).resolve().parents[2]


class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_file=(Path.home() / ".keys", APP_ROOT / ".env"), extra="ignore"
    )

    openrouter_api_key: SecretStr = SecretStr("")
    openrouter_model: str = "stealth/ox-alpha"
    marmot_data_file: Path = APP_ROOT / "data" / "spring-2027.json"
    marmot_runtime_dir: Path = APP_ROOT / "runtime"
    current_assignments_url: str = (
        "https://dixiestate-my.sharepoint.com/:x:/g/personal/"
        "d00003177_utahtech_edu/IQBwn5uigXgnSbURqT5f7uSXATuBEraNsUaHm5xpQ8uTJPQ"
        "?rtime=KvRjnMcC30g&download=1"
    )
    openrouter_base_url: str = "https://openrouter.ai/api/v1"
    max_tool_rounds: int = 8


@lru_cache(maxsize=1)
def get_settings() -> Settings:
    return Settings()
