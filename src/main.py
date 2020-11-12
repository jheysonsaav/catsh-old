import os
from src.utils.error import Error


class Cli:
    def starting(self):
        command = input("➜ ")
        if command.startswith("exit"):
            command = command.split(" ", 2)
            exit_code = command[1] or 0
            exit(exit_code)
        elif command.startswith("print"):
            command = command.split(" ", 1)
            print(command[1])
        elif command.startswith("exec"):
            command = command.split(" ", 3)
            os.system(f"{command[1]} {command[2]}")


if __name__ == "__main__":
    Cli.starting("")
