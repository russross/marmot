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
    openrouter_model: str = "deepseek/deepseek-v4-flash-0731"
    marmot_data_file: Path = APP_ROOT / "data" / "fall-2026.json"
    marmot_runtime_dir: Path = APP_ROOT / "runtime"
    openrouter_base_url: str = "https://openrouter.ai/api/v1"
    max_tool_rounds: int = 8


@lru_cache(maxsize=1)
def get_settings() -> Settings:
    return Settings()
