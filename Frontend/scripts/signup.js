document.getElementById("Submit").addEventListener("click", function () {
  const username = document.getElementById("User").value;
  const password = document.getElementById("Pass").value;

  let status_data = "";

  if (localStorage.getItem("is_user")) {
    window.location.href = "../user_pages/map.html";
  } else {
    fetch("http://76.167.195.153:8000/send_signup", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ user: username, pass: password }),
    })
      .then((response) => response.text())
      .then((data) => {
        status_data = data;

        if (status_data == "already_user") {
          alert("Sorry, that user name is already taken.");
        }
        if (status_data == "created_user") {
          localStorage.setItem("is_user", true);
          localStorage.setItem("User", username);
          localStorage.setItem("Pass", password);
          window.location.href = "../user_pages/map.html";
        }

        console.log(status_data);
      });
  }
});
