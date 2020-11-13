from commands import close


class Main:
    def __init__(self, prompt="-> "):
        self.prompt = input(prompt)

    def commands(self):
        close_command = close.Close()
        command = self.prompt
        if command.startswith("exit"):
            close_command.run()


if __name__ == "__main__":
    main = Main()
    main.commands()
