from colorama import *


class Error:
    def __init__(self, err):
        self.err = err
        self.text_err = "Error: " + Fore.RED

    def nose(self):
        init(autoreset=True)
        print(f"{self.text_err}{self.err}")
