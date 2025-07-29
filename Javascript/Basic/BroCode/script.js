/* Basics

console.log(`Hello, World!`);
window.alert("ALERT!!!");

document.getElementById("hello").textContent = "Hello, World!";
*/

/* Variables

let a;
a = 100;

let b = 200;

console.log(`a + b = ${a + b}`);
console.log(typeof a);
*/

/* Booleans

let online = true;
let for_sale = false;

console.log(typeof online);

*/

/* With HTML 

let name = "Mahi";

document.getElementById("heading").textContent = `My name is ${name}`;
*/

/* Arithmetic Operators

let students = 30;

students = students + 1;
students += 1;
students ++;

students = students - 1;
students -= 1;
students --;

students = students * 1;
students *= 1;

students = students / 5;
students /= 5;

students = students ** 2;
students **= 2;

students = students % 3;
students %= 3;

console.log(students);

let num = 12 / 2 * 10 % 5 -(7 - 4) ** 3

console.log(num)
*/

/* User Input

// Widnow Prompt

let username;

username = window.prompt("What's your username: ");

console.log(username);

//HTML Textbox

document.getElementById("submit").onclick = function(){
    let username = document.getElementById("input").value;
    document.getElementById("heading").textContent = `Hello ${username}`;
}
*/

/* Type Conversion 

let age = window.prompt("What's your age: ");

age = Number(age);
console.log(age, typeof age);

let x = "pizza";
let y = 0;
let z = "";

x = Number(x);
y = String(y);
z = Boolean(z);

let a;
console.log(a, typeof a)

console.log(x, typeof x);
console.log(y, typeof y);
console.log(z, typeof z);
*/

/* Constants */

// Circumference Calculator

let pi = 3.14159;


