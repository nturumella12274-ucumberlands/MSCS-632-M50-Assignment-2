function greet(name) {
    let hello = "Hello, ";
    return function() {
        return hello + name;
    };
}

let greeting = greet("Nihar");
console.log(greeting());   

let x = 10;
console.log(typeof x);   
x = "ten";
console.log(typeof x); 
