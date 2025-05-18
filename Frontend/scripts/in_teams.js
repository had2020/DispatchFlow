if (localStorage.getItem("is_user") == false) {
  window.location.href = "../guest_pages/home.html";
}

if (localStorage.getItem("In_team") != "true") {
  window.location.href = "../user_pages/teams.html";
}

console.log(localStorage.getItem("In_team"));

const team_code = localStorage.getItem("Team_code");

document.getElementById("code").innerText = team_code;

document.getElementById("leave_button").addEventListener("click", function () {
  localStorage.setItem("In_team", false);
  localStorage.setItem("Team_name", "");
  localStorage.setItem("Team_code", "");

  window.location.href = "../user_pages/teams.html";
});
