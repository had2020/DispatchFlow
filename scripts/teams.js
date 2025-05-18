if (localStorage.getItem("is_user") == false) {
  window.location.href = "../guest_pages/home.html";
}

if (localStorage.getItem("In_team") == "true") {
  window.location.href = "../user_pages/in_teams.html";
}

document.getElementById("Join_button").addEventListener("click", function () {
  const username = localStorage.getItem("User");
  const password = localStorage.getItem("Pass");

  let teams_code =
    document.getElementById("1").value +
    document.getElementById("2").value +
    document.getElementById("3").value +
    document.getElementById("4").value +
    document.getElementById("5").value +
    document.getElementById("6").value;

  if (!localStorage.getItem("is_user")) {
    window.location.href = "../guest_pages/home.html";
  } else {
    fetch("http://76.167.195.153:8000/join_team", {
      referrerPolicy: "unsafe-url",
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        user: username,
        pass: password,
        action: "Create",
        code: teams_code,
        team_name: "",
      }),
    })
      .then((response) => response.text())
      .then((data) => {
        status_data = data;

        if (status_data !== null && status_data !== "") {
          alert("👋 Welcome to the team!");
          localStorage.setItem("In_team", "true");
          localStorage.setItem("Team_name", status_data);
          localStorage.setItem("Team_code", teams_code);

          window.location.href = "../user_pages/in_teams.html";
        } else {
          alert("Incorrect Team Code!");
        }

        console.log(status_data);
      });
  }
});

document.getElementById("Create_button").addEventListener("click", function () {
  const username = localStorage.getItem("User");
  const password = localStorage.getItem("Pass");

  let team_name = document.getElementById("Team_name_input").value;

  if (!localStorage.getItem("is_user")) {
    window.location.href = "../guest_pages/home.html";
  } else {
    fetch("http://76.167.195.153:8000/create_team", {
      referrerPolicy: "unsafe-url",
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        user: username,
        pass: password,
        action: "Create",
        code: "",
        team_name: team_name,
      }),
    })
      .then((response) => response.text())
      .then((data) => {
        status_data = data;

        if (status_data !== null && status_data !== "Already") {
          localStorage.setItem("In_team", "true");
          localStorage.setItem("Team_name", team_name);
          localStorage.setItem("Team_code", status_data);
          alert("Created Team!");
          window.location.href = "../user_pages/in_teams.html";
        } else {
          if (status_data == "Already") {
            alert("Team name is already taken!");
          } else {
            alert("Error with team creation!");
          }
        }

        console.log(status_data);
      });
  }
});
