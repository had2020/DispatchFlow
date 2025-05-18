document.getElementById("Submit").addEventListener("click", function () {
  const username = document.getElementById("User").value;
  const password = document.getElementById("Pass").value;

  let status_data = "";

  if (localStorage.getItem("is_user")) {
    window.location.href = "../user_pages/map.html";
  } else {
    fetch("http://76.167.195.153:8000/send_login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ user: username, pass: password }),
    })
      .then((response) => response.text())
      .then((data) => {
        status_data = data;

        if (status_data == "is_user") {
          localStorage.setItem("is_user", true);
          localStorage.setItem("User", username);
          localStorage.setItem("Pass", password);
          window.location.href = "../user_pages/map.html";
        }
        if (status_data == "not_user") {
          alert("Incorrect Username or Password");
        }

        console.log(status_data);
      });
  }
});
