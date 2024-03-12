import sys


def color_print(content: str, typ="normal", asc_control="0"):
    """
    彩色输出, 黑色背景无格式;
    三种模式: 常规(绿), 警告(黄), 错误(红)

    :param content: 要输出的内容，应为字符串。
    :param typ: 输出内容的类型，可选值为"normal"、"warning"、"error"，默认为"normal"。
    :param asc_control: ASCII控制码，用于重置终端颜色等，应为符合ANSI转义序列格式的字符串，默认为"0"。

    :raises ValueError: 当传入的`typ`不是受支持的类型时抛出。
    """

    # 定义颜色代码字典
    COLOR_CODE = {
        "normal": "\033[32m",  # 绿色
        "warning": "\033[33m",  # 黄色
        "error": "\033[31m",  # 红色
    }

    # 验证`typ`参数是否有效
    if typ not in COLOR_CODE:
        raise ValueError(
            f"Unsupported type: {typ}. Must be one of 'normal', 'warning', 'error'."
        )

    # 验证`asc_control`参数是否为字符串
    if not isinstance(asc_control, str):
        raise TypeError("asc_control must be a string.")

    # 使用颜色代码字典简化逻辑
    color_code = COLOR_CODE.get(typ, COLOR_CODE["normal"])  # 默认为绿色

    try:
        print(f"{color_code}{content}\033[{asc_control}m")
    except IOError as e:
        # 异常处理，可以选择记录日志或者抛出更具体的异常
        print(f"Failed to print colored text: {e}")
        sys.exit(0)


def my_input(content, empty="no"):
    """
    交互式获取输入文件
    :param content: 提示信息
    :param empty: 当输入为空时的处理方式 ("no"为要求重新输入，其他值为直接返回空字符串)
    :return: 用户输入的字符串
    """
    result = ""
    while True:
        try:
            result = input(f"\033[32m>>>{content}\033[0m\n")
            result = result.rstrip()

            if not result and empty == "no":
                color_print("请重新输入", typ="warning")
            else:
                if result == "exit":
                    color_print("程序退出")
                    sys.exit(0)
                else:
                    return result
        except SystemExit:
            color_print("程序退出")
            sys.exit(0)
        except KeyboardInterrupt:
            color_print("操作中断", typ="warning")
            sys.exit(0)
        except IOError as e:
            color_print(f"输入过程中发生错误: {str(e)}", typ="error")
            # 为了防止无限循环，此处可以选择退出或重新抛出异常
            sys.exit(1)
