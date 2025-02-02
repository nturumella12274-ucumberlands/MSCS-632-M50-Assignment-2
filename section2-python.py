def greet(name):
    hello = "Hello, "
    def say_hello():
        return hello + name
    return say_hello

greeting = greet("Nihar")
print(greeting())  

x = 10
print(type(x))  # Output: <class 'int'>
x = "ten"
print(type(x))  # Output: <class 'str'>
