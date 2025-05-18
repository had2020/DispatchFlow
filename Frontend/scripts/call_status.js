if (localStorage.getItem("is_user") == false) {
  window.location.href = "../guest_pages/home.html";
}

if (localStorage.getItem("In_team") != "true") {
  window.location.href = "../user_pages/teams.html";
}
