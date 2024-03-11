import logging
from typing import Optional


# pylint: disable=no-member
def setup_logger(
    name: str, log_file: Optional[str] = None, level: int = logging.INFO
) -> logging.Logger:
    """To setup logger"""
    # 创建日志记录器
    logger = logging.getLogger(name)
    logger.setLevel(level)

    # 创建控制台处理程序, 并设级别为DEBUG
    console_handler = logging.StreamHandler()
    console_handler.setLevel(logging.DEBUG)

    # 创建文件处理程序, 并设置级别为INFO
    if log_file:
        file_handler = logging.FileHandler(log_file, mode="a", encoding="utf-8")
        file_handler.setLevel(logging.INFO)

        # 创建日志格式
        formatter = logging.Formatter(
            "%(asctime)s - %(name)s - %(levelname)s - %(message)s"
        )
        console_handler.setFormatter(formatter)
        file_handler.setFormatter(formatter)

        # 将处理程序添加到日志记录器
        logger.addHandler(file_handler)
    else:
        logger.addHandler(console_handler)

    return logger


def _some_function() -> None:
    """
    test1
    """
    logger = setup_logger("module1_logger")
    logger.debug("Debug message from module1")
    logger.info("Info message from module1")


def _another_function() -> None:
    """
    test2
    """
    logger = setup_logger("module2_logger", log_file="module2.log", level=logging.DEBUG)
    logger.debug("Debug message from module2")
    logger.info("Info message from module2")


def main() -> None:
    """
    test
    """
    _some_function()
    _another_function()


if __name__ == "__main__":
    main()
