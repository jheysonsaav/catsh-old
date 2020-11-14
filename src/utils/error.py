from colorama import init, Fore


class Error:
    def __init__(self, msg="Undefined error message"):
        self.msg = msg

    def run(self):
        init()
        print("Error: {0}".format(self.msg))
