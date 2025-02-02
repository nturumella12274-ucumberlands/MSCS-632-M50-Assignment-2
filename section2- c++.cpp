#include <iostream>
#include <string>
#include <functional>

using namespace std;

function<string()> greet(string name) {
    string hello = "Hello, ";
    return [hello, name]() {
        return hello + name;
    };
}

int main() {
    auto greeting = greet("Sujan");
    cout << greeting() << endl;  // Output: Hello, Sujan

    int x = 10;
    cout << typeid(x).name() << endl;  // Output: i (for int)
    
    // x = "ten";  // This would cause a compile-time error
    return 0;
}
