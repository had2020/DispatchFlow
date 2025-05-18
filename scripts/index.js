// This is single-line comment.
/*
This is a multi-line comment.
*/

console.log(
  "This is message, viewable by inspecting the website, and clicking the console tab in your browser.",
);

let is_user = localStorage.getItem("is_user"); // accessing the property on a key("is_user") for a boolean.

setTimeout(redirect_function, 300); // This is a added delay, which likly be needed in the future, when loading data.

function redirect_function() {
  if (is_user == true) {
    window.location.href = "user_pages/map.html";
  } else {
    window.location.href = "guest_pages/home.html";
  }
}
