from webapp.config.logs import logger


def file_to_string(path: str, encoding: str = "utf-8") -> str:
    """Store the contents of file into memory as a string variable.

    Args:
        path (str): Relative or absolute file path of file to open.
        encoding (str): The encoding method used to read file. Defaults to "utf-8"

    Returns:
        str: Contents of file.

    Raises:
        FileNotFoundError: Raises if the path specified does not exist.
        PermissionError: Raises if the user running the script does not have adequate
            permissions to open the file.
        IsADirectoryError: Raises if the path is a directory rather than a file.
        UnicodeDecodeError: Raises if there is an issue when parsing the file with
            the specified encoding.

    """
    try:
        with open(path, encoding=encoding) as file:
            file_str: str = file.read()
            return file_str
    except FileNotFoundError:
        logger.exception("File path %s does not exist.", path)
        raise
    except (PermissionError, IsADirectoryError, UnicodeDecodeError):
        logger.exception("Error reading file at path %s.", path)
        raise
