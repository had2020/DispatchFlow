if (!localStorage.getItem("is_user")) {
  window.location.href = "../guest_pages/home.html";
}

function Erase_User_Data() {
  fetch("https://76.167.195.153:8000/send_clear", {
    referrerPolicy: "unsafe-url",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      user: localStorage.getItem("User"),
      pass: localStorage.getItem("Pass"),
    }),
  })
    .then((response) => response.text())
    .then((data) => {
      status_data = data;

      if (status_data == "cleared") {
        localStorage.setItem("is_user", false);
        localStorage.setItem("User", "");
        localStorage.setItem("Pass", "");
        localStorage.setItem("In_team", false);
        localStorage.setItem("Team_name", "");
        localStorage.setItem("Team_code", "");
        localStorage.clear();
        localStorage.setItem("In_team", false);
        window.location.href = "../guest_pages/home.html";
      }
      if (status_data == "failed") {
        alert("Failure with server response!");
      }

      console.log(status_data);
    });
}

const button = document.getElementById("Erase");
button.addEventListener("click", Erase_User_Data);

//Logout User
function Logout_User() {
  localStorage.setItem("is_user", false);
  localStorage.setItem("User", "");
  localStorage.setItem("Pass", "");

  window.location.href = "../guest_pages/home.html";
}

const button1 = document.getElementById("Logout");
button1.addEventListener("click", Logout_User);

//Privacy Policy
//TODO
