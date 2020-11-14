from commands import close, clean, print
from utils import error
from colorama import Fore


class Main:
    def __init__(self, prompt="-| "):
        self.prompt = prompt + Fore.BLUE
        self.cli = input(self.prompt)
        self.error = error.Error()

    def shell(self):
        commands = {
            1: "Clean",
            2: "Print",
            3: "Close",
        }
        print(commands.get(1))


if __name__ == "__main__":
    main = Main()
    main.shell()
