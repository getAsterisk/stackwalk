import os

class Something:
    def __init__(self):
        self.value = 1

    def get_value(self):
        return self.value

    def set_value(self, value):
        self.value = value

    def execute_system_command(self, command):
        os.system(command)

def main():
    something = Something()
    print(something.get_value())
    something.set_value(2)
    print(something.get_value())
    something.execute_system_command("echo 'Hello, World!'")

if __name__ == "__main__":
    main()
